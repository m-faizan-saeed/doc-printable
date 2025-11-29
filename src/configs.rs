use printpdf::Mm;

pub struct PageMarginConfig {
    pub page_width: Mm,
    pub page_height: Mm,
    pub card_width: Mm,
    pub card_height: Mm,
    pub margin_top: Mm,
    pub margin_left: Mm,
    pub margin_bottom: Mm,
    pub horizontal_spacing: Mm,
    pub vertical_spacing: Mm,
}

impl Default for PageMarginConfig {
    fn default() -> Self {
        Self {
            page_width: Mm(210.0),
            page_height: Mm(297.0),
            card_width: Mm(85.6),
            card_height: Mm(54.0),
            margin_top: Mm(10.0),
            margin_left: Mm(10.0),
            margin_bottom: Mm(10.0),
            horizontal_spacing: Mm(18.0),
            vertical_spacing: Mm(10.0),
        }
    }
}
