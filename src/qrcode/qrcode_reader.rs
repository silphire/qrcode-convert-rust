use std::collections::HashMap;
use crate::binary_bitmap::BinaryBitmap;
use crate::decode_hint_type::DecodeHintType;
use crate::qrcode::decoder::decoder::Decoder;
use crate::reader::Reader;

pub struct QRCodeReader {
    pub decoder: Decoder,
}

impl<'a> Reader<'a> for QRCodeReader {
    fn decode(&mut self, image: &BinaryBitmap, hints: Option<&HashMap<DecodeHintType, u8>>) {
    }
    
    fn reset(&mut self) {

    }
}