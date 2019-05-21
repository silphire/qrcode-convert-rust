use crate::common::bitmatrix::BitMatrix;

pub enum DataMask {
    DATA_MASK_000,
    DATA_MASK_001,
    DATA_MASK_010,
    DATA_MASK_011,
    DATA_MASK_100,
    DATA_MASK_101,
    DATA_MASK_110,
    DATA_MASK_111,
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