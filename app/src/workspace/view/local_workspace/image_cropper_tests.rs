use image::{DynamicImage, GenericImageView, ImageFormat, RgbImage};

use super::CropViewport;
use super::{
    ImageCropError, crop_image_to_png, crop_image_to_png_with_view, crop_image_to_png_with_zoom,
};

#[test]
fn crop_image_creates_square_128_pixel_png() {
    let source = DynamicImage::ImageRgb8(RgbImage::new(200, 100));
    let mut encoded = std::io::Cursor::new(Vec::new());
    source.write_to(&mut encoded, ImageFormat::Png).unwrap();
    let cropped = crop_image_to_png(encoded.get_ref()).unwrap();
    let decoded = image::load_from_memory(&cropped).unwrap();
    assert_eq!(decoded.dimensions(), (128, 128));
}

#[test]
fn crop_image_rejects_oversized_and_unsupported_data() {
    assert_eq!(crop_image_to_png(&[]), Err(ImageCropError::TooLarge));
    assert_eq!(
        crop_image_to_png(b"not an image"),
        Err(ImageCropError::UnsupportedFormat)
    );
    assert_eq!(
        crop_image_to_png(&vec![0; 5 * 1024 * 1024 + 1]),
        Err(ImageCropError::TooLarge)
    );
}

#[test]
fn crop_svg_creates_square_128_pixel_png() {
    let svg = br##"<svg xmlns="http://www.w3.org/2000/svg" width="256" height="128"><rect width="256" height="128" fill="#336699"/></svg>"##;
    let cropped = crop_image_to_png(svg).unwrap();
    let decoded = image::load_from_memory(&cropped).unwrap();
    assert_eq!(decoded.dimensions(), (128, 128));
}

#[test]
fn crop_viewport_keeps_zoom_and_pan_within_bounds() {
    let viewport = CropViewport {
        width: 100.,
        height: 100.,
        zoom: 1.,
        offset_x: 100.,
        offset_y: -1000.,
    }
    .zoomed(8.);
    assert_eq!(viewport.zoom, 4.);
    assert_eq!(viewport.offset_x, 1.);
    assert_eq!(viewport.offset_y, -1.);
}

#[test]
fn crop_viewport_pan_is_bounded_and_changes_the_crop() {
    let viewport = CropViewport {
        width: 100.,
        height: 100.,
        zoom: 2.,
        offset_x: 0.,
        offset_y: 0.,
    }
    .panned(500., -500., 100., 100.);
    assert_eq!(viewport.offset_x, -1.);
    assert_eq!(viewport.offset_y, 1.);

    let source = DynamicImage::ImageRgb8(RgbImage::from_fn(256, 256, |x, y| {
        image::Rgb([x as u8, y as u8, 0])
    }));
    let mut encoded = std::io::Cursor::new(Vec::new());
    source.write_to(&mut encoded, ImageFormat::Png).unwrap();
    let center = crop_image_to_png_with_view(encoded.get_ref(), 2., 0., 0.).unwrap();
    let corner = crop_image_to_png_with_view(encoded.get_ref(), 2., 1., -1.).unwrap();
    assert_ne!(center, corner);
}

#[test]
fn crop_viewport_uses_source_aspect_ratio_at_base_zoom() {
    let wide = CropViewport {
        width: 128.,
        height: 128.,
        zoom: 1.,
        offset_x: 0.,
        offset_y: 0.,
    }
    .panned(64., 64., 256., 128.);
    assert_eq!(wide.offset_x, -1.);
    assert_eq!(wide.offset_y, 0.);

    let tall = CropViewport {
        width: 128.,
        height: 128.,
        zoom: 1.,
        offset_x: 0.,
        offset_y: 0.,
    }
    .panned(64., 64., 128., 256.);
    assert_eq!(tall.offset_x, 0.);
    assert_eq!(tall.offset_y, -1.);

    let square = CropViewport {
        width: 128.,
        height: 128.,
        zoom: 1.,
        offset_x: 0.,
        offset_y: 0.,
    }
    .panned(64., 64., 128., 128.);
    assert_eq!(square.offset_x, 0.);
    assert_eq!(square.offset_y, 0.);
}

#[test]
fn dragging_right_moves_image_content_right_in_crop_preview() {
    let source = DynamicImage::ImageRgb8(RgbImage::from_fn(256, 128, |x, _| {
        image::Rgb([x as u8, 0, 0])
    }));
    let mut encoded = std::io::Cursor::new(Vec::new());
    source.write_to(&mut encoded, ImageFormat::Png).unwrap();

    let center = crop_image_to_png_with_view(encoded.get_ref(), 1., 0., 0.).unwrap();
    let dragged_right = crop_image_to_png_with_view(encoded.get_ref(), 1., -1., 0.).unwrap();
    let center_pixel = image::load_from_memory(&center).unwrap().get_pixel(64, 64)[0];
    let dragged_pixel = image::load_from_memory(&dragged_right)
        .unwrap()
        .get_pixel(64, 64)[0];
    assert!(dragged_pixel < center_pixel);
}

#[test]
fn image_dimension_validation_rejects_decode_memory_over_budget() {
    assert_eq!(
        super::validate_source_dimensions(4000, 3000),
        Err(ImageCropError::DimensionsTooLarge)
    );
    assert_eq!(super::validate_source_dimensions(2000, 2000), Ok(()));
    assert_eq!(
        super::validate_source_dimensions(2001, 2000),
        Err(ImageCropError::DimensionsTooLarge)
    );
}

#[test]
fn crop_zoom_rebuilds_the_preview_from_the_original_image() {
    let source = DynamicImage::ImageRgb8(RgbImage::from_fn(256, 128, |x, _| {
        image::Rgb([x as u8, 0, 0])
    }));
    let mut encoded = std::io::Cursor::new(Vec::new());
    source.write_to(&mut encoded, ImageFormat::Png).unwrap();
    let zoomed = crop_image_to_png_with_zoom(encoded.get_ref(), 4.).unwrap();
    let clamped = crop_image_to_png_with_zoom(encoded.get_ref(), 20.).unwrap();
    assert_eq!(zoomed, clamped);
    assert_ne!(crop_image_to_png(encoded.get_ref()).unwrap(), zoomed);
}
