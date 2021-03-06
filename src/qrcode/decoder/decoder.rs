use std::any::Any;
use std::collections::HashMap;
use crate::decode_hint_type::DecodeHintType;
use crate::common::reedsolomon::generic_gf;
use crate::common::reedsolomon::generic_gf::GenericGF;
use crate::qrcode::decoder::bitmatrix_parser::BitMatrixParser;
use crate::common::reedsolomon::reedsolomon_decoder::ReedSolomonDecoder;

pub struct Decoder {
    pub rs_decoder: ReedSolomonDecoder,
}

impl Decoder {
    pub fn new() -> Decoder {
        return Decoder {
            rs_decoder: ReedSolomonDecoder::new(generic_gf::QR_CODE_FIELD_256),
        };
    }

    pub fn decode(&mut self) {

    }

    pub fn decode_with_hint(&mut self, hints: &HashMap<DecodeHintType, &Any>) {

    }

    pub fn decode_with_parser_and_hint(&mut self, parser: &BitMatrixParser, hints: &HashMap<DecodeHintType, u8>) {
        // TODO implement
    }

    fn correct_errors(&mut self, codeword_bytes: &mut Vec<u8>, num_data_codewords: isize) {
        let num_codewords = codeword_bytes.len();
        let mut codewords_ints: Vec<i32> = vec![];

        for i in 0..num_codewords {
            codewords_ints[i] = (codeword_bytes[i] & 0xFF) as i32;
        }

        self.rs_decoder.decode(&codewords_ints, codeword_bytes.len() as isize - num_data_codewords);

        for i in 0..num_codewords {
            codeword_bytes[i] = codewords_ints[i] as u8;
        }
    }
}