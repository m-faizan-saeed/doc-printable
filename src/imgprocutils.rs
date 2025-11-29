use image::DynamicImage;
use image::Rgba;
use image::RgbaImage;
use imageproc::contours::{Contour, find_contours_with_threshold};
use imageproc::contrast::equalize_histogram;
use imageproc::contrast::{ThresholdType, threshold_mut};
use imageproc::distance_transform::Norm;
use imageproc::morphology::close;
use imageproc::rect::Rect;

pub struct ImgProcUtils {}

impl ImgProcUtils {
    /// Compute bounding rectangle from a list of points
    fn bounding_rect_from_points(points: &[imageproc::point::Point<u32>]) -> Rect {
        let (min_x, max_x) = points
            .iter()
            .map(|p| p.x)
            .fold((u32::MAX, u32::MIN), |(min, max), x| {
                (min.min(x), max.max(x))
            });

        let (min_y, max_y) = points
            .iter()
            .map(|p| p.y)
            .fold((u32::MAX, u32::MIN), |(min, max), y| {
                (min.min(y), max.max(y))
            });

        Rect::at(min_x as i32, min_y as i32).of_size(max_x - min_x + 1, max_y - min_y + 1)
    }

    /// Crop the image to the main subject by removing white background.
    pub fn crop_to_subject(input: &DynamicImage) -> Option<DynamicImage> {
        // 1. Convert to grayscale
        let mut gray = input.to_luma8();

        // 2. Threshold: invert the white background
        threshold_mut(&mut gray, 240, ThresholdType::BinaryInverted);

        // 3. Morphological closing to remove noise
        let morphed = close(&gray, Norm::L2, 11);

        // 4. Find contours
        let contours: Vec<Contour<u32>> = find_contours_with_threshold(&morphed, 1);

        // 5. Get the largest contour's bounding box
        let largest_rect = contours
            .iter()
            .map(|c| ImgProcUtils::bounding_rect_from_points(&c.points))
            .max_by_key(|r| r.width() * r.height())?;

        // 6. Crop the original image
        let cropped = input.crop_imm(
            largest_rect.left() as u32,
            largest_rect.top() as u32,
            largest_rect.width(),
            largest_rect.height(),
        );

        Some(cropped)
    }

    /// Adjust brightness and contrast of an image.
    /// `brightness`: -255 to +255
    /// `contrast`: -127 to +127
    pub fn apply_brightness_contrast(
        input: &DynamicImage,
        brightness: i32,
        contrast: i32,
    ) -> DynamicImage {
        let mut img = input.to_rgba8();
        let (width, height) = img.dimensions();

        // Brightness
        let (alpha_b, gamma_b) = if brightness != 0 {
            let shadow = if brightness > 0 { brightness } else { 0 };
            let highlight = if brightness > 0 {
                255
            } else {
                255 + brightness
            };

            let alpha_b = (highlight - shadow) as f32 / 255.0;
            let gamma_b = shadow as f32;

            (alpha_b, gamma_b)
        } else {
            (1.0, 0.0)
        };

        // Contrast
        let (alpha_c, gamma_c) = if contrast != 0 {
            let f = 131.0 * (contrast as f32 + 127.0) / (127.0 * (131.0 - contrast as f32));
            let alpha_c = f;
            let gamma_c = 127.0 * (1.0 - f);
            (alpha_c, gamma_c)
        } else {
            (1.0, 0.0)
        };

        // Apply brightness and contrast
        for y in 0..height {
            for x in 0..width {
                let pixel = img.get_pixel(x, y);
                let rgba = pixel.0;

                // Apply brightness first, then contrast
                let adjusted = rgba
                    .iter()
                    .enumerate()
                    .map(|(i, &channel)| {
                        if i == 3 {
                            // Alpha channel unchanged
                            return channel;
                        }

                        let mut value = channel as f32;
                        value = alpha_b * value + gamma_b;
                        value = alpha_c * value + gamma_c;

                        value.clamp(0.0, 255.0) as u8
                    })
                    .collect::<Vec<u8>>();

                img.put_pixel(x, y, Rgba([adjusted[0], adjusted[1], adjusted[2], rgba[3]]));
            }
        }

        DynamicImage::ImageRgba8(img)
    }

    pub fn enhance_image(input: &DynamicImage) -> DynamicImage {
        // Convert to RGB8 (ignores alpha for now)
        let rgb = input.to_rgb8();
        let (width, height) = rgb.dimensions();

        // Step 1: Convert to grayscale (approximating L channel from LAB)
        let gray = image::imageops::grayscale(&DynamicImage::ImageRgb8(rgb.clone()));

        // Step 2: Apply global histogram equalization (not CLAHE but similar)
        let equalized = equalize_histogram(&gray);

        // Step 3: Reconstruct enhanced RGB by replacing brightness
        let mut enhanced_img = RgbaImage::new(width, height);

        for y in 0..height {
            for x in 0..width {
                let orig = rgb.get_pixel(x, y).0;
                let luma = equalized.get_pixel(x, y).0[0] as f32 / 255.0;

                // Scale original R, G, B by new "L" value
                let scale_channel = |c: u8| -> u8 {
                    let scaled = (c as f32 * luma).round();
                    scaled.clamp(0.0, 255.0) as u8
                };

                let r = scale_channel(orig[0]);
                let g = scale_channel(orig[1]);
                let b = scale_channel(orig[2]);

                enhanced_img.put_pixel(x, y, Rgba([r, g, b, 255]));
            }
        }

        DynamicImage::ImageRgba8(enhanced_img)
    }
}
