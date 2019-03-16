use crate::common::detector::math_utils;

pub struct ResultPoint {
    x: f64,
    y: f64,
}

impl ResultPoint {
    pub fn order_best_patterns(patterns: &Vec<ResultPoint>) {
        let zero_one_distance = ResultPoint::distance(&patterns[0], &patterns[1]);
        let one_two_distance = ResultPoint::distance(&patterns[1], &patterns[2]);
        let zero_two_distance = ResultPoint::distance(&patterns[0], &patterns[2]);

        let mut point_a: &ResultPoint;
        let mut point_b: &ResultPoint;
        let mut point_c: &ResultPoint;

        if one_two_distance >= zero_one_distance && one_two_distance >= zero_two_distance {
            point_b = &patterns[0];
            point_a = &patterns[1];
            point_c = &patterns[2];
        } else if zero_two_distance >= one_two_distance && zero_two_distance >= zero_one_distance {
            point_b = &patterns[1];
            point_a = &patterns[0];
            point_c = &patterns[2];
        } else {
            point_b = &patterns[2];
            point_a = &patterns[0];
            point_c = &patterns[1];
        }

        if ResultPoint::cross_product_z(point_a, point_b, point_c) < 0.0 {
            let temp = point_a;
            point_a = point_c;
            point_c = temp;
        }

        patterns[0] = *point_a;
        patterns[1] = *point_b;
        patterns[2] = *point_c;
    }

    pub fn distance(pattern1: &ResultPoint, pattern2: &ResultPoint) -> f64 {
        return math_utils::distance(pattern1.x, pattern1.y, pattern2.x, pattern2.y);
    }

    pub fn cross_product_z(point_a: &ResultPoint, point_b: &ResultPoint, point_c: &ResultPoint) -> f64 {
        let bx = point_b.x;
        let by = point_b.y;

        return ((point_c.x - bx) * (point_a.y - by)) - ((point_c.y - by) * (point_a.x - bx));
    }
}