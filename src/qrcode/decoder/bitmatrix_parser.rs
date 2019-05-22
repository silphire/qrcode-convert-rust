use crate::common::bitmatrix::BitMatrix;
use crate::qrcode::decoder::format_information::FormatInformation;

pub struct BitMatrixParser {
    pub bitmatrix: BitMatrix,
    // pub version: Version,
    pub parsed_format_info: Option<FormatInformation>,
    pub mirror: bool,
}

impl BitMatrixParser {
    pub fn new(bitmatrix: BitMatrix) -> BitMatrixParser {
        return BitMatrixParser {
            bitmatrix: bitmatrix,
            parsed_format_info: None,
            mirror: false,
        }
    }

    pub fn read_format_information(&self) -> &FormatInformation {
        if let Some(parsed_format_info) =  &self.parsed_format_info {
            return &parsed_format_info;
        }

        let mut format_info_bits_1 = 0;
        for i in 1..6 {
            format_info_bits_1 = self.copy_bit(i, 8, format_info_bits_1);
        }

        unimplemented!();
    }

    fn copy_bit(&self, i: isize, j: isize, version_bits: isize) -> isize {
        unimplemented!();
    }
}