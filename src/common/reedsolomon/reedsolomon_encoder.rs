use crate::common::reedsolomon::generic_gf::GenericGF;
use crate::common::reedsolomon::generic_gf_poly::GenericGFPoly;
use crate::error::Error;


pub struct ReedSolomonEncoder {
    field: GenericGF,
    cached_generators: Vec<GenericGFPoly>,
}

impl ReedSolomonEncoder {
    fn build_generator(&self, degree: isize) -> GenericGFPoly {
        unimplemented!();
    }

    pub fn encode(&self, to_encode: &Vec<isize>, ec_bytes: isize) -> Result<(), Error> {
        if ec_bytes == 0 {
            return Err(Error::IllegalArgumentError);
        }

        if to_encode.len() < ec_bytes as usize {
            return Err(Error::IllegalArgumentError);
        }

        return Ok(());
    }
}