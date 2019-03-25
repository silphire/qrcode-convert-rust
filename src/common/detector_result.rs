use crate::result_point::ResultPoint;
use crate::common::bitmatrix::BitMatrix;

pub struct DetectorResult {
    bits: BitMatrix,
    points: Vec<ResultPoint>,
}
