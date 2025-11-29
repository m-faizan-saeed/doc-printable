mod configs;
mod extensions;
mod imgprocutils;
mod pdf_doc_ext_idcard;
mod pdf_doc_util;

// use image::DynamicImage;
// use imageproc::{contrast::stretch_contrast, filter::gaussian_blur_f32};

use crate::{
    configs::PageMarginConfig, imgprocutils::ImgProcUtils, pdf_doc_ext_idcard::PdfDocIdCardExt,
    pdf_doc_util::PdfDocUtil,
};

use clap::Parser;
use std::path::PathBuf;

#[derive(Debug, Clone)]
enum ProcOp {
    Crop2Subject,
    Contrast(f32),
    Brightness(f32),
}

fn parse_proc_op(param_str: &str) -> Result<ProcOp, String> {
    let lower = param_str.to_lowercase();

    if lower == "c2s" {
        return Ok(ProcOp::Crop2Subject);
    } else if let Some(idx) = ["contrast", "brightness"]
        .iter()
        .position(|s| lower.starts_with(*s))
    {
        let parts: Vec<&str> = lower.split(':').collect();
        let val: f32 = parts[1].parse().map_err(|_| "Invalid Value.")?;
        // println!("str: {}, idx: {}", lower, idx);
        return Ok(match idx {
            0 => ProcOp::Contrast(val),
            _ => ProcOp::Brightness(val),
        });
    }
    Err("Unknown color".into())
}

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
pub struct Cli {
    /// A list of input images in sequence
    #[clap(short, long, num_args = 1..)] // Accepts one or more values
    pub input_images: Vec<PathBuf>,

    /// A list of images titles in sequence
    #[clap(short, long, num_args = 1..)] // Accepts one or more values
    pub titles: Vec<String>,

    /// Path to the output file
    #[arg(short, long, default_value = "./output.pdf")]
    output_path: PathBuf,

    /// Image processing Operations
    /// 1) c2s : Crop to Subject
    /// 2) contrast:30  Adjust Contrast
    /// 3) brightness:-10   Adjust Brightness
    #[arg(short = 'I' , long, value_enum, value_delimiter = ',', num_args=1.., value_parser = parse_proc_op)]
    image_processing_operation: Vec<ProcOp>,
}

// fn main() {
//     let mut pdf = PdfDocUtil::new(PageMarginConfig::default());
//     pdf.register_image_processor(|img| ImgProcUtils::crop_to_subject(&img).unwrap_or(img));
//     pdf.register_image_processor(|img| img.adjust_contrast(40.0));
//     // pdf.register_image_processor(|img| img.unsharpen(5.0, 1));
//     // pdf.register_image_processor(|img| {
//     //     // Convert the image to grayscale
//     //     let gray_img = img.to_luma8();
//     //     // Apply Gaussian blur to reduce noise
//     //     let blurred_img = gaussian_blur_f32(&gray_img, 1.0);
//     //     // Stretch the contrast of the image
//     //     let contrast_stretched_img = stretch_contrast(
//     //         &blurred_img,
//     //         50,  // min_value (lower bound of original image's pixel intensity)
//     //         180, // max_value (upper bound of original image's pixel intensity)
//     //         80,  // low_value (lower bound of the new contrast range)
//     //         200, // high_value (upper bound of the new contrast range)
//     //     );
//     //     // Convert back to RGB
//     //     DynamicImage::ImageLuma8(contrast_stretched_img)
//     //         .to_rgb8()
//     //         .into()
//     // });
//     pdf.add_card_side(&"./input.jpg".to_string(), Some("Card Front side".into()));
//     pdf.add_card_side(&"./input.jpg".to_string(), Some("Card Back side".into()));
//     pdf.save_pdf(&"./image_example.pdf".to_string());
// }

fn main() {
    let cli = Cli::parse();
    println!("CLI is {:#?}", cli);

    if cli.input_images.len() == 0 {
        println!("Must define atleast one image.");
        return;
    }

    let mut pdf = PdfDocUtil::new(PageMarginConfig::default());

    for opreation in cli.image_processing_operation {
        match opreation {
            ProcOp::Crop2Subject => pdf.register_image_processor(|img| {
                println!("Cropping To Subject");
                ImgProcUtils::crop_to_subject(&img).unwrap_or(img)
            }),
            ProcOp::Contrast(val) => pdf.register_image_processor(move |img| {
                println!("Applying Contrast");
                img.adjust_contrast(val)
            }),
            ProcOp::Brightness(val) => pdf.register_image_processor(move |img| {
                println!("Applying Brightness");
                img.brighten(val as i32)
            }),
        }
    }

    for (idx, path) in cli.input_images.iter().enumerate() {
        let path_string = path
            .to_str()
            .map(|s| s.to_string())
            .unwrap_or_else(|| String::from("Invalid path"));
        let title = cli.titles.get(idx);
        pdf.add_card_side(&path_string, title.cloned());
    }

    pdf.save_pdf(
        &cli.output_path
            .to_str()
            .map(|s| s.to_string())
            .unwrap_or_else(|| String::from("Invalid Output path")),
    );
}
