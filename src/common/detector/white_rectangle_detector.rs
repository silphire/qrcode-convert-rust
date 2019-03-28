use crate::common::bitmatrix::BitMatrix;
use crate::result_point::ResultPoint;
use crate::common::detector::math_utils;

pub struct WhiteRectangleDetector {
    image: BitMatrix,
    height: usize,
    width: usize,
    left_init: usize,
    right_init: usize,
    down_init: usize,
    up_init: usize,
}

const INIT_SIZE: usize = 10;
const CORR: f64 = 1.0;

impl WhiteRectangleDetector {
    pub fn new(image: BitMatrix, init_size: usize, x: usize, y: usize) -> Option<WhiteRectangleDetector> {
        let half_size = init_size / 2;

        if y < half_size || x < half_size {
            return None;
        }

        let up_init = y - half_size;
        let left_init = x - half_size;
        let down_init = y + half_size;
        let right_init = x + half_size;

        if down_init >= image.height || right_init >= image.width {
            return None;
        }

        return Some(WhiteRectangleDetector {
            image: image,
            height: image.height,
            width: image.width,
            left_init: left_init,
            up_init: up_init,
            right_init: right_init,
            down_init: down_init,
        });
    }
    pub fn detect(&self) -> &Vec<ResultPoint> {
        let left = self.left_init;
        let right = self.right_init;
        let up = self.up_init;
        let down = self.down_init;
        let size_exceeded = false;
        let a_black_point_found_on_border = true;

        let at_least_one_black_point_found_on_right = false;
        let at_least_one_black_point_found_on_bottom = false;
        let at_least_one_black_point_found_on_left = false;
        let at_least_one_black_point_found_on_top = false;

        while a_black_point_found_on_border {
            let right_border_not_white = true;
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

            let bottom_border_not_white = true;
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

            let left_border_not_white = true;
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

            if left < 0 {   // TODO for usize
                size_exceeded = true;
                break;
            }

            let top_border_not_white = true;
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

            if up < 0 {   // TODO for usize
                size_exceeded = true;
                break;
            }
        }

        if size_exceeded {
            return &vec![];
        }

        let max_size = right - left;
        let i: usize;

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
            return &vec![]; // TODO Err?
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
            return &vec![]; // TODO Err?
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
            return &vec![]; // TODO Err?
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
            return &vec![]; // TODO Err?
        }

        return self.center_edges(&y.unwrap(), &z.unwrap(), &x.unwrap(), &t.unwrap());
    }

    fn get_black_point_on_segment(&self, ax: f64, ay: f64, bx: f64, by: f64) -> Option<ResultPoint> {
        let dist = math_utils::round(math_utils::distance(ax, ay, bx, by));
        let x_step = (bx - ax) / dist as f64;
        let y_step = (by - ay) / dist as f64;
        for i in 0..dist {
            let x = math_utils::round(ax + (i as f64) * x_step) as usize;
            let y = math_utils::round(ay + (i as f64) * y_step) as usize;
            if self.image.get(x, y) {
                return Some(ResultPoint {x: x as f64, y: y as f64});
            }
        }
        return None;
    }

    fn center_edges(&self, y: &ResultPoint, z: &ResultPoint, x: &ResultPoint, t: &ResultPoint) -> &Vec<ResultPoint> {
        let yi = y.x;
        let yj = y.y;
        let zi = z.x;
        let zj = z.y;
        let xi = x.x;
        let xj = x.y;
        let ti = t.x;
        let tj = t.y;

        if yi < (self.width / 2) as f64 {
            return &vec![
                ResultPoint{x: ti - CORR, y: tj + CORR},
                ResultPoint{x: zi + CORR, y: zj + CORR},
                ResultPoint{x: xi - CORR, y: xj - CORR},
                ResultPoint{x: yi + CORR, y: yj - CORR},
            ];
        } else {
            return &vec![
                ResultPoint{x: ti + CORR, y: tj + CORR},
                ResultPoint{x: zi + CORR, y: zj - CORR},
                ResultPoint{x: xi - CORR, y: xj + CORR},
                ResultPoint{x: yi - CORR, y: yj - CORR},
            ];
        }
    }

    fn contains_black_point(&self, a: usize, b: usize, fixed: usize, horizontal: bool) -> bool {
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