use crate::common::bitmatrix::BitMatrix;

pub enum DataMask {
    DataMask000,
    DataMask001,
    DataMask010,
    DataMask011,
    DataMask100,
    DataMask101,
    DataMask110,
    DataMask111,
}

impl DataMask {
    pub fn unmask_bit_matrix(&self, bits: &mut BitMatrix, dimension: isize) {
        for i in 0..dimension {
            for j in 0..dimension {
                if self.is_masked(i, j) {
                    bits.flip(j, i);
                }
            }
        }
    }

    pub fn is_masked(&self, i: isize, j: isize) -> bool {
        unimplemented!();
    }
}