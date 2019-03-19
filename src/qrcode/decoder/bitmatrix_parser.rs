use crate::common::bitmatrix::BitMatrix;
use crate::qrcode::decoder::format_information::FormatInformation;

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

    pub fn read_format_information(&self) -> FormatInformation {
        return FormatInformation{};
    }
}