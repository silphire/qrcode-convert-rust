use crate::common::bitmatrix::BitMatrix;
use crate::result_point::ResultPoint;
use crate::result_point::ResultPointTrait;
use crate::common::detector::math_utils;

pub struct WhiteRectangleDetector<'a> {
    image: &'a BitMatrix,
    height: isize,
    width: isize,
    left_init: isize,
    right_init: isize,
    down_init: isize,
    up_init: isize,
}

const INIT_SIZE: isize = 10;
const CORR: f64 = 1.0;

impl<'a> WhiteRectangleDetector<'a> {
    pub fn new(image: &'a BitMatrix, init_size: isize, x: isize, y: isize) -> Option<WhiteRectangleDetector> {
        let half_size = init_size / 2;

        if y < half_size || x < half_size {
            return None;
        }

        let up_init = y - half_size;
        let left_init = x - half_size;
        let down_init = y + half_size;
        let right_init = x + half_size;

        if down_init >= image.get_height() || right_init >= image.get_width() {
            return None;
        }

        return Some(WhiteRectangleDetector {
            image: image,
            height: image.get_height(),
            width: image.get_width(),
            left_init: left_init,
            up_init: up_init,
            right_init: right_init,
            down_init: down_init,
        });
    }

    pub fn detect(&self) -> Vec<ResultPoint> {
        let mut left = self.left_init;
        let mut right = self.right_init;
        let mut up = self.up_init;
        let mut down = self.down_init;
        let mut size_exceeded = false;
        let mut a_black_point_found_on_border = true;

        let mut at_least_one_black_point_found_on_right = false;
        let mut at_least_one_black_point_found_on_bottom = false;
        let mut at_least_one_black_point_found_on_left = false;
        let mut at_least_one_black_point_found_on_top = false;

        while a_black_point_found_on_border {
            let mut right_border_not_white = true;
            while (right_border_not_white || !at_least_one_black_point_found_on_right) && right < self.width {
                right_border_not_white = self.contains_black_point(up, down, right, false);
                if right_border_not_white {
                    right += 1;
                    a_black_point_found_on_border = true;
                    at_least_one_black_point_found_on_right = true;
                } else if !at_least_one_black_point_found_on_right {
                    right += 1;
                }
            }

            if right >= self.width {
                size_exceeded = true;
                break;
            }

            let mut  bottom_border_not_white = true;
            while (bottom_border_not_white || !at_least_one_black_point_found_on_right) && down < self.height {
                bottom_border_not_white = self.contains_black_point(left, right, down, true);
                if bottom_border_not_white {
                    down += 1;
                    a_black_point_found_on_border = true;
                    at_least_one_black_point_found_on_bottom = true;
                } else if !at_least_one_black_point_found_on_bottom {
                    down += 1;
                }
            }

            if down >= self.height {
                size_exceeded = true;
                break;
            }

            let mut left_border_not_white = true;
            while (left_border_not_white || !at_least_one_black_point_found_on_right) && left >= 0 {
                left_border_not_white = self.contains_black_point(up, down, left, true);
                if left_border_not_white {
                    left -= 1;
                    a_black_point_found_on_border = true;
                    at_least_one_black_point_found_on_left = true;
                } else if !at_least_one_black_point_found_on_left {
                    left -= 1;
                }
            }

            if left < 0 {   // TODO for isize
                size_exceeded = true;
                break;
            }

            let mut top_border_not_white = true;
            while (top_border_not_white || !at_least_one_black_point_found_on_top) && up >= 0 {
                top_border_not_white = self.contains_black_point(left, right, up, true);
                if top_border_not_white {
                    up -= 1;
                    a_black_point_found_on_border = true;
                    at_least_one_black_point_found_on_top = true;
                } else if !at_least_one_black_point_found_on_top {
                    up -= 1;
                }
            }

            if up < 0 {   // TODO for isize
                size_exceeded = true;
                break;
            }
        }

        if size_exceeded {
            return vec![];
        }

        let max_size = right - left;
        let i: isize;

        let mut z: Option<ResultPoint> = None;
        for i in 1..max_size {
            z = self.get_black_point_on_segment(
                left as f64,
                (down - i) as f64,
                (left + i) as f64,
                down as f64);
            if z.is_some() {
                break;
            }
        }

        if z.is_none() {
            return vec![]; // TODO Err?
        }

        let mut t : Option<ResultPoint> = None;
        for i in 1..max_size {
            t = self.get_black_point_on_segment(
                left as f64,
                (up + i) as f64,
                (left + i) as f64,
                up as f64);
            if t.is_some() {
                break;
            }
        }

        if t.is_none() {
            return vec![]; // TODO Err?
        }

        let mut x : Option<ResultPoint> = None;
        for i in 1..max_size {
            x = self.get_black_point_on_segment(
                right as f64,
                (up + i) as f64,
                (right - i) as f64,
                up as f64);
            if x.is_some() {
                break;
            }
        }

        if x.is_none() {
            return vec![]; // TODO Err?
        }

        let mut y : Option<ResultPoint> = None;
        for i in 1..max_size {
            y = self.get_black_point_on_segment(
                right as f64,
                (down - i) as f64,
                (right - i) as f64,
                down as f64);
            if y.is_some() {
                break;
            }
        }

        if y.is_none() {
            return vec![]; // TODO Err?
        }

        return self.center_edges(&y.unwrap(), &z.unwrap(), &x.unwrap(), &t.unwrap());
    }

    fn get_black_point_on_segment(&self, ax: f64, ay: f64, bx: f64, by: f64) -> Option<ResultPoint> {
        let dist = math_utils::round(math_utils::distance(ax, ay, bx, by));
        let x_step = (bx - ax) / dist as f64;
        let y_step = (by - ay) / dist as f64;
        for i in 0..dist {
            let x = math_utils::round(ax + (i as f64) * x_step) as isize;
            let y = math_utils::round(ay + (i as f64) * y_step) as isize;
            if self.image.get(x, y) {
                return Some(ResultPoint {x: x as f64, y: y as f64});
            }
        }
        return None;
    }

    fn center_edges(&self, y: &ResultPoint, z: &ResultPoint, x: &ResultPoint, t: &ResultPoint) -> Vec<ResultPoint> {
        let yi = y.get_x();
        let yj = y.get_y();
        let zi = z.get_x();
        let zj = z.get_y();
        let xi = x.get_x();
        let xj = x.get_y();
        let ti = t.get_x();
        let tj = t.get_y();

        if yi < (self.width / 2) as f64 {
            return vec![
                ResultPoint{x: ti - CORR, y: tj + CORR},
                ResultPoint{x: zi + CORR, y: zj + CORR},
                ResultPoint{x: xi - CORR, y: xj - CORR},
                ResultPoint{x: yi + CORR, y: yj - CORR},
            ];
        } else {
            return vec![
                ResultPoint{x: ti + CORR, y: tj + CORR},
                ResultPoint{x: zi + CORR, y: zj - CORR},
                ResultPoint{x: xi - CORR, y: xj + CORR},
                ResultPoint{x: yi - CORR, y: yj - CORR},
            ];
        }
    }

    fn contains_black_point(&self, a: isize, b: isize, fixed: isize, horizontal: bool) -> bool {
        if horizontal {
            for x in a..(b + 1) {
                if self.image.get(x, fixed) {
                    return true;
                }
            }
        } else {
            for y in a..(b + 1) {
                if self.image.get(fixed, y) {
                    return true;
                }
            }
        }

        return false;
    }
}