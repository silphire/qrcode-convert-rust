use crate::error::Error;
use crate::common::reedsolomon::generic_gf::GenericGF;

pub struct GenericGFPoly {
    field: GenericGF,
    coefficients: Vec<isize>,
}

impl GenericGFPoly {
    pub fn new(field: GenericGF, coefficients: Vec<isize>) -> Result<GenericGFPoly, Error> {
        let mut coefficients = coefficients;
        let coefficients_length = coefficients.len();

        if coefficients_length == 0 {
            return Err(Error::IllegalArgumentError);
        }

        if coefficients_length > 1 && coefficients[0] == 0 {
            let mut first_non_zero = 1;
            while first_non_zero < coefficients_length && coefficients[first_non_zero] == 0 {
                first_non_zero += 1;
            }

            if first_non_zero == coefficients_length {
                coefficients = vec![0];
            } else {
                coefficients = vec![0; coefficients_length - first_non_zero];
            }
            // TODO arraycopy
        }

        return Ok(GenericGFPoly {
            field: field,
            coefficients: coefficients,
        });
    }

    pub fn get_degree(&self) -> isize {
        return self.coefficients.len() as isize - 1;
    }

    pub fn get_coefficient(&self, degree: isize) -> isize {
        return self.coefficients[self.coefficients.len() - 1 - degree as usize];
    }
}