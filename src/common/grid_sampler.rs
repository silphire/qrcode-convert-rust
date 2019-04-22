use crate::common::bitmatrix::BitMatrix;
use crate::common::perspective_transform::PerspectiveTransform;

pub trait GridSampler {
    fn sample_grid(&self, image: BitMatrix, dimension_x: isize, dimension_y: isize, transform: PerspectiveTransform);
}