pub struct DecoderResult {
    raw_bytes: Vec<u8>, 
    num_bits: isize,
    text: String,
    byte_segments: Vec<Vec<u8>>,
    ec_level: String,
    errors_corrected: isize,
    erasures: isize,
    // other,
    structured_append_parity: isize,
    structured_append_sequence_number: isize,
}

impl DecoderResult {
    pub const fn get_raw_bytes(&self) -> &Vec<u8> {
        return &self.raw_bytes;
    }

    pub const fn get_num_bits(&self) -> isize {
        return self.num_bits;
    }
    
    pub fn set_num_bits(&self, num_bits: isize) {
        self.num_bits = num_bits;
    }

    pub const fn get_text(&self) -> &String {
        return &self.text;
    }

    pub const fn get_byte_segments(&self) -> &Vec<Vec<u8>> {
        return &self.byte_segments;
    }

    pub const fn get_ec_level(&self) -> &String {
        return &self.ec_level;
    }

    pub fn set_ec_level(&self, ec_level: &String) {
        self.ec_level = *ec_level;
    }

    pub const fn get_errors_corrected(&self) -> isize {
        return self.errors_corrected;
    }

    pub fn set_errors_corrected(&self, errors_corrected: isize) {
        self.errors_corrected = errors_corrected;
    }

    pub const fn get_erasures(&self) -> isize {
        return self.erasures;
    }

    pub fn set_erasures(&self, erasures: isize) {
        self.erasures = erasures;
    }

    pub fn has_structured_append(&self) -> bool {
        return self.structured_append_parity >= 0 && self.structured_append_sequence_number >= 0;
    }

    pub const fn get_structured_append_parity(&self) -> isize {
        return self.structured_append_parity;
    }

    pub const fn get_structured_append_sequence_number(&self) -> isize {
        return self.structured_append_sequence_number;
    }
}