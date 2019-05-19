use crate::error::Error;

pub struct BitSource<'a> {
    bytes: &'a Vec<u8>,
    byte_offset: isize,
    bit_offset: isize,
}

impl<'a> BitSource<'a> {
    pub fn new(bytes: &'a Vec<u8>) -> BitSource {
        return BitSource {
            bytes: bytes,
            byte_offset: 0,
            bit_offset: 0,
        };
    }

    pub const fn get_byte_offset(&self) -> isize {
        return self.byte_offset;
    }

    pub const fn get_bit_offset(&self) -> isize {
        return self.bit_offset;
    }

    pub fn read_bits(&self, num_bits: isize) -> Result<isize, Error> {
        if num_bits < 1 || num_bits > 32 || num_bits > self.available() {
            return Err(Error::IllegalArgumentError);
        }

        let result: isize = 0;

        if self.bit_offset > 0 {
            let bits_left = 8 - self.bit_offset;
            let to_read = if num_bits < bits_left { num_bits } else { bits_left };
            let bits_to_not_read = bits_left - to_read;
            let mask = (0xff >> (8 - to_read)) << bits_to_not_read;
            
            num_bits -= to_read;
            self.bit_offset += to_read;
            if self.bit_offset == 8 {
                self.bit_offset = 0;
                self.byte_offset += 1;
            }
        }

        if num_bits > 0 {
            while num_bits >= 8 {
                result = (result << 8) | (self.bytes[self.byte_offset as usize] & 0xff) as isize;
                self.byte_offset += 1;
                num_bits -= 8;
            }
        }

        if num_bits > 0 {
            let bits_to_not_read = 8 - num_bits;
            let mask = (0xff >> bits_to_not_read) << bits_to_not_read;
            result = (result << num_bits) | ((self.bytes[self.byte_offset as usize] & mask) >> bits_to_not_read) as isize;
            self.bit_offset += num_bits;
        }

        return Ok(result);
    }

    pub const fn available(&self) -> isize {
        return 8 * (self.bytes.len() as isize - self.byte_offset) - self.bit_offset;
    }
}