// mod image_processing_utilities;

// use crate::image_processing_utilities::ImageProcessingUtilities;

// fn maina() {
//     let input = image::open("input.jpg").expect("Failed to open image");

//     let image = if let Some(cropped) = ImageProcessingUtilities::crop_to_subject(&input) {
//         cropped
//     } else {
//         println!("No subject found to crop.");
//         input
//     };

//     let image = ImageProcessingUtilities::apply_brightness_contrast(&image, 0, 30);
//     // let image = enhance_image(&image);
//     image
//         .save("output.jpg")
//         .expect("Failed to save output image");
//     println!("Saved cropped image!");
// }

// use image::DynamicImage;

use image::DynamicImage;
use printpdf::{
    Mm, Op, PdfDocument, PdfPage, PdfSaveOptions, Pt, Px, RawImage, XObjectRotation,
    XObjectTransform,
};

mod extensions;
mod image_processing_utilities;

use crate::extensions::MyRawImage;
use crate::image_processing_utilities::ImageProcessingUtilities;

// --- Configuration ---
// You can adjust these values to fit your needs.

// Page dimensions (A4 paper)
const PAGE_WIDTH: Mm = Mm(210.0);
const PAGE_HEIGHT: Mm = Mm(297.0);

// ID Card dimensions (CR80 is the standard size for credit cards/ID cards)
const CARD_WIDTH: Mm = Mm(85.6);
const CARD_HEIGHT: Mm = Mm(54.0);

// Margins from the edge of the page
const MARGIN_TOP: Mm = Mm(15.0);
const MARGIN_LEFT: Mm = Mm(10.0);
// A right margin is implicitly handled by the line-wrapping logic
const MARGIN_BOTTOM: Mm = Mm(10.0);

// Spacing between individual cards
const HORIZONTAL_SPACING: Mm = Mm(10.0);
const VERTICAL_SPACING: Mm = Mm(10.0);

fn calc_dpi(width: &Mm, height: &Mm, img: &RawImage) -> f32 {
    let width_in = width.0 / 25.4_f32;
    let height_in = height.0 / 25.4_f32;
    let width_dpi = img.width as f32 / width_in;
    let height_dpi = img.height as f32 / height_in;
    (width_dpi + height_dpi) / 2.0
}

fn main() {
    // Create a new PDF document
    let mut doc = PdfDocument::new("Image Example");

    // doc.shape_text(text, font_id, options)

    let mut page_count = 1;
    // --- Position Tracking ---
    let mut current_x = MARGIN_LEFT;
    let mut current_y = PAGE_HEIGHT - MARGIN_TOP - CARD_HEIGHT;

    // Create operations for our page
    let mut ops = Vec::new();

    // let image_bytes = include_bytes!("../input.jpg");
    let dyn_input = image::open("input.jpg").expect("Failed to open image");
    let image_processed = ImageProcessingUtilities::apply_brightness_contrast(&dyn_input, 0, 30);
    // let image = RawImage::decode_from_bytes(image_bytes, &mut Vec::new()).unwrap();

    let image_width_px = image_processed.width() as f32;
    let image_height_px = image_processed.height() as f32;
    // Calculate the scale factors for both width and height
    let scale_x_factor = CARD_WIDTH.0 as f32 / image_width_px;
    let scale_y_factor = CARD_HEIGHT.0 as f32 / image_height_px;

    // Use the smaller scale factor to maintain aspect ratio
    let scale_factor = scale_x_factor.min(scale_y_factor);

    let mut warnings = Vec::new();
    let image = RawImage::from_dynamic_image(image_processed, &mut warnings).unwrap();
    println!("Warnings: {:?}", warnings);

    // Add the image to the document resources and get its ID
    let image_id = doc.add_image(&image);

    for i in 1..=8 {
        // --- Placement Logic ---
        if current_x + CARD_WIDTH > PAGE_WIDTH - MARGIN_LEFT {
            current_x = MARGIN_LEFT;
            current_y -= CARD_HEIGHT + VERTICAL_SPACING;
        }

        if current_y < MARGIN_BOTTOM {
            page_count += 1;
            current_y = PAGE_HEIGHT - MARGIN_TOP - CARD_HEIGHT;
        }

        let avg_dpi = calc_dpi(&CARD_WIDTH,&CARD_HEIGHT,&image);

        ops.push(Op::UseXobject {
            id: image_id.clone(),
            transform: XObjectTransform {
                translate_x: Some(current_x.into_pt()),
                translate_y: Some(current_y.into_pt()),
                rotate: None,
                scale_x: Some(1.0),
                scale_y: Some(1.0),
                dpi: Some(avg_dpi),
            },
        });

        current_x += CARD_WIDTH + HORIZONTAL_SPACING;
    }

    // Create a page with our operations
    let page = PdfPage::new(Mm(210.0), Mm(297.0), ops);

    // Save the PDF to a file
    let bytes = doc
        .with_pages(vec![page])
        .save(&PdfSaveOptions::default(), &mut Vec::new());

    std::fs::write("./image_example.pdf", bytes).unwrap();
    println!("Created image_example.pdf");
}
