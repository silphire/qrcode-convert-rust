use crate::qrcode::decoder::error_correction_level::ErrorCorrectionLevel;

const FORMAT_INFO_MASK_QR: isize = 0x5412;
const FORMAT_INFO_DECODE_LOOKUP: [[isize; 2]; 0x20] = [
    [0x5412, 0x00],
    [0x5125, 0x01],
    [0x5E7C, 0x02],
    [0x5B4B, 0x03],
    [0x45F9, 0x04],
    [0x40CE, 0x05],
    [0x4F97, 0x06],
    [0x4AA0, 0x07],
    [0x77C4, 0x08],
    [0x72F3, 0x09],
    [0x7DAA, 0x0A],
    [0x789D, 0x0B],
    [0x662F, 0x0C],
    [0x6318, 0x0D],
    [0x6C41, 0x0E],
    [0x6976, 0x0F],
    [0x1689, 0x10],
    [0x13BE, 0x11],
    [0x1CE7, 0x12],
    [0x19D0, 0x13],
    [0x0762, 0x14],
    [0x0255, 0x15],
    [0x0D0C, 0x16],
    [0x083B, 0x17],
    [0x355F, 0x18],
    [0x3068, 0x19],
    [0x3F31, 0x1A],
    [0x3A06, 0x1B],
    [0x24B4, 0x1C],
    [0x2183, 0x1D],
    [0x2EDA, 0x1E],
    [0x2BED, 0x1F],
];

pub struct FormatInformation {
    error_correction_level: ErrorCorrectionLevel,
    data_mask: u8,
}

impl FormatInformation {
    fn new(format_info: isize) -> FormatInformation {
        return FormatInformation {
            error_correction_level: ErrorCorrectionLevel::for_bits(((format_info >> 3) & 0x03) as usize),
            data_mask: (format_info & 0x07) as u8
        }
    }

    pub fn num_bits_differing(a: isize, b: isize) -> isize {
        return (a ^ b).count_ones() as isize;
    }

    fn do_decode_format_information(masked_format_info_1: isize, masked_format_info_2: isize) -> Option<FormatInformation> {
        let mut best_difference = std::isize::MAX;
        let mut best_format_info = 0;

        for decode_info in &FORMAT_INFO_DECODE_LOOKUP {
            let target_info = decode_info[0];
            if target_info == masked_format_info_1 || target_info == masked_format_info_2 {
                return Some(FormatInformation::new(decode_info[1]));
            }

            let mut bits_difference = Self::num_bits_differing(masked_format_info_1, target_info);
            if bits_difference < best_difference {
                best_format_info = decode_info[1];
                best_difference = bits_difference;
            }

            if masked_format_info_1 != masked_format_info_2 {
                bits_difference = Self::num_bits_differing(masked_format_info_2, target_info);
                if bits_difference < best_difference {
                    best_format_info = decode_info[1];
                    best_difference = bits_difference;
                }
            }
        }

        if best_difference <= 3 {
            return Some(FormatInformation::new(best_format_info));
        }

        return None;
    }

    pub const fn get_error_correction_level(&self) -> ErrorCorrectionLevel {
        return self.error_correction_level;
    }

    pub const fn get_data_mask(&self) -> u8 {
        return self.data_mask;
    }
}