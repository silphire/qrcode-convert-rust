use crate::common::detector::math_utils;

#[derive(Copy, Clone, Debug)]
pub struct ResultPoint {
    x: f64,
    y: f64,
}

pub trait ResultPointTrait {
    fn get_x(&self) -> f64;
    fn set_x(&mut self, x: f64);
    fn get_y(&self) -> f64;
    fn set_y(&mut self, y: f64);

    fn order_best_patterns(patterns: &mut Vec<&ResultPointTrait>)  where Self: Sized {
        let zero_one_distance = Self::distance(patterns[0], patterns[1]);
        let one_two_distance = Self::distance(patterns[1], patterns[2]);
        let zero_two_distance = Self::distance(patterns[0], patterns[2]);

        let mut point_a: &ResultPointTrait;
        let point_b: &ResultPointTrait;
        let mut point_c: &ResultPointTrait;

        if one_two_distance >= zero_one_distance && one_two_distance >= zero_two_distance {
            point_b = patterns[0];
            point_a = patterns[1];
            point_c = patterns[2];
        } else if zero_two_distance >= one_two_distance && zero_two_distance >= zero_one_distance {
            point_b = patterns[1];
            point_a = patterns[0];
            point_c = patterns[2];
        } else {
            point_b = patterns[2];
            point_a = patterns[0];
            point_c = patterns[1];
        }

        if Self::cross_product_z(point_a, point_b, point_c) < 0.0 {
            let temp = point_a;
            point_a = point_c;
            point_c = temp;
        }

        patterns[0] = point_a;
        patterns[1] = point_b;
        patterns[2] = point_c;
    }

    fn distance(pattern1: &ResultPointTrait, pattern2: &ResultPointTrait) -> f64 where Self: Sized {
        return math_utils::distance(pattern1.get_x(), pattern1.get_y(), pattern2.get_x(), pattern2.get_y());
    }

    fn cross_product_z(point_a: &ResultPointTrait, point_b: &ResultPointTrait, point_c: &ResultPointTrait) -> f64 where Self: Sized {
        let bx = point_b.get_x();
        let by = point_b.get_y();

        return ((point_c.get_x() - bx) * (point_a.get_y() - by)) - ((point_c.get_y() - by) * (point_a.get_x() - bx));
    }
}

impl ResultPointTrait for ResultPoint {
    fn get_x(&self) -> f64 {
        return self.x;
    }

    fn set_x(&mut self, x: f64) {
        self.x = x;
    }

    fn get_y(&self) -> f64 {
        return self.y;
    }

    fn set_y(&mut self, y: f64) {
        self.y = y;
    }
}