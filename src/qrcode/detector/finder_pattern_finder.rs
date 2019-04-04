use std::any::Any;
use std::collections::HashMap;
use crate::common::bitmatrix::BitMatrix;
use crate::decode_hint_type::DecodeHintType;
use crate::result_point::ResultPoint;
use crate::qrcode::detector::finder_pattern_info::FinderPatternInfo;

pub struct FinderPatternFinder {
    image: BitMatrix,
    possible_centers: Vec<isize>,
    cross_check_state_count: [isize; 5],
    result_point_callback: fn(point: &ResultPoint),
}

const CENTER_QUORUM: isize = 2;
const MIN_SKIP: isize = 3;
const MAX_MODULES: isize = 97;

impl FinderPatternFinder {
    pub fn new(image: BitMatrix, result_point_callback: &fn(point: &ResultPoint)) -> FinderPatternFinder {
        unimplemented!();
    }

    pub fn find_with_hints(&self, hints: Option<HashMap<DecodeHintType, &Any>>) -> FinderPatternInfo {
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

    fn found_pattern_cross(state_count: [isize; 5]) -> bool {
        let total_module_size = 0;
        for i in 0..5 {
            let count = state_count[i];
            if count == 0 {
                return false;
            }
            total_module_size += count;
        }
        if total_module_size < 7 {
            return false;
        }

        let module_size = total_module_size as f64 / 7.0;
        let max_variance = module_size / 2.0;

        return 
            (module_size - state_count[0] as f64).abs() < max_variance && 
            (module_size - state_count[1] as f64).abs() < max_variance && 
            (3.0 * module_size - state_count[2] as f64).abs() < 3.0 * max_variance && 
            (module_size - state_count[3] as f64).abs() < max_variance && 
            (module_size - state_count[4] as f64).abs() < max_variance; 
    }
}