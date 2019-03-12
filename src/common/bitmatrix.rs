#[derive(Clone)]
pub struct BitMatrix {
    pub width: i32,
    pub height: i32,
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

    pub fn new(width: i32, height: i32, row_size: usize, bits: Vec<i32>) -> BitMatrix {
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
}

#[test]
fn create_bitmatrix() {
    let x = BitMatrix::new_with_dimension(100);

    assert_eq!(x.width, 0);
    assert_eq!(x.height, 0);
    assert_eq!(x.row_size, 100);
}
