pub fn round(d: f64) -> isize {
    if d < 0.0 {
        return (d - 0.5) as isize;
    } else {
        return (d + 0.5) as isize;
    }
}

pub fn distance(ax: f64, ay: f64, bx: f64, by: f64) -> f64 {
    let x_diff = ax - bx;
    let y_diff = ay - by;
    return (x_diff * x_diff + y_diff * y_diff).sqrt();
}

