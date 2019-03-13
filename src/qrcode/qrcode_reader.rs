use std::collections::HashMap;
use crate::binary_bitmap::BinaryBitmap;
use crate::common::bitmatrix::BitMatrix;
use crate::decode_hint_type::DecodeHintType;
use crate::qrcode::decoder::decoder::Decoder;
use crate::reader::Reader;

pub struct QRCodeReader {
    pub decoder: Decoder,
}

impl<'a> Reader<'a> for QRCodeReader {
    fn decode(&mut self, image: &BinaryBitmap, hints: Option<&HashMap<DecodeHintType, u8>>) {
        if hints.map_or(false, |v| (*v).contains_key(&DecodeHintType::PureBarcode)) {
            let bits: BitMatrix = self.extract_pure_bits(image.get_black_matrix());
        } else {
        }
    }

    fn reset(&mut self) {

    }
}

impl QRCodeReader {
    pub fn extract_pure_bits(&self, image: &BitMatrix) -> BitMatrix {
        let left_top_black = image.get_top_left_on_bit();

        // TODO implement
        return BitMatrix {
            bits: vec![],
            width: image.width,
            height: image.height,
            row_size: image.row_size,
        };
    }
}