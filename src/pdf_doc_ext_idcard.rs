use crate::pdf_doc_util::{PdfDocUtil, calc_avg_dpi};

use printpdf::{
    BuiltinFont, Color, Line, LineDashPattern, LinePoint, Mm, Op, Point, Pt, Rgb, TextItem,
    XObjectTransform,
};

pub(crate) trait PdfDocIdCardExt {
    fn add_card_side(&mut self, image_path: &String, text: Option<String>);
}

impl PdfDocIdCardExt for PdfDocUtil {
    fn add_card_side(&mut self, image_path: &String, text: Option<String>) {
        let mut current_x = self.cfg.margin_left;
        let mut current_y = self.cfg.page_height - self.cfg.margin_top - self.cfg.card_height;

        let mut ops: Vec<Op> = Vec::new();

        let image = self.load_and_process_image(image_path);
        let image_id = self.document.add_image(&image);

        for idx in 1..=8 {
            // --- Placement Logic ---
            if current_x + self.cfg.card_width > self.cfg.page_width - self.cfg.margin_left {
                current_x = self.cfg.margin_left;
                current_y -= self.cfg.card_height + self.cfg.vertical_spacing;
            }

            if current_y < self.cfg.margin_bottom {
                self.add_page_to_document(std::mem::take(&mut ops));
                println!("ops len after mem::take {}", ops.len());
                current_y = self.cfg.page_height - self.cfg.margin_top - self.cfg.card_height;
            }

            let avg_dpi = calc_avg_dpi(&self.cfg.card_width, &self.cfg.card_height, &image);

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

            if idx % 2 == 0 {
                ops.append(&mut vec![
                    Op::SetOutlineColor {
                        col: Color::Rgb(Rgb {
                            r: 0.0,
                            g: 0.0,
                            b: 0.0,
                            icc_profile: None,
                        }),
                    },
                    Op::SetLineDashPattern {
                        dash: LineDashPattern {
                            offset: 0,
                            dash_1: Some(10),
                            gap_1: Some(5),
                            dash_2: None,
                            gap_2: None,
                            dash_3: None,
                            gap_3: None,
                        },
                    },
                    Op::DrawLine {
                        line: Line {
                            points: vec![
                                LinePoint {
                                    p: Point {
                                        x: Pt(0.0),
                                        y: (current_y - self.cfg.vertical_spacing / 2.0).into_pt(),
                                    },
                                    bezier: false,
                                },
                                LinePoint {
                                    p: Point {
                                        x: self.cfg.page_width.into_pt(),
                                        y: (current_y - self.cfg.vertical_spacing / 2.0).into_pt(),
                                    },
                                    bezier: false,
                                },
                            ],
                            is_closed: false,
                        },
                    },
                ]);
            }

            current_x += self.cfg.card_width + self.cfg.horizontal_spacing;
        }
        ops.push(Op::DrawLine {
            line: Line {
                points: vec![
                    LinePoint {
                        p: Point {
                            x: (self.cfg.page_width / 2.0).into_pt(),
                            y: self.cfg.page_height.into_pt(),
                        },
                        bezier: false,
                    },
                    LinePoint {
                        p: Point {
                            x: (self.cfg.page_width / 2.0).into_pt(),
                            y: (current_y - self.cfg.vertical_spacing / 2.0).into_pt(),
                        },
                        bezier: false,
                    },
                ],
                is_closed: false,
            },
        });

        if let Some(text) = text {
            ops.append(&mut vec![
                // Save the graphics state to allow for position resets later
                Op::SaveGraphicsState,
                // Start a text section (required for text operations)
                Op::StartTextSection,
                // Position the text cursor from the bottom left
                Op::SetTextCursor {
                    pos: Point::new(
                        (self.cfg.page_width / 2.0) - Mm(text.len() as f32) * 1.8,
                        current_y - Mm(20.0),
                    ),
                },
                // Set a built-in font (Helvetica) with its size
                Op::SetFontSizeBuiltinFont {
                    size: Pt(20.0),
                    font: BuiltinFont::Helvetica,
                },
                Op::SetLineHeight { lh: Pt(20.0) },
                // Set text color to blue
                Op::SetFillColor {
                    col: Color::Rgb(Rgb {
                        r: 0.0,
                        g: 0.0,
                        b: 0.0,
                        icc_profile: None,
                    }),
                },
                // Write text with the built-in font
                Op::WriteTextBuiltinFont {
                    items: vec![TextItem::Text(text)],
                    font: BuiltinFont::Helvetica,
                },
                // End the text section
                Op::EndTextSection,
                // Restore the graphics state
                Op::RestoreGraphicsState,
            ]);
        }

        self.add_page_to_document(ops);
    }
}
