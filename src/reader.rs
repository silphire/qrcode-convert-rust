use std::collections::HashMap;
use crate::binary_bitmap::BinaryBitmap;

pub trait Reader {
    fn decode(&mut self, image: BinaryBitmap);
    fn decode_with_hint(&mut self, image: BinaryBitmap, hints: HashMap<u8, u8>);
    fn reset(&mut self);
}