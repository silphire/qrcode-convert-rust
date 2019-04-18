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
}