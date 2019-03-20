use crate::common::bitmatrix::BitMatrix;
use crate::result_point::ResultPoint;

pub struct MonochromeRectangleDetector {
    image: BitMatrix,
}

impl MonochromeRectangleDetector {
    pub fn detect(&self) -> Option<Vec<ResultPoint>> {
        const MAX_MODULES: usize = 32;

        let height = self.image.height;
        let width = self.image.width;
        let half_height = height / 2;
        let half_width = width / 2;
        let delta_y = std::cmp::max(1, height / MAX_MODULES * 8) as isize;
        let delta_x = std::cmp::max(1, width / MAX_MODULES * 8) as isize;

        let top = 0;
        let bottom = height;
        let left = 0;
        let right = width;

        let point_a: ResultPoint = self.find_corner_from_center(
            half_width,
            0,
            left,
            right,
            half_height,
            -delta_y,
            top,
            bottom,
            half_width / 2
        )?;

        return Some(vec![point_a]);
    }

    fn find_corner_from_center(
        &self,
        center_x: usize,
        delta_x: isize,
        left: usize,
        right: usize,
        center_y: usize,
        delta_y: isize,
        top: usize,
        bottom: usize,
        max_white_run: usize
    ) -> Option<ResultPoint> {

        return None;
    }

    fn black_white_range(
        &self,
        fixed_dimension: usize, 
        max_white_run: usize, 
        min_dim: usize, 
        max_dim: usize, 
        horizontal: bool
    ) -> Option<Vec<usize>> {
        let center = (min_dim + max_dim) / 2;

        let start = center;
        while start >= min_dim {
            ;
        }
        start += 1;

        let end = center;
        while end < max_dim {
            ;
        }
        end -= 1;

        if end > start {
            return Some(vec![]);
        } else {
            return None;
        }
    }
}