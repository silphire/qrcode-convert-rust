use crate::common::reedsolomon::generic_gf::GenericGF;

pub struct GenericGFPoly {
    field: GenericGF,
    coefficients: Vec<isize>,
}

impl GenericGFPoly {
    pub fn get_degree(&self) -> isize {
        return self.coefficients.len() as isize - 1;
    }

    pub fn get_coefficient(&self, degree: isize) -> isize {
        return self.coefficients[self.coefficients.len() - 1 - degree as usize];
    }
}