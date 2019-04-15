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
    pub const fn times(&self, other: &PerspectiveTransform) -> PerspectiveTransform {
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