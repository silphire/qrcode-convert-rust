use crate::common::reedsolomon::generic_gf_poly::GenericGFPoly;
use crate::common::reedsolomon::reedsolomon_error::ReedSolomonError;

pub struct ReedSolomonDecoder {
    generic_gf: u8, // TODO GenericGF
}

impl ReedSolomonDecoder {
    pub fn new() -> ReedSolomonDecoder {
        return ReedSolomonDecoder {
            generic_gf: 0,
        }
    }

    pub fn decode(&mut self, received: &Vec<i32>, two_s: isize) {
        // TODO implement
    }

    fn run_eclidean_algorithm(&self, a: &GenericGFPoly, b: &GenericGFPoly, r: isize) -> Result<Vec<GenericGFPoly>, ReedSolomonError> {
        unimplemented!();
    }

    fn find_error_locations(error_locator: &GenericGFPoly) -> Result<Vec<isize>, ReedSolomonError> {
        unimplemented!();
    }

    fn find_error_magnitudes(error_evaluator: &GenericGFPoly, error_locations: &Vec<isize>) -> Vec<isize> {
        unimplemented!();
    }
}