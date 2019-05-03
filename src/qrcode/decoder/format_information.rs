use crate::qrcode::decoder::error_correction_level::ErrorCorrectionLevel;

const FORMAT_INFO_MASK_QR: isize = 0x5412;

pub struct FormatInformation {
    error_correction_level: ErrorCorrectionLevel,
    data_mask: u8,
}

impl FormatInformation {
    pub fn num_bits_differing(a: isize, b: isize) -> isize {
        return (a ^ b).count_ones() as isize;
    }

    pub const fn get_error_correction_level(&self) -> ErrorCorrectionLevel {
        return self.error_correction_level;
    }

    pub const fn get_data_mask(&self) -> u8 {
        return self.data_mask;
    }
}