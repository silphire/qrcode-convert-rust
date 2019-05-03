use crate::error::Error;
use crate::qrcode::decoder::error_correction_level::ErrorCorrectionLevel;
use crate::qrcode::decoder::version::Version;

pub struct DataBlock {
    num_data_codewords: isize,
    codewords: Vec<u8>,
}

impl DataBlock {
    pub fn get_data_blocks(raw_codewords: &[u8], version: &Version, ec_level: &ErrorCorrectionLevel) -> Result<Vec<DataBlock>, Error> {
        if raw_codewords.len() as isize != version.get_total_codewords() {
            return Err(Error::IllegalArgumentError);
        }

        unimplemented!();
    }
    
    pub const fn get_num_data_codewords(&self) -> isize {
        return self.num_data_codewords;
    }

    pub const fn get_codewords(&self) -> &Vec<u8> {
        return &self.codewords;
    }
}