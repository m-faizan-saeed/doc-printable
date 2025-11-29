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
const MARGIN_TOP: Mm = Mm(10.0);
const MARGIN_LEFT: Mm = Mm(10.0);
// A right margin is implicitly handled by the line-wrapping logic
const MARGIN_BOTTOM: Mm = Mm(10.0);

// Spacing between individual cards
const HORIZONTAL_SPACING: Mm = Mm(5.0);
const VERTICAL_SPACING: Mm = Mm(5.0);

fn main() {
    // Create a new PDF document
    let mut doc = PdfDocument::new("Image Example");

    // Load an image
    // let image_bytes = include_bytes!("../input.jpg");
    let dyn_input = image::open("input.jpg").expect("Failed to open image");

    let image_processed = ImageProcessingUtilities::apply_brightness_contrast(&dyn_input, 0, 30);
    // let image = RawImage::decode_from_bytes(image_bytes, &mut Vec::new()).unwrap();
    let image = RawImage::from_dynamic_image(image_processed, &mut Vec::new()).unwrap();

    // Create operations for our page
    let mut ops = Vec::new();

    // Add the image to the document resources and get its ID
    let image_id = doc.add_image(&image);

    // Place the image with default transform (at 0,0)
    ops.push(Op::UseXobject {
        id: image_id.clone(),
        transform: XObjectTransform::default(),
    });

    // Place the same image again, but translated, rotated, and scaled
    ops.push(Op::UseXobject {
        id: image_id.clone(),
        transform: XObjectTransform {
            translate_x: Some(Pt(300.0)),
            translate_y: Some(Pt(300.0)),
            rotate: Some(XObjectRotation {
                angle_ccw_degrees: 45.0,
                rotation_center_x: Px(100),
                rotation_center_y: Px(100),
            }),
            scale_x: Some(0.5),
            scale_y: Some(0.5),
            dpi: Some(300.0),
        },
    });

    ops.push(Op::UseXobject {
        id: image_id.clone(),
        transform: XObjectTransform {
            translate_x: Some(Pt(300.0)),
            translate_y: Some(Pt(300.0)),
            rotate: None,
            scale_x: Some(0.5),
            scale_y: Some(0.5),
            dpi: Some(300.0),
        },
    });

    // Create a page with our operations
    let page = PdfPage::new(Mm(210.0), Mm(297.0), ops);

    // Save the PDF to a file
    let bytes = doc
        .with_pages(vec![page])
        .save(&PdfSaveOptions::default(), &mut Vec::new());

    std::fs::write("./image_example.pdf", bytes).unwrap();
    println!("Created image_example.pdf");
}
