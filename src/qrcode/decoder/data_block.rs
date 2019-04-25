use crate::qrcode::decoder::error_correction_level::ErrorCorrectionLevel;
use crate::qrcode::decoder::version::Version;

pub struct DataBlock {
    num_data_codewords: isize,
    codewords: [u8],
}

impl DataBlock {
    pub fn get_data_blocks(raw_codewords: &[u8], version: &Version, ec_level: &ErrorCorrectionLevel) -> Vec<DataBlock> {
        unimplemented!();
    }
    
    pub const fn get_num_data_codewords(&self) -> isize {
        return self.num_data_codewords;
    }

    pub const fn get_codewords(&self) -> &[u8] {
        return &self.codewords;
    }
}