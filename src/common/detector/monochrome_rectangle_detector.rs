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

        let mut top = 0;
        let mut bottom = height;
        let mut left = 0;
        let mut right = width;

        let mut point_a: ResultPoint = self.find_corner_from_center(
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
        top = (point_a.y as usize) - 1;

        let point_b: ResultPoint = self.find_corner_from_center(
            half_width,
            -delta_x,
            left,
            right,
            half_height,
            0,
            top,
            bottom,
            half_height / 2
        )?;
        left = (point_b.x as usize) - 1;

        let point_c: ResultPoint = self.find_corner_from_center(
            half_width,
            delta_x,
            left,
            right,
            half_height,
            0,
            top,
            bottom,
            half_height / 2
        )?;
        right = (point_c.x as usize) + 1;

        let point_d: ResultPoint = self.find_corner_from_center(
            half_width,
            0,
            left,
            right,
            half_height,
            delta_y,
            top,
            bottom,
            half_width / 2
        )?;
        bottom = (point_d.y as usize) + 1;

        point_a = self.find_corner_from_center(
            half_width,
            0,
            left,
            right,
            half_height,
            -delta_y,
            top,
            bottom,
            half_width / 4
        )?;

        return Some(vec![point_a, point_b, point_c, point_d]);
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
            if horizontal && self.image.get(start, fixed_dimension) || !horizontal && self.image.get(fixed_dimension, start) {
                start += 1;
            } else {
                let white_run_start = start;
                start -= 1;
            }
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