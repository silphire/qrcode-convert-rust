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
    result_point_callback: Option<fn(point: &ResultPoint)>,
}

const CENTER_QUORUM: isize = 2;
const MIN_SKIP: isize = 3;
const MAX_MODULES: isize = 97;

impl FinderPatternFinder {
    pub const fn get_image(&self) -> &BitMatrix {
        return &self.image;
    }

    pub fn new(image: BitMatrix, result_point_callback: Option<fn(point: &ResultPoint)>) -> FinderPatternFinder {
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

    fn found_pattern_diagonal(state_count: [isize; 5]) -> bool {
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
        let max_variance = module_size / 1.333;

        return 
            (module_size - state_count[0] as f64).abs() < max_variance && 
            (module_size - state_count[1] as f64).abs() < max_variance && 
            (3.0 * module_size - state_count[2] as f64).abs() < max_variance &&
            (module_size - state_count[3] as f64).abs() < max_variance &&
            (module_size - state_count[4] as f64).abs() < max_variance;
    }

    fn get_cross_check_state_count(&self) -> [isize; 5] {
        self.clear_counts(self.cross_check_state_count);
        return self.cross_check_state_count;
    }

    fn clear_counts(&self, counts: [isize; 5]) {
        for x in 0..counts.len() {
            counts[x] = 0;
        }
    }
    
    fn shift_counts_2(&self, state_count: &mut [isize; 5]) {
        state_count[0] = state_count[2];
        state_count[1] = state_count[3];
        state_count[2] = state_count[4];
        state_count[3] = 1;
        state_count[4] = 0;
    }

    fn cross_check_diagonal(&self, center_i: isize, center_j: isize) -> bool {
        let state_count = self.get_cross_check_state_count();

        let i = 0;
        while center_i >= i && center_j >= i && self.image.get(center_j - i, center_i - i) {
            state_count[2] += 1;
            i += 1;
        }

        if state_count[2] == 0 {
            return false;
        }

        while center_i >= i && center_j >= i && self.image.get(center_j - i, center_i - i) {
            state_count[1] += 1;
            i += 1;
        }

        if state_count[1] == 0 {
            return false;
        }

        while center_i >= i && center_j >= i && self.image.get(center_j - i, center_i - i) {
            state_count[0] += 1;
            i += 1;
        }

        if state_count[0] == 0 {
            return false;
        }

        let max_i = self.get_image().height;
        let max_j = self.get_image().width;

        i = 1;
        while center_i + i < max_i && center_j + i < max_j && self.image.get(center_j + i, center_i + i) {
            state_count[2] += 1;
            i += 1;
        }

        while center_i + i < max_i && center_j + i < max_j && !self.image.get(center_j + i, center_i + i) {
            state_count[3] += 1;
            i += 1;
        }

        if state_count[3] == 0 {
            return false;
        }

        while center_i + i < max_i && center_j + i < max_j && self.image.get(center_j + i, center_i + i) {
            state_count[4] += 1;
            i += 1;
        }

        if state_count[4] == 0 {
            return false;
        }

        return Self::found_pattern_diagonal(state_count);
    }

    fn cross_check_vertical(&self, start_i: isize, center_j: isize, max_count: isize, original_state_count_total: isize) -> f64 {
        let image = self.get_image();
        let max_i = image.height;
        let state_count = self.get_cross_check_state_count();

        let i = start_i;
        while i >= 0 && image.get(center_j, i) {
            state_count[2] += 1;
            i -= 1;
        }
        if i < 0 {
            return std::f64::NAN;
        }
        while i >= 0 && !image.get(center_j, i) && state_count[1] <= max_count {
            state_count[1] += 1;
            i -= 1;
        }
        if i < 0 || state_count[1] > max_count {
            return std::f64::NAN;
        }
        while i >= 0 && image.get(center_j, i) && state_count[0] <= max_count {
            state_count[0] += 1;
            i -= 1;
        }
        if state_count[0] > max_count {
            return std::f64::NAN;
        }

        i = start_i + 1;
        while i < max_i && image.get(center_j, i) {
            state_count[2] += 1;
            i += 1;
        }
        if i == max_i {
            return std::f64::NAN;
        }
        while i < max_i && !image.get(center_j, i) && state_count[3] < max_count {
            state_count[3] += 1;
            i += 1;
        }
        if i == max_i || state_count[3] >= max_count {
            return std::f64::NAN;
        }
        while i < max_i && image.get(center_j, i) && state_count[4] < max_count {
            state_count[4] += 1;
            i += 1;
        }
        if state_count[4] >= max_count {
            return std::f64::NAN;
        }

        let state_count_total = 0;
        for i in &state_count {
            state_count_total += i;
        }
        if 5 * (state_count_total - original_state_count_total).abs() >= 2 * original_state_count_total {
            return std::f64::NAN;
        }

        return if Self::found_pattern_cross(state_count) { self.center_from_end(&state_count, i) } else { std::f64::NAN };
    }

    fn cross_check_horizontal(&self, start_j: isize, center_i: isize, max_count: isize, original_state_count_total: isize) -> f64 {
        let image = self.get_image();

        let max_j = image.width;
        let state_count = self.get_cross_check_state_count();

        let j = start_j;
        while j >= 0 && image.get(j, center_i) {
            state_count[2] += 1;
            j -= 1;
        }
        if j < 0 {
            return std::f64::NAN;
        }
        while j >= 0 && !image.get(j, center_i) && state_count[1] <= max_count {
            state_count[1] += 1;
            j -= 1;
        }
        if j < 0 || state_count[1] > max_count {
            return std::f64::NAN;
        }
        while j >= 0 && image.get(j, center_i) && state_count[0] <= max_count {
            state_count[0] += 1;
            j -= 1;
        }
        if state_count[0] > max_count {
            return std::f64::NAN;
        }

        j = start_j + 1;
        while j < max_j && image.get(j, center_i) {
            state_count[2] += 1;
            j += 1;
        }
        if j == max_j {
            return std::f64::NAN;
        }
        while j < max_j && !image.get(j, center_i) && state_count[3] < max_count {
            state_count[3] += 1;
            j += 1;
        }
        if j == max_j || state_count[3] >= max_count {
            return std::f64::NAN;
        }
        while j < max_j && image.get(j, center_i) && state_count[4] < max_count {
            state_count[4] += 1;
            j += 1;
        }
        if state_count[4] >= max_count {
            return std::f64::NAN;
        }

        let state_count_total = 0;
        for i in &state_count {
            state_count_total += i;
        }
        if 5 * (state_count_total - original_state_count_total).abs() >= original_state_count_total {
            return std::f64::NAN;
        }

        return if Self::found_pattern_cross(state_count) { self.center_from_end(&state_count, j) } else { std::f64::NAN };
    }

    fn handle_possible_center(state_count: [isize; 5], i: isize, j: isize) -> bool {
        unimplemented!();
    }
}