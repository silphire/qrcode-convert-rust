use crate::qrcode::detector::finder_pattern::FinderPattern;

pub struct FinderPatternInfo {
    pub bottom_left: FinderPattern,
    pub top_left: FinderPattern,
    pub top_right: FinderPattern,
}
