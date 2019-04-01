use std::collections::HashMap;
use crate::common::bitmatrix::BitMatrix;
use crate::decode_hint_type::DecodeHintType;
use crate::result_point::ResultPoint;
use crate::qrcode::detector::finder_pattern_info::FinderPatternInfo;

pub struct FinderPatternFinder {
    image: BitMatrix,
    possible_centers: Vec<isize>,
    cross_check_state_count: Vec<isize>,  // TODO fixed size
    result_point_callback: fn(point: &ResultPoint),
}

const CENTER_QUORUM: isize = 2;
const MIN_SKIP: isize = 3;
const MAX_MODULES: isize = 97;

impl FinderPatternFinder {
    fn find_with_hints(&self, hints: Option<HashMap<DecodeHintType, i32>>) -> FinderPatternInfo {
        let try_harder = hints.map_or(false, |v| v.contains_key(&DecodeHintType::TryHarder));
        let max_i = self.image.height;
        let max_j = self.image.width;

        let i_skip = (3 * max_i) / (4 * MAX_MODULES);
        if i_skip < MIN_SKIP || try_harder {
            i_skip = MIN_SKIP;
        }

        let done = false;
        let state_count: Vec<isize>;

        unimplemented!();
    }

    const fn center_from_end(&self, state_count: &[isize], end: isize) -> f64 {
        return (end - state_count[4] - state_count[3]) as f64 - state_count[2] as f64 / 2.0;
    }
}