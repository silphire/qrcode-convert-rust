use crate::common::bitarray::BitArray;
use crate::error::Error;

#[derive(Clone)]
pub struct BitMatrix {
    pub width: isize,
    pub height: isize,
    pub row_size: isize,
    pub bits: Vec<i32>,
}

impl BitMatrix {
    pub fn new_with_dimension(dimension: isize) -> BitMatrix {
        return BitMatrix{
            width: 0,
            height: 0,
            row_size: dimension,
            bits: vec![],
        }
    }

    pub fn new(width: isize, height: isize) -> BitMatrix {
        let row_size = (width + 31) / 32;
        return BitMatrix::new_with_bits(width, height, row_size, Vec::<i32>::with_capacity((row_size * height) as usize))
    }

    fn new_with_bits(width: isize, height: isize, row_size: isize, bits: Vec<i32>) -> BitMatrix {
        return BitMatrix {
            width: width,
            height: height,
            row_size: row_size,
            bits: bits,
        }
    }

    pub fn parse_from_bool(image: &[&[bool]]) -> BitMatrix {
        let height = image.len() as isize;
        let width = image[0].len() as isize;

        let bits = BitMatrix::new(width, height);
        for i in 0..height {
            let image_i = image[i as usize];
            for j in 0..width {
                if image_i[j as usize] {
                    bits.set(j, i);
                }
            }
        }

        return bits;
    }

    pub fn parse_from_str(string_representation: &str, set_string: &str, unset_string: &str) -> BitMatrix {
        unimplemented!();
    }

    pub const fn get(&self, x: isize, y: isize) -> bool {
        let offset = y * self.row_size + (x / 32);
        return ((self.bits[offset as usize] >> (x & 0x1f)) & 1) != 0;
    }

    pub fn set(&mut self, x: isize, y: isize) {
        let offset = y * self.row_size + (x / 32);
        self.bits[offset as usize] |= 1 << (x & 0x1f);
    }

    pub fn unset(&mut self, x: isize, y: isize) {
        let offset = y * self.row_size + (x / 32);
        self.bits[offset as usize] &= !(1 << (x & 0x1f));
    }

    pub fn flip(&mut self, x: isize, y: isize) {
        let offset = y * self.row_size + (x / 32);
        self.bits[offset as usize] ^= 1 << (x & 0x1f);
    }

    pub fn xor(&mut self, mask: &BitMatrix) -> Result<(), Error> {
        if self.width != mask.width || self.height != mask.height || self.row_size != mask.row_size {
            return Err(Error::IllegalArgumentError);
        }

        let mut row_array = BitArray::new_with_size(self.width);
        for y in 0..self.height {
            let offset = y * self.row_size;
            let row = mask.get_row(y, Some(Box::new(row_array))).get_bit_array();
            for x in 0..self.row_size {
                self.bits[(offset + x) as usize] ^= row[x as usize];
            }
        }

        return Ok(())
    }

    pub fn clear(&mut self) {
        let max = self.bits.len();
        for i in 0..max {
            self.bits[i] = 0;
        }
    }

    pub fn set_region(&mut self, left: isize, top: isize, width: isize, height: isize) -> Result<(), Error> {
        if height < 1 || width < 1 {
            return Err(Error::IllegalArgumentError);
        }

        let right = left + width;
        let bottom = top + height;
        if bottom > self.height || right > self.width {
            return Err(Error::IllegalArgumentError);
        }
        for y in top..bottom {
            let offset = y * self.row_size;
            for x in left..right {
                self.bits[(offset + (x / 32)) as usize] |= 1 << (x & 0x1f);
            }
        }

        return Ok(());
    }

    pub fn get_row(&self, y: isize, row: Option<Box<BitArray>>) -> Box<BitArray> {

        let new_row: Box<BitArray>;
        if row.is_none() || row.unwrap().get_size() < self.width {
            new_row = Box::new(BitArray::new_with_size(self.width));
        } else {
            new_row = row.unwrap();
            new_row.clear();
        }

        let offset = y * self.row_size;
        for x in 0..self.row_size {
            new_row.set_bulk(x * 32, self.bits[(offset + x) as usize]);
        }

        return new_row;
    }

    pub fn set_row(&mut self, y: isize, row: &BitArray) {
        unimplemented!();
    }

    pub fn rotate_180(&mut self) {
        let mut top_row = Box::new(BitArray::new_with_size(self.width));
        let mut bottom_row = Box::new(BitArray::new_with_size(self.width));
        for i in 0..((self.height + 1) / 2) {
            top_row = self.get_row(i, Some(top_row));
            bottom_row = self.get_row(self.height - 1 - i, Some(bottom_row));
            top_row.reverse();
            bottom_row.reverse();
            self.set_row(i, &bottom_row);
            self.set_row(self.height - 1 - i, &top_row);
        }
    }

    pub fn get_enclosing_rectangle(&self) -> Vec<isize> {
        let mut left = self.width;
        let mut top = self.height;
        let mut right = 0;
        let mut bottom = 0;

        for y in 0..self.height {
            for x32 in 0..self.row_size {
                let the_bits = self.bits[(y * self.row_size + x32) as usize];
                if the_bits != 0 {
                    if y < top {
                        top = y;
                    }
                    if y > bottom {
                        bottom = y;
                    }
                    if x32 * 32 < left {
                        let mut bit = 0;
                        while (the_bits << (31 - bit)) == 0 {
                            bit += 1;
                        }
                        if (x32 * 32 + bit) < left {
                            left = x32 * 32 + bit;
                        }
                    }

                    if x32 * 32 + 31 > right {
                        let mut bit = 31;
                        while (the_bits >> bit) == 0 {
                            bit -= 1;
                        }
                        if (x32 * 32 + bit) > right {
                            right = x32 * 32 + bit;
                        }
                    }
                }
            }
        }

        if right < left || bottom < top {
            return vec![];
        }

        return vec![left, top, right - left + 1, bottom - top + 1];
    }

    pub fn get_top_left_on_bit(&self) -> Vec<isize> {
        let mut bits_offset = 0;
        
        while bits_offset < self.bits.len() && self.bits[bits_offset] == 0 {
            bits_offset += 1;
        }
        if bits_offset == self.bits.len() {
            return vec![];
        }

        let y = bits_offset as isize / self.row_size;
        let mut x = (bits_offset as isize % self.row_size) * 32;

        let the_bits = self.bits[bits_offset];
        let mut bit = 0;
        while (the_bits << (31 - bit)) == 0 {
            bit += 1;
        }
        x += bit;

        return vec![x, y];
    }

    pub fn get_bottom_right_on_bit(&self) -> Vec<isize> {
        let mut bits_offset = self.bits.len() - 1;
        while bits_offset >= 0 && self.bits[bits_offset] == 0 {
            if bits_offset == 0 {
                return vec![];
            }

            bits_offset -= 1;
        }

        let y = bits_offset as isize / self.row_size;
        let mut x = bits_offset as isize % self.row_size * 32;

        let the_bits = self.bits[bits_offset];
        let mut bit = 31;
        while the_bits >> bit == 0 {
            bit -= 1;
        }
        x += bit;

        return vec![x, y];
    }

    pub const fn get_width(&self) -> isize {
        return self.width;
    }

    pub const fn get_height(&self) -> isize {
        return self.height;
    }

    pub const fn get_rowsize(&self) -> isize {
        return self.row_size;
    }
}

#[test]
fn create_bitmatrix() {
    let x = BitMatrix::new_with_dimension(100);

    assert_eq!(x.width, 0);
    assert_eq!(x.height, 0);
    assert_eq!(x.row_size, 100);
}
