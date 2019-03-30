use std::collections::HashMap;
use crate::common::bitmatrix::BitMatrix;
use crate::common::detector_result::DetectorResult;
use crate::decode_hint_type::DecodeHintType;
use crate::qrcode::detector::finder_pattern_info::FinderPatternInfo;
use crate::result_point::ResultPoint;

pub struct Detector {
    pub image: BitMatrix,
}

impl Detector {
    pub fn detect(&self, hints: Option<HashMap<DecodeHintType, u8>>) -> DetectorResult {
        ;
    }

    fn process_finder_pattern_info(&self, info: FinderPatternInfo) -> DetectorResult {
        let bits: BitMatrix;
        let points: Vec<ResultPoint>;

        return DetectorResult {
            bits: bits,
            points: points,
        }
    }
}