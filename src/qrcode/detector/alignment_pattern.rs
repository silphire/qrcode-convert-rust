use crate::result_point::ResultPointTrait;

pub struct AlignmentPattern {
    pub x: f64,
    pub y: f64,
    pub estimated_module_size: f64,
}

impl ResultPointTrait for AlignmentPattern {}

impl AlignmentPattern {
    pub fn about_equals(&self, module_size: f64, i: f64, j: f64) -> bool {
        if (i - self.y).abs() <= module_size && (j - self.x) <= module_size {
            let module_size_diff = (module_size - self.estimated_module_size).abs();
            return module_size_diff <= 1.0 || module_size_diff <= self.estimated_module_size;
        }

        return false;
    }

    pub fn combine_estimate(&self, i: f64, j: f64, new_module_size: f64) -> AlignmentPattern {
        let combined_x = (self.x + j) / 2.0;
        let combined_y = (self.y + i) / 2.0;
        let combined_module_size = (self.estimated_module_size + new_module_size) / 2.0;

        return AlignmentPattern {
            x: combined_x,
            y: combined_y,
            estimated_module_size: combined_module_size,
        }
    }
}