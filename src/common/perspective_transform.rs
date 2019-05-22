pub struct PerspectiveTransform {
    pub a11: f64,
    pub a12: f64,
    pub a13: f64,
    pub a21: f64,
    pub a22: f64,
    pub a23: f64,
    pub a31: f64,
    pub a32: f64,
    pub a33: f64,
}

impl PerspectiveTransform {
    pub fn quadrilateral_to_quadrilateral(
        x0: f64, y0: f64,
        x1: f64, y1: f64,
        x2: f64, y2: f64,
        x3: f64, y3: f64,
        x0p: f64, y0p: f64,
        x1p: f64, y1p: f64,
        x2p: f64, y2p: f64,
        x3p: f64, y3p: f64,
    ) -> PerspectiveTransform {
        let qtos = Self::quadrilateral_to_square(x0, y0, x1, y1, x2, y2, x3, y3);
        let stoq = Self::square_to_quadrilateral(x0p, y0p, x1p, y1p, x2p, y2p, x3p, y3p);
        return stoq.times(&qtos);
    }

    pub fn transform_points(&self, points: &mut [f64]) {
        let max_i = points.len();
        for i in (0..max_i).step_by(2) {
            let x = points[i];
            let y = points[i + 1];
            let denominator = self.a13 * x + self.a23 * y + self.a33;
            points[i] = (self.a11 * x + self.a21 * y + self.a31) / denominator;
            points[i + 1] = (self.a12 * x + self.a22 * y + self.a32) / denominator;
        }
    }

    // Is transform_points(&self, x_values: &[f64], y_values: &[f64]) needed?

    pub fn square_to_quadrilateral(x0: f64, y0: f64, x1: f64, y1: f64, x2: f64, y2: f64, x3: f64, y3: f64) -> PerspectiveTransform {
        let dx3 = x0 - x1 + x2 - x3;
        let dy3 = y0 - y1 + y2 - y3;
        if dx3 == 0.0 && dy3 == 0.0 {
            return PerspectiveTransform {
                a11: x1 - x0,
                a12: x2 - x1,
                a13: x0,
                a21: y1 - y0,
                a22: y2 - y1,
                a23: y0,
                a31: 0.0,
                a32: 0.0,
                a33: 1.0,
            };
        } else {
            let dx1 = x1 - x2;
            let dx2 = x3 - x2;
            let dy1 = y1 - y2;
            let dy2 = y3 - y2;
            let denominator = dx1 * dy2 - dx2 * dy1;
            let a13 = (dx3 * dy2 - dx2 * dy3) / denominator;
            let a23 = (dx1 * dy3 - dx3 * dy1) / denominator;
            return PerspectiveTransform {
                a11: x1 - x0 + a13 * x1,
                a12: x3 - x0 + a23 * x3,
                a13: x0,
                a21: y1 - y0 + a13 * y1,
                a22: y3 - y0 + a23 * y3,
                a23: y0,
                a31: a13,
                a32: a23,
                a33: 1.0,
            }
        }
    }

    pub fn quadrilateral_to_square(x0: f64, y0: f64, x1: f64, y1: f64, x2: f64, y2: f64, x3: f64, y3: f64) -> PerspectiveTransform {
        return Self::square_to_quadrilateral(x0, y0, x1, y1, x2, y2, x3, y3).build_adjoint();
    }

    pub fn build_adjoint(&self) -> PerspectiveTransform {
        return PerspectiveTransform {
            a11: self.a22 * self.a33 - self.a23 * self.a32,
            a21: self.a23 * self.a31 - self.a21 * self.a33,
            a31: self.a21 * self.a32 - self.a22 * self.a31,
            a12: self.a13 * self.a32 - self.a12 * self.a33,
            a22: self.a11 * self.a33 - self.a13 * self.a31,
            a32: self.a12 * self.a31 - self.a11 * self.a32,
            a13: self.a12 * self.a23 - self.a13 * self.a22,
            a23: self.a13 * self.a21 - self.a11 * self.a23,
            a33: self.a11 * self.a22 - self.a12 * self.a21,
        };
    }

    pub fn times(&self, other: &PerspectiveTransform) -> PerspectiveTransform {
        return PerspectiveTransform {
            a11: self.a11 * other.a11 + self.a21 * other.a12 + self.a31 * other.a13,
            a21: self.a11 * other.a21 + self.a21 * other.a22 + self.a31 * other.a23,
            a31: self.a11 * other.a31 + self.a21 * other.a32 + self.a31 * other.a33,
            a12: self.a12 * other.a11 + self.a22 * other.a12 + self.a32 * other.a13,
            a22: self.a12 * other.a21 + self.a22 * other.a22 + self.a32 * other.a23,
            a32: self.a12 * other.a31 + self.a22 * other.a32 + self.a32 * other.a33,
            a13: self.a13 * other.a11 + self.a23 * other.a12 + self.a33 * other.a13,
            a23: self.a13 * other.a21 + self.a23 * other.a22 + self.a33 * other.a23,
            a33: self.a13 * other.a31 + self.a23 * other.a32 + self.a33 * other.a33,
        };
    }
}