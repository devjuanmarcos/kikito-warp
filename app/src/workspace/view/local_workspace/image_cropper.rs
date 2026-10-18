#[derive(Clone, Copy, Debug, PartialEq)]
pub(super) struct CropViewport {
    pub width: f32,
    pub height: f32,
    pub zoom: f32,
    pub offset_x: f32,
    pub offset_y: f32,
}

impl CropViewport {
    pub fn zoomed(self, zoom: f32) -> Self {
        Self {
            zoom: zoom.clamp(1., 4.),
            offset_x: self.offset_x.clamp(-1., 1.),
            offset_y: self.offset_y.clamp(-1., 1.),
            ..self
        }
    }

    pub fn panned(self, delta_x: f32, delta_y: f32, image_width: f32, image_height: f32) -> Self {
        if image_width <= 0. || image_height <= 0. {
            return Self {
                offset_x: 0.,
                offset_y: 0.,
                ..self
            };
        }
        let crop_side = image_width.min(image_height) / self.zoom;
        let available_pan_x = (image_width - crop_side).max(0.) * self.width / crop_side;
        let available_pan_y = (image_height - crop_side).max(0.) * self.height / crop_side;
        Self {
            offset_x: pan_offset(self.offset_x, delta_x, available_pan_x),
            offset_y: pan_offset(self.offset_y, delta_y, available_pan_y),
            ..self
        }
    }
}

fn pan_offset(current: f32, delta: f32, available_pan: f32) -> f32 {
    if available_pan <= 0. {
        return 0.;
    }
    (current - 2. * delta / available_pan).clamp(-1., 1.)
}
use std::io::Cursor;

use image::{
    DynamicImage, GenericImageView, ImageFormat, ImageReader, Limits, imageops::FilterType,
};
use resvg::tiny_skia::{Pixmap, Transform};
use resvg::usvg::{Options, Tree};

pub(crate) const MAX_UPLOAD_BYTES: usize = 5 * 1024 * 1024;
const MAX_SOURCE_DIMENSION: u32 = 4096;
const MAX_SOURCE_PIXELS: u64 = 4_000_000;
const MAX_DECODE_ALLOCATION: u64 = 40 * 1024 * 1024;
const OUTPUT_SIZE: u32 = 128;
const MAX_CROPPED_BYTES: usize = 128 * 1024;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum ImageCropError {
    TooLarge,
    UnsupportedFormat,
    InvalidImage,
    DimensionsTooLarge,
}

pub(crate) struct CroppedImage {
    pub png: Vec<u8>,
    pub source_width: u32,
    pub source_height: u32,
}

pub(crate) fn crop_image_to_png(bytes: &[u8]) -> Result<Vec<u8>, ImageCropError> {
    crop_image_to_png_with_zoom(bytes, 1.)
}

pub(crate) fn crop_image_to_png_with_zoom(
    bytes: &[u8],
    zoom: f32,
) -> Result<Vec<u8>, ImageCropError> {
    crop_image_to_png_with_view(bytes, zoom, 0., 0.)
}

pub(crate) fn crop_image_to_png_with_view(
    bytes: &[u8],
    zoom: f32,
    pan_x: f32,
    pan_y: f32,
) -> Result<Vec<u8>, ImageCropError> {
    crop_image_to_png_with_view_and_dimensions(bytes, zoom, pan_x, pan_y).map(|crop| crop.png)
}

pub(crate) fn crop_image_to_png_with_view_and_dimensions(
    bytes: &[u8],
    zoom: f32,
    pan_x: f32,
    pan_y: f32,
) -> Result<CroppedImage, ImageCropError> {
    if bytes.is_empty() || bytes.len() > MAX_UPLOAD_BYTES {
        return Err(ImageCropError::TooLarge);
    }
    let zoom = zoom.clamp(1., 4.);
    if bytes
        .iter()
        .find(|byte| !byte.is_ascii_whitespace())
        .copied()
        == Some(b'<')
    {
        return crop_svg_to_png(bytes, zoom, pan_x, pan_y);
    }
    let format = image::guess_format(bytes).map_err(|_| ImageCropError::UnsupportedFormat)?;
    if !matches!(format, ImageFormat::Png | ImageFormat::Jpeg) {
        return Err(ImageCropError::UnsupportedFormat);
    }
    let mut limits = Limits::default();
    limits.max_image_width = Some(MAX_SOURCE_DIMENSION);
    limits.max_image_height = Some(MAX_SOURCE_DIMENSION);
    limits.max_alloc = Some(MAX_DECODE_ALLOCATION);
    let mut reader = ImageReader::with_format(Cursor::new(bytes), format);
    reader.limits(limits);
    let image = reader.decode().map_err(|_| ImageCropError::InvalidImage)?;
    let (width, height) = image.dimensions();
    validate_source_dimensions(width, height)?;
    let png = encode_center_crop(image, zoom, pan_x, pan_y)?;
    Ok(CroppedImage {
        png,
        source_width: width,
        source_height: height,
    })
}

fn validate_source_dimensions(width: u32, height: u32) -> Result<(), ImageCropError> {
    if width == 0
        || height == 0
        || width > MAX_SOURCE_DIMENSION
        || height > MAX_SOURCE_DIMENSION
        || u64::from(width) * u64::from(height) > MAX_SOURCE_PIXELS
    {
        return Err(ImageCropError::DimensionsTooLarge);
    }
    Ok(())
}

fn crop_svg_to_png(
    bytes: &[u8],
    zoom: f32,
    pan_x: f32,
    pan_y: f32,
) -> Result<CroppedImage, ImageCropError> {
    let options = Options::default();
    let tree = Tree::from_data(bytes, &options).map_err(|_| ImageCropError::InvalidImage)?;
    let size = tree.size();
    if size.width() <= 0.
        || size.height() <= 0.
        || size.width() > MAX_SOURCE_DIMENSION as f32
        || size.height() > MAX_SOURCE_DIMENSION as f32
    {
        return Err(ImageCropError::DimensionsTooLarge);
    }
    let scale =
        (512. / size.width().min(size.height())).min(4096. / size.width().max(size.height()));
    let width = (size.width() * scale).round().clamp(1., 4096.) as u32;
    let height = (size.height() * scale).round().clamp(1., 4096.) as u32;
    if u64::from(width) * u64::from(height) > MAX_SOURCE_PIXELS
        || u64::from(width) * u64::from(height) * 4 > MAX_DECODE_ALLOCATION
    {
        return Err(ImageCropError::DimensionsTooLarge);
    }
    let mut pixmap = Pixmap::new(width, height).ok_or(ImageCropError::InvalidImage)?;
    let transform = Transform::from_scale(scale, scale);
    resvg::render(&tree, transform, &mut pixmap.as_mut());
    let image = image::RgbaImage::from_raw(width, height, pixmap.take())
        .ok_or(ImageCropError::InvalidImage)?;
    validate_source_dimensions(width, height)?;
    let png = encode_center_crop(DynamicImage::ImageRgba8(image), zoom, pan_x, pan_y)?;
    Ok(CroppedImage {
        png,
        source_width: width,
        source_height: height,
    })
}

fn encode_center_crop(
    image: DynamicImage,
    zoom: f32,
    pan_x: f32,
    pan_y: f32,
) -> Result<Vec<u8>, ImageCropError> {
    let (width, height) = image.dimensions();
    let side = ((width.min(height) as f32 / zoom.clamp(1., 4.)).round() as u32).max(1);
    let available_x = width - side;
    let available_y = height - side;
    let x =
        ((available_x as f32 / 2.) + pan_x.clamp(-1., 1.) * available_x as f32 / 2.).round() as u32;
    let y =
        ((available_y as f32 / 2.) + pan_y.clamp(-1., 1.) * available_y as f32 / 2.).round() as u32;
    let cropped = image.crop_imm(x, y, side, side);
    let output = cropped.resize_exact(OUTPUT_SIZE, OUTPUT_SIZE, FilterType::Lanczos3);
    let mut bytes = Cursor::new(Vec::new());
    output
        .write_to(&mut bytes, ImageFormat::Png)
        .map_err(|_| ImageCropError::InvalidImage)?;
    let bytes = bytes.into_inner();
    if bytes.len() > MAX_CROPPED_BYTES {
        return Err(ImageCropError::TooLarge);
    }
    Ok(bytes)
}

#[cfg(test)]
#[path = "image_cropper_tests.rs"]
mod tests;
