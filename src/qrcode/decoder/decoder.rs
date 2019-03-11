use std::collections::HashMap;
use crate::decode_hint_type::DecodeHintType;
use crate::qrcode::decoder::bitmatrix_parser::BitMatrixParser;
use crate::common::reedsolomon::reedsolomon_decoder::ReedSolomonDecoder;

pub struct Decoder {
    pub rs_decoder: ReedSolomonDecoder,
}

impl Decoder {
    pub fn new() -> Decoder {
        return Decoder {
            rs_decoder: ReedSolomonDecoder::new(),
        }
    }

    pub fn decode(&mut self) {

    }

    pub fn decode_with_hint(&mut self, hints: &HashMap<DecodeHintType, u8>) {

    }

    pub fn decode_with_parser_and_hint(&mut self, parser: &BitMatrixParser, hints: &HashMap<DecodeHintType, u8>) {

    }

    fn correct_errors(&mut self) {

    }
}