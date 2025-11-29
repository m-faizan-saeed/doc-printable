use image::DynamicImage;
use printpdf::{Mm, Op, PdfDocument, PdfPage, PdfSaveOptions, RawImage};

use crate::{configs::PageMarginConfig, extensions::RawImageExt};

pub(crate) fn calc_avg_dpi(width: &Mm, height: &Mm, img: &RawImage) -> f32 {
    let width_in = width.0 / 25.4_f32;
    let height_in = height.0 / 25.4_f32;
    let width_dpi = img.width as f32 / width_in;
    let height_dpi = img.height as f32 / height_in;
    (width_dpi + height_dpi) / 2.0
}

pub struct PdfDocUtil {
    pub(crate) document: PdfDocument,
    pub(crate) cfg: PageMarginConfig,
    image_processors: Vec<Box<dyn FnMut(DynamicImage) -> DynamicImage>>, // List of processing callbacks
}
impl PdfDocUtil {
    pub fn new(cfg: PageMarginConfig) -> Self {
        Self {
            document: PdfDocument::new("Image Example"),
            cfg,
            image_processors: Vec::new(),
        }
    }

    pub fn register_image_processor<F>(&mut self, callback: F)
    where
        F: FnMut(DynamicImage) -> DynamicImage + 'static, // Use FnMut instead of FnOnce
    {
        self.image_processors.push(Box::new(callback)); // Store the callback as a boxed trait object
    }

    pub(crate) fn add_page_to_document(&mut self, ops: Vec<Op>) {
        let page = PdfPage::new(self.cfg.page_width, self.cfg.page_height, ops);
        self.document.with_pages(vec![page]);
    }

    pub(crate) fn load_and_process_image(&mut self, image_path: &String) -> RawImage {
        print!("Loading Image {} ... ", image_path);
        let mut image = image::open(image_path).expect("Failed to open image");
        println!("Loaded");

        println!("Processing Image ...");
        let image = {
            for processor in &mut self.image_processors {
                // Use `&mut self.processors` to mutate the closures
                image = processor(image); // Apply the processing
            }
            image
        };
        println!("Image Processed");

        let image = RawImage::from_dynamic_image(image, &mut Vec::new()).unwrap();
        image
    }

    pub fn serialize_pdf(&self) -> Vec<u8> {
        self.document
            .save(&PdfSaveOptions::default(), &mut Vec::new())
    }

    pub fn save_pdf(&self, pdf_path: &String) {
        let bytes = self.serialize_pdf();
        std::fs::write(pdf_path, bytes).unwrap();
        println!("Created {}", pdf_path);
    }
}
