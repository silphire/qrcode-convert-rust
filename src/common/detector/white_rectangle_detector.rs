use crate::common::bitmatrix::BitMatrix;
use crate::result_point::ResultPoint;

pub struct WhiteRectangleDetector {
    image: BitMatrix,
    height: usize,
    width: usize,
    left_init: usize,
    right_init: usize,
    down_init: usize,
    up_init: usize,
}

impl WhiteRectangleDetector {
    pub fn detect(&self) -> &Vec<ResultPoint> {
        return &vec![];
    }
}