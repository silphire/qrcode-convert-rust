use crate::common::reedsolomon::generic_gf_poly::GenericGFPoly;
use crate::error::Error;

lazy_static! {
    pub static ref AZTEC_DATA_12: GenericGF = GenericGF::new(0x1069, 4096, 1);
    pub static ref AZTEC_DATA_10: GenericGF = GenericGF::new(0x409,  1024, 1);
    pub static ref AZTEC_DATA_6:  GenericGF = GenericGF::new(0x43,   64, 1);
    pub static ref AZTEC_DATA_PARAM: GenericGF = GenericGF::new(0x13, 16, 1);
    pub static ref QR_CODE_FIELD_256: GenericGF = GenericGF::new(0x011D, 256, 0);
    pub static ref DATA_MATRIX_FIELD_256: GenericGF = GenericGF::new(0x012D, 256, 1);
}

pub struct GenericGF {
    exp_table: Vec<isize>,
    log_table: Vec<isize>,
    zero: Box<GenericGFPoly>,
    one: Box<GenericGFPoly>,
    size: isize,
    primitive: isize,
    generator_base: isize,
}

impl GenericGF {

    pub fn new(primitive: isize, size: isize, b: isize) -> GenericGF {
        unimplemented!();
    }

    pub fn add_or_subtract(a: isize, b: isize) -> isize {
        return a ^ b;
    }

    pub fn exp(&self, a: isize) -> isize {
        return self.exp_table[a as usize];
    }

    pub fn log(&self, a: isize) -> Result<isize, Error> {
        if a == 0 {
            return Err(Error::IllegalArgumentError);
        }
        return Ok(self.log_table[a as usize]);
    }

    pub fn inverse(&self, a: isize) -> Result<isize, Error> {
        if a == 0 {
            return Err(Error::ArithmeticError);
        }
        return Ok(self.exp_table[(self.size - self.log_table[a as usize] - 1) as usize]);
    }

    pub fn multiply(&self, a: isize, b: isize) -> isize {
        if a == 0 || b == 0 {
            return 0;
        }

        return self.exp_table[((self.log_table[a as usize] + self.log_table[b as usize]) % (self.size - 1)) as usize];
    }

    pub const fn get_size(&self) -> isize {
        return self.size;
    }
}