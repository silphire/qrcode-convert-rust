use std::collections::HashMap;

use crate::reader::Reader;
use crate::binary_bitmap::BinaryBitmap;

pub struct MultiFormatReader {
    pub readers: Vec<Box<Reader>>,
}

impl Reader for MultiFormatReader {
    fn decode(&mut self, image: BinaryBitmap) {
    }

    fn decode_with_hint(&mut self, image: BinaryBitmap, hints: HashMap<u8, u8>) {

    }

    fn reset(&mut self) {

    }
}