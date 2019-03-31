use crate::result_point::ResultPointTrait;

pub struct FinderPattern {
    x: f64, 
    y: f64,
}

impl ResultPointTrait for FinderPattern {
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
