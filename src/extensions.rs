use image::DynamicImage;
use image::GenericImageView;
use printpdf::PdfWarnMsg;
use printpdf::RawImage;
use printpdf::RawImageData;
use printpdf::RawImageFormat;

pub trait RawImageExt {
    fn from_dynamic_image(
        im: DynamicImage,
        warnings: &mut Vec<PdfWarnMsg>,
    ) -> Result<RawImage, String>;
}

impl RawImageExt for RawImage {
    fn from_dynamic_image(
        im: DynamicImage,
        warnings: &mut Vec<PdfWarnMsg>,
    ) -> Result<RawImage, String> {
        use image::DynamicImage::*;

        let (w, h) = im.dimensions();
        warnings.push(PdfWarnMsg::info(
            0,
            0,
            format!("Image dimensions: {}x{} pixels", w, h),
        ));

        // Map the color type with informative messages
        let ct = match im.color() {
            image::ColorType::L8 => {
                warnings.push(PdfWarnMsg::info(
                    0,
                    0,
                    "Detected grayscale (L8) image".to_string(),
                ));
                RawImageFormat::R8
            }
            image::ColorType::La8 => {
                warnings.push(PdfWarnMsg::info(
                    0,
                    0,
                    "Detected grayscale with alpha (La8) image".to_string(),
                ));
                RawImageFormat::RG8
            }
            image::ColorType::Rgb8 => {
                warnings.push(PdfWarnMsg::info(
                    0,
                    0,
                    "Detected RGB (Rgb8) image".to_string(),
                ));
                RawImageFormat::RGB8
            }
            image::ColorType::Rgba8 => {
                warnings.push(PdfWarnMsg::info(
                    0,
                    0,
                    "Detected RGBA (Rgba8) image".to_string(),
                ));
                RawImageFormat::RGBA8
            }
            image::ColorType::L16 => {
                warnings.push(PdfWarnMsg::info(
                    0,
                    0,
                    "Detected 16-bit grayscale (L16) image".to_string(),
                ));
                RawImageFormat::R16
            }
            image::ColorType::La16 => {
                warnings.push(PdfWarnMsg::info(
                    0,
                    0,
                    "Detected 16-bit grayscale with alpha (La16) image".to_string(),
                ));
                RawImageFormat::RG16
            }
            image::ColorType::Rgb16 => {
                warnings.push(PdfWarnMsg::info(
                    0,
                    0,
                    "Detected 16-bit RGB (Rgb16) image".to_string(),
                ));
                RawImageFormat::RGB16
            }
            image::ColorType::Rgba16 => {
                warnings.push(PdfWarnMsg::info(
                    0,
                    0,
                    "Detected 16-bit RGBA (Rgba16) image".to_string(),
                ));
                RawImageFormat::RGBA16
            }
            image::ColorType::Rgb32F => {
                warnings.push(PdfWarnMsg::info(
                    0,
                    0,
                    "Detected 32-bit float RGB (Rgb32F) image".to_string(),
                ));
                RawImageFormat::RGBF32
            }
            image::ColorType::Rgba32F => {
                warnings.push(PdfWarnMsg::info(
                    0,
                    0,
                    "Detected 32-bit float RGBA (Rgba32F) image".to_string(),
                ));
                RawImageFormat::RGBAF32
            }
            other => {
                let err_msg = format!("Unsupported color type: {:?}", other);
                warnings.push(PdfWarnMsg::warning(0, 0, err_msg.clone()));
                return Err("invalid raw image format".to_string());
            }
        };

        // Extract pixel data
        let pixels = match im {
            ImageLuma8(image_buffer) => {
                warnings.push(PdfWarnMsg::info(
                    0,
                    0,
                    format!(
                        "Converting ImageLuma8 buffer of {} pixels",
                        image_buffer.len()
                    ),
                ));
                RawImageData::U8(image_buffer.into_raw())
            }
            ImageLumaA8(image_buffer) => {
                warnings.push(PdfWarnMsg::info(
                    0,
                    0,
                    format!(
                        "Converting ImageLumaA8 buffer of {} pixels",
                        image_buffer.len()
                    ),
                ));
                RawImageData::U8(image_buffer.into_raw())
            }
            ImageRgb8(image_buffer) => {
                warnings.push(PdfWarnMsg::info(
                    0,
                    0,
                    format!(
                        "Converting ImageRgb8 buffer of {} pixels",
                        image_buffer.len()
                    ),
                ));
                RawImageData::U8(image_buffer.into_raw())
            }
            ImageRgba8(image_buffer) => {
                warnings.push(PdfWarnMsg::info(
                    0,
                    0,
                    format!(
                        "Converting ImageRgba8 buffer of {} pixels",
                        image_buffer.len()
                    ),
                ));
                RawImageData::U8(image_buffer.into_raw())
            }
            ImageLuma16(image_buffer) => {
                warnings.push(PdfWarnMsg::info(
                    0,
                    0,
                    format!(
                        "Converting ImageLuma16 buffer of {} pixels",
                        image_buffer.len()
                    ),
                ));
                RawImageData::U16(image_buffer.into_raw())
            }
            ImageLumaA16(image_buffer) => {
                warnings.push(PdfWarnMsg::info(
                    0,
                    0,
                    format!(
                        "Converting ImageLumaA16 buffer of {} pixels",
                        image_buffer.len()
                    ),
                ));
                RawImageData::U16(image_buffer.into_raw())
            }
            ImageRgb16(image_buffer) => {
                warnings.push(PdfWarnMsg::info(
                    0,
                    0,
                    format!(
                        "Converting ImageRgb16 buffer of {} pixels",
                        image_buffer.len()
                    ),
                ));
                RawImageData::U16(image_buffer.into_raw())
            }
            ImageRgba16(image_buffer) => {
                warnings.push(PdfWarnMsg::info(
                    0,
                    0,
                    format!(
                        "Converting ImageRgba16 buffer of {} pixels",
                        image_buffer.len()
                    ),
                ));
                RawImageData::U16(image_buffer.into_raw())
            }
            ImageRgb32F(image_buffer) => {
                warnings.push(PdfWarnMsg::info(
                    0,
                    0,
                    format!(
                        "Converting ImageRgb32F buffer of {} pixels",
                        image_buffer.len()
                    ),
                ));
                RawImageData::F32(image_buffer.into_raw())
            }
            ImageRgba32F(image_buffer) => {
                warnings.push(PdfWarnMsg::info(
                    0,
                    0,
                    format!(
                        "Converting ImageRgba32F buffer of {} pixels",
                        image_buffer.len()
                    ),
                ));
                RawImageData::F32(image_buffer.into_raw())
            }
            _ => {
                warnings.push(PdfWarnMsg::warning(
                    0,
                    0,
                    "Invalid pixel format".to_string(),
                ));
                return Err("invalid pixel format".to_string());
            }
        };

        warnings.push(PdfWarnMsg::info(
            0,
            0,
            "Image decoded successfully".to_string(),
        ));

        Ok(RawImage {
            pixels,
            width: w as usize,
            height: h as usize,
            data_format: ct,
            tag: Vec::new(),
        })
    }
}
