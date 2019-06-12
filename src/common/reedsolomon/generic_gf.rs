use crate::common::reedsolomon::generic_gf_poly::GenericGFPoly;
use crate::error::Error;

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
    pub fn add_or_subtract(a: isize, b: isize) -> isize {
        return a ^ b;
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