use std::collections::HashMap;
use crate::binary_bitmap::BinaryBitmap;
use crate::decode_hint_type::DecodeHintType;

pub trait Reader<'a> {
    fn decode(&'a mut self, image: &'a BinaryBitmap, hints: Option<&'a HashMap<DecodeHintType, u8>>);
    fn reset(&mut self);
}