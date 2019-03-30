use crate::common::bitmatrix::BitMatrix;

pub struct BinaryBitmap {
    // pub binarizer: Binarizer,
    pub matrix: BitMatrix,
}

impl BinaryBitmap {
    pub fn get_width(&self) -> isize {
        return 0;
        // TODO implement
    }

    pub fn get_black_matrix(&self) -> &BitMatrix {
        return &self.matrix;
        // TODO implement
    }
}