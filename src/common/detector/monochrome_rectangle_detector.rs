use crate::common::bitmatrix::BitMatrix;
use crate::result_point::ResultPoint;
use crate::result_point::ResultPointTrait;

pub struct MonochromeRectangleDetector {
    image: BitMatrix,
}

impl MonochromeRectangleDetector {
    pub fn detect(&self) -> Option<Vec<&ResultPointTrait>> {
        const MAX_MODULES: isize = 32;

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

        let mut point_a = self.find_corner_from_center(
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
        top = (point_a.get_y() as isize) - 1;

        let point_b = self.find_corner_from_center(
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
        left = (point_b.get_x() as isize) - 1;

        let point_c = self.find_corner_from_center(
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
        right = (point_c.get_x() as isize) + 1;

        let point_d = self.find_corner_from_center(
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
        bottom = (point_d.get_y() as isize) + 1;

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
        center_x: isize,
        delta_x: isize,
        left: isize,
        right: isize,
        center_y: isize,
        delta_y: isize,
        top: isize,
        bottom: isize,
        max_white_run: isize
    ) -> Option<&ResultPointTrait> {
        let mut last_range: Option<Vec<isize>>;
        let y = center_y;
        let x = center_x;
        while y < bottom && y >= top && x < right && x >= left {
            let range = if delta_x == 0 {
                self.black_white_range(y, max_white_run, left, right, true)
            } else {
                self.black_white_range(x, max_white_run, top, bottom, false)
            };

            if range.is_none() {
                if last_range.is_none() {
                    return None;    // NotFound
                }

                if delta_x == 0 {
                    let last_y = y - delta_y;
                    if last_range.unwrap()[0] < center_x {
                        if last_range.unwrap()[1] > center_x {
                            return Some(&ResultPoint {
                                x: last_range.unwrap()[(if delta_y > 0 {0} else {1})] as f64,
                                y: last_y as f64,
                            });
                        }
                    }
                }
            }

            last_range = range;
            y += delta_y;
            x += delta_x;
        }

        return None;
    }

    fn black_white_range(
        &self,
        fixed_dimension: isize, 
        max_white_run: isize, 
        min_dim: isize, 
        max_dim: isize, 
        horizontal: bool
    ) -> Option<Vec<isize>> {
        let center = (min_dim + max_dim) / 2;

        let start = center;
        while start >= min_dim {
            if if horizontal { self.image.get(start, fixed_dimension) } else { self.image.get(fixed_dimension, start) } {
                start += 1;
            } else {
                let white_run_start = start;
                start -= 1;
                while start >= min_dim && !if horizontal { self.image.get(start, fixed_dimension) } else { self.image.get(fixed_dimension, start)} {
                    start -= 1;
                }

                let white_run_size = white_run_start - start;
                if start < min_dim || white_run_size > max_white_run {
                    start = white_run_start;
                    break;
                }
            }
        }
        start += 1;

        let end = center;
        while end < max_dim {
            if if horizontal { self.image.get(end, fixed_dimension)}  else {self.image.get(fixed_dimension, end)} {
                end += 1;
            } else {
                let white_run_start = end;
                end += 1;
                while end < max_dim && !if horizontal { self.image.get(end, fixed_dimension)} else {self.image.get(fixed_dimension, end)} {
                    end += 1;
                }

                let white_run_size = end - white_run_start;
                if end >= max_dim || white_run_size > max_white_run {
                    end = white_run_start;
                    break;
                }
            }
        }
        end -= 1;

        if end > start {
            return Some(vec![start, end]);
        } else {
            return None;
        }
    }
}