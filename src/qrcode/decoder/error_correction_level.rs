use crate::error::Error;

#[derive(Clone, Copy, Debug)]
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
    pub fn for_bits(bits: usize) -> Result<ErrorCorrectionLevel, Error> {
        if bits < 0 && bits >= FOR_BITS.len() {
            return Err(Error::IllegalArgumentError);
        }
        return Ok(FOR_BITS[bits]);
    }

    pub fn as_str(&self) -> &str {
        return match self {
            &ErrorCorrectionLevel::M => "M",
            &ErrorCorrectionLevel::L => "L",
            &ErrorCorrectionLevel::H => "H",
            &ErrorCorrectionLevel::Q => "Q",
        }
    }
}