use crate::common::bitmatrix::BitMatrix;
use crate::qrcode::detector::alignment_pattern::AlignmentPattern;

pub struct AlignmentPatternFinder {
    image: BitMatrix,
    possible_centers: Vec<AlignmentPattern>,
    start_x: isize,
    start_y: isize,
    width: isize,
    height: isize,
    module_size: isize,
    cross_check_state_count: [isize; 3],
    result_point_callback: &'static fn(pattern: AlignmentPattern),
}

impl AlignmentPatternFinder {
    pub fn new(image: BitMatrix, start_x: isize, start_y: isize, width: isize, height: isize, module_size: isize, result_point_callback: &'static fn(pattern: AlignmentPattern)) -> AlignmentPatternFinder {
        return AlignmentPatternFinder {
            image: image,
            possible_centers: vec![],
            start_x: start_x,
            start_y: start_y,
            width: width,
            height: height,
            module_size: module_size,
            cross_check_state_count: [0, 0, 0],
            result_point_callback: result_point_callback,
        };
    }

    pub fn find(&self) -> AlignmentPattern {
        let max_j = self.start_x + self.width;
        let middle_i = self.start_y + self.height / 2;

        let state_count: [isize; 3];
        
        for i_gen in 0..self.height {
            let i = middle_i + if i_gen & 0x01 == 0 { (i_gen + 1) / 2 } else { -(i_gen + 1) / 2 };
            state_count[0] = 0;
            state_count[1] = 0;
            state_count[2] = 0;
            
            let j = self.start_x;
            while j < max_j && !self.image.get(j, i) {
                j += 1;
            }

            let current_state = 0;
            while j < max_j {
                if self.image.get(j, i) {
                    // black pixel
                    if current_state == 1 {
                        state_count[1] += 1;
                    } else {
                        if current_state == 2 {
                            if self.found_pattern_cross(&state_count) {
                                if let Some(confirmed) = self.handle_possible_center(&state_count, i, j) {
                                    return confirmed;
                                }
                            }
                            state_count[0] = state_count[2];
                            state_count[1] = 1;
                            state_count[2] = 0;
                            current_state = 1;
                        } else {
                            current_state += 1;
                            state_count[current_state] += 1;
                        }
                    }
                } else {
                    // white pixel
                    if current_state == 1 {
                        current_state += 1;
                    }
                    state_count[current_state] += 1;
                }
                j += 1;
            }
            if self.found_pattern_cross(&state_count) {
                if let Some(confirmed) = self.handle_possible_center(&state_count, i, max_j) {
                    return confirmed;
                }
            }
        }

        if !self.possible_centers.is_empty() {
            return self.possible_centers[0];
        }
            
        unimplemented!();
    }

    fn found_pattern_cross(&self, state_count: &[isize; 3]) -> bool {
        unimplemented!();
    }

    fn handle_possible_center(&self, state_count: &[isize; 3], i: isize, j: isize) -> Option<AlignmentPattern> {
        unimplemented!();
    }
}