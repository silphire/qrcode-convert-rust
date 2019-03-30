use std::collections::HashMap;

use crate::reader::Reader;
use crate::binary_bitmap::BinaryBitmap;
use crate::decode_hint_type::DecodeHintType;

pub struct MultiFormatReader<'a> {
    pub readers: Vec<Box<Reader<'a>>>,
    pub hints: &'a HashMap<DecodeHintType, u8>,
}

impl<'a> MultiFormatReader<'a> {
    fn decode_internal(&'a mut self, image: &'a BinaryBitmap) {
        for reader in &mut self.readers {
            reader.decode(image, Some(self.hints));
        }
    }

    fn set_hints(&'a mut self, hints: &'a HashMap<DecodeHintType, u8>) -> &mut MultiFormatReader<'a> {
        self.hints = hints;
        // TODO implement

        return self;
    }
}

impl<'a> Reader<'a> for MultiFormatReader<'a> {
    fn decode(&'a mut self, image: &'a BinaryBitmap, hints: Option<&'a HashMap<DecodeHintType, u8>>) {
        if hints.is_none() {
            self.set_hints(hints.unwrap()).decode_internal(image);
        } else {
            self.set_hints(hints.unwrap());
        }
    }

    fn reset(&mut self) {
        for reader in &mut self.readers {
            (*reader).reset();
        }
    }
}
