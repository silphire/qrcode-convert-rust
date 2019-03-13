use crate::common::bitarray::BitArray;

#[derive(Clone)]
pub struct BitMatrix {
    pub width: usize,
    pub height: usize,
    pub row_size: usize,
    pub bits: Vec<i32>,
}

impl BitMatrix {
    pub fn new_with_dimension(dimension: usize) -> BitMatrix {
        return BitMatrix{
            width: 0,
            height: 0,
            row_size: dimension,
            bits: vec![],
        }
    }

    pub fn new(width: usize, height: usize, row_size: usize, bits: Vec<i32>) -> BitMatrix {
        return BitMatrix {
            width: width,
            height: height,
            row_size: row_size,
            bits: bits,
        }
    }

    //pub fn parse_from_bool() -> BitMatrix {
    //}

    //pub fn parse_from_str() -> BitMatrix {
    //}

    pub fn get(&self, x: usize, y: usize) -> bool {
        let offset = y * self.row_size + (x / 32);
        return ((self.bits[offset] >> (x & 0x1f)) & 1) != 0;
    }

    pub fn set(&mut self, x: usize, y: usize) {
        let offset = y * self.row_size + (x / 32);
        self.bits[offset] |= 1 << (x & 0x1f);
    }

    pub fn unset(&mut self, x: usize, y: usize) {
        let offset = y * self.row_size + (x / 32);
        self.bits[offset] &= !(1 << (x & 0x1f));
    }

    pub fn flip(&mut self, x: usize, y: usize) {
        let offset = y * self.row_size + (x / 32);
        self.bits[offset] ^= 1 << (x & 0x1f);
    }

    pub fn xor(&mut self, mask: &BitMatrix) {
        ;
    }

    pub fn clear(&mut self) {
        let max = self.bits.len();
        for i in 0..max {
            self.bits[i] = 0;
        }
    }

    pub fn set_region(&mut self, left: usize, top: usize, width: usize, height: usize) {
        ;
    }

    pub fn get_row(&mut self, y: usize, row: Option<BitArray>) -> BitArray {
        let mut new_row: BitArray;
        if row.is_none() || row.unwrap().get_size() < self.width {
            new_row = BitArray::new_with_size(self.width);
        } else {
            new_row = row.unwrap();
            new_row.clear();
        }
        let offset = y * self.row_size;
        for x in 0..self.row_size {
            new_row.set_bulk(x * 32, self.bits[offset + x]);
        }

        return new_row;
    }

    pub fn set_row(&mut self, y: usize, row: &BitArray) {
        ;
    }

    pub fn rotate_180(&mut self) {
        ;
    }

    pub fn get_enclosing_rectangle(&mut self) {
        ;
    }
}

#[test]
fn create_bitmatrix() {
    let x = BitMatrix::new_with_dimension(100);

    assert_eq!(x.width, 0);
    assert_eq!(x.height, 0);
    assert_eq!(x.row_size, 100);
}
