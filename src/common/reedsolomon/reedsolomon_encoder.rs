use crate::common::reedsolomon::generic_gf::GenericGF;
use crate::common::reedsolomon::generic_gf_poly::GenericGFPoly;


pub struct ReedSolomonEncoder {
    field: GenericGF,
    cached_generators: Vec<GenericGFPoly>,
}

impl ReedSolomonEncoder {
    fn build_generator(&self, degree: isize) -> GenericGFPoly {
        unimplemented!();
    }

    pub fn encode(&self, to_encode: &Vec<isize>, ec_bytes: isize) {
        unimplemented!();
    }
}