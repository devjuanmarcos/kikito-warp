use image::{DynamicImage, GenericImageView, ImageFormat, RgbImage};

use super::{CROP_VIEW_SIZE, CropView};
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

fn assert_close(actual: (f32, f32, f32), expected: (f32, f32, f32)) {
    let close = |a: f32, b: f32| (a - b).abs() < 1e-4;
    assert!(
        close(actual.0, expected.0) && close(actual.1, expected.1) && close(actual.2, expected.2),
        "{actual:?} != {expected:?}"
    );
}

#[test]
fn crop_view_starts_centered_with_the_short_side_filling_the_viewport() {
    let wide = CropView::centered(400, 200);
    let (width, height) = wide.displayed_size();
    assert_close(
        (width, height, wide.x),
        (2. * CROP_VIEW_SIZE, CROP_VIEW_SIZE, -CROP_VIEW_SIZE / 2.),
    );
    assert_eq!(wide.y, 0.);
    assert_close(wide.crop_parameters(), (1., 0., 0.));
}

#[test]
fn crop_view_never_uncovers_the_viewport() {
    let view = CropView::centered(400, 200).moved_to(500., -500.);
    assert_eq!((view.x, view.y), (0., 0.));
    let view = view.moved_to(-10_000., 0.);
    assert!((view.x + CROP_VIEW_SIZE).abs() < 1e-3);
    assert_eq!(view.zoomed(8.).zoom, 4.);
    assert_eq!(view.zoomed(0.1).zoom, 1.);
}

#[test]
fn crop_view_maps_edges_to_full_pan_and_keeps_the_anchor_while_zooming() {
    let square = CropView::centered(200, 200).zoomed(2.);
    let (zoom, pan_x, pan_y) = square.moved_to(0., -10_000.).crop_parameters();
    assert_close((zoom, pan_x, pan_y), (2., -1., 1.));
    let anchored = CropView::centered(200, 200).zoomed_at(2., 0., 0.);
    assert_eq!((anchored.x, anchored.y), (0., 0.));

    let source = DynamicImage::ImageRgb8(RgbImage::from_fn(256, 256, |x, y| {
        image::Rgb([x as u8, y as u8, 0])
    }));
    let mut encoded = std::io::Cursor::new(Vec::new());
    source.write_to(&mut encoded, ImageFormat::Png).unwrap();
    let center = crop_image_to_png_with_view(encoded.get_ref(), 2., 0., 0.).unwrap();
    let corner = crop_image_to_png_with_view(encoded.get_ref(), 2., pan_x, pan_y).unwrap();
    assert_ne!(center, corner);
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
