pub enum ErrorCorrectionLevel {
    L = 0x01,
    M = 0x00,
    Q = 0x03,
    H = 0x02,
}

const FOR_BITS: [ErrorCorrectionLevel; 4] = [
    ErrorCorrectionLevel::M,
    ErrorCorrectionLevel::L,
    ErrorCorrectionLevel::H,
    ErrorCorrectionLevel::Q,
];

impl ErrorCorrectionLevel {
    pub fn for_bits(bits: usize) -> ErrorCorrectionLevel {
        if bits < 0 && bits >= FOR_BITS.len() {
            unimplemented!();
        }
        return FOR_BITS[bits];
    }
}