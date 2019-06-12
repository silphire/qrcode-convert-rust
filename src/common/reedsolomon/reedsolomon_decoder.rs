use crate::common::reedsolomon::generic_gf::GenericGF;
use crate::common::reedsolomon::generic_gf_poly::GenericGFPoly;
use crate::common::reedsolomon::reedsolomon_error::ReedSolomonError;

pub struct ReedSolomonDecoder {
    field: GenericGF,
}

impl ReedSolomonDecoder {
    pub fn new(field: GenericGF) -> ReedSolomonDecoder {
        return ReedSolomonDecoder {
            field: field,
        }
    }

    pub fn decode(&mut self, received: &Vec<i32>, two_s: isize) {
        // TODO implement
    }

    fn run_eclidean_algorithm(&self, a: &GenericGFPoly, b: &GenericGFPoly, r: isize) -> Result<Vec<GenericGFPoly>, ReedSolomonError> {
        unimplemented!();
    }

    fn find_error_locations(&self, error_locator: &GenericGFPoly) -> Result<Vec<isize>, ReedSolomonError> {
        let num_errors = error_locator.get_degree();
        if num_errors == 1 {
            return Ok(vec![error_locator.get_coefficient(1)]);
        }

        let mut result = vec![0; num_errors as usize];
        let mut e = 0;

        for i in 1..self.field.get_size() {
            if e >= num_errors {
                break;
            }
            if let Ok(r) = self.field.inverse(i) {
                result[e as usize] = r;
            } else {
                return Err(ReedSolomonError::ReedSolomonError);
            }
            e += 1;
        }

        if e != num_errors {
            return Err(ReedSolomonError::ReedSolomonError);
        }

        return Ok(result);
    }

    fn find_error_magnitudes(error_evaluator: &GenericGFPoly, error_locations: &Vec<isize>) -> Vec<isize> {
        unimplemented!();
    }
}