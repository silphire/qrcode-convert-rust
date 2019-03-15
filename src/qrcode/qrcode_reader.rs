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
        let right_bottom_black = image.get_bottom_right_on_bit();
        if left_top_black.len() == 0 || right_bottom_black.len() == 0 {
            // throw NotFound
        }

        let module_size = self.module_size(&left_top_black, image);

        // TODO implement
        return BitMatrix {
            bits: vec![],
            width: image.width,
            height: image.height,
            row_size: image.row_size,
        };
    }

    fn module_size(&mut, left_top_black: &Vec<usize>, image: BitMatrix) -> f64 {
        let x = left_top_black[0];
        let y = left_top_black[1];
        let in_black = true;
        let transition = 0;

        while x < image.width && y < image.heght {
            if in_black != image.get(x, y) {
                transition += 1;
                if transition == 5 {
                    break;
                }
                in_black = !in_black;
            }
            x += 1;
            y += 1;
        }

        if x == width || y == height {
            // throw not found
        }

        return (x - left_top_black[0]) / 7.0;
    }
}