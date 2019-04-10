use crate::result_point::ResultPointTrait;

pub struct FinderPattern {
    x: f64, 
    y: f64,
    estimated_module_size: f64,
    count: isize,
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

impl FinderPattern {
    pub fn new(pos_x: f64, pos_y: f64, estimated_module_size: f64, count: isize) -> FinderPattern {
        return FinderPattern {
            x: pos_x,
            y: pos_y,
            estimated_module_size: estimated_module_size,
            count: count,
        }
    }

    pub const fn get_estimated_module_size(&self) -> f64 {
        return self.estimated_module_size;
    }

    pub const fn get_count(&self) -> isize {
        return self.count;
    }

    pub fn about_equals(&self, module_size: f64, i: f64, j: f64) -> bool {
        if (i - self.get_y()).abs() <= module_size && (j - self.get_x()).abs() <= module_size {
            let module_size_diff = (module_size - self.get_estimated_module_size()).abs();
            return module_size_diff <= 1.0 || module_size_diff <= self.get_estimated_module_size();
        }
        return false;
    }

    pub fn combine_estimate(&self, i: f64, j: f64, new_module_size: f64) -> FinderPattern {
        let combined_count = self.get_count() + 1;
        let combined_x = (self.get_count() as f64 * self.get_x() + j) / combined_count as f64;
        let combined_y = (self.get_count() as f64 * self.get_y() + i) / combined_count as f64;
        let combined_module_size = (self.get_count() as f64 * self.get_estimated_module_size() + new_module_size) / combined_count as f64;
        return FinderPattern::new(combined_x, combined_y, combined_module_size, combined_count);
    }
}