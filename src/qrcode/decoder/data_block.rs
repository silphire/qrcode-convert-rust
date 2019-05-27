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

        let ec_blocks = version.get_ec_blocks_for_level(ec_level);

        let ec_block_array = ec_blocks.get_ec_blocks();
        /* unused
        let mut total_blocks = 0;
        for ec_block in ec_block_array {
            total_blocks += ec_block.get_count();
        }
        */

        let mut result = vec![];
        let mut num_result_blocks = 0;
        for ec_block in ec_block_array {
            for _ in 0..ec_block.get_count() {
                let num_data_codewords = ec_block.get_data_codewords();
                let num_block_codewords = ec_blocks.get_ec_codewords_per_block() + num_data_codewords;
                let data_block = DataBlock {
                    num_data_codewords: num_data_codewords, 
                    codewords: vec![0; num_block_codewords as usize],
                };
                result.push(data_block);
                num_result_blocks += 1;
            }
        }

        let shoter_blocks_total_codewords = result[0].codewords.len();
        let mut longer_blocks_start_at = result.len() - 1;
        while longer_blocks_start_at >= 0 {
            let num_codewords = result[longer_blocks_start_at].codewords.len();
            if num_codewords == shoter_blocks_total_codewords {
                break;
            }
            longer_blocks_start_at -= 1;
        }
        longer_blocks_start_at -= 1;

        let shorter_blocks_num_data_codewords = shoter_blocks_total_codewords - ec_blocks.get_ec_codewords_per_block() as usize;
        let mut raw_codewords_offset = 0;
        for i in 0..shorter_blocks_num_data_codewords {
            for j in 0..num_result_blocks {
                result[j].codewords[i] = raw_codewords[raw_codewords_offset];
                raw_codewords_offset += 1;
            }
        }

        for j in longer_blocks_start_at..num_result_blocks {
            result[j].codewords[shorter_blocks_num_data_codewords] = raw_codewords[raw_codewords_offset];
            raw_codewords_offset += 1;
        }

        let max = result[0].codewords.len();
        for i in shorter_blocks_num_data_codewords..max {
            for j in 0..num_result_blocks {
                let i_offset = if j < longer_blocks_start_at { i } else { i + 1};
                result[j].codewords[i_offset] = raw_codewords[raw_codewords_offset];
                raw_codewords_offset += 1;
            }
        }

        return Ok(result);
    }
    
    pub const fn get_num_data_codewords(&self) -> isize {
        return self.num_data_codewords;
    }

    pub const fn get_codewords(&self) -> &Vec<u8> {
        return &self.codewords;
    }
}