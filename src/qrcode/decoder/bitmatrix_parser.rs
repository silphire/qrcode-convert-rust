use crate::common::bitmatrix::BitMatrix;

pub struct BitMatrixParser {
    pub bitmatrix: BitMatrix,
    // pub version: Version,
    // pub parsedFormatInfo: FormatInformation,
    pub mirror: bool,
}

impl BitMatrixParser {
    pub fn new(bitmatrix: BitMatrix) -> BitMatrixParser {
        return BitMatrixParser {
            bitmatrix: bitmatrix,
            mirror: false,
        }
    }
}