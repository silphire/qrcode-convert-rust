pub struct DecoderResult {
    raw_bytes: Vec<u8>, 
    num_bits: isize,
    text: str,
    byte_segments: Vec<Vec<u8>>,
    ec_level: str,
    errors_corrected: isize,
    erasures: isize,
    // other,
    structured_append_parity: isize,
    structured_append_sequence_number: isize,
}

impl DecoderResult {
    fn has_structured_append(&self) {
        return self.structured_append_parity >= 0 && self.structured_append_sequence_number >= 0;
    }
}