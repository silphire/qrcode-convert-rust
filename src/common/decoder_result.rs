#[derive(Debug)]
pub struct DecoderResult<'a> {
    raw_bytes: &'a Vec<u8>,
    num_bits: isize,
    text: String,
    byte_segments: &'a Vec<Vec<u8>>,
    ec_level: String,
    errors_corrected: isize,
    erasures: isize,
    // other,
    structured_append_parity: isize,
    structured_append_sequence_number: isize,
}

impl<'a> DecoderResult<'a> {
    pub fn new(raw_bytes: &'a Vec<u8>, text: &str, byte_segments: &'a Vec<Vec<u8>>, ec_level: &str, sa_sequence: isize, sa_parity: isize) -> DecoderResult<'a> {
        return DecoderResult {
            raw_bytes: raw_bytes,
            num_bits: if raw_bytes.len() == 0 { 0 } else { 8 * raw_bytes.len() as isize },
            text: text.to_string(),
            byte_segments: byte_segments,
            ec_level: ec_level.to_string(),
            errors_corrected: 0,
            erasures: 0,
            structured_append_parity: sa_parity,
            structured_append_sequence_number: sa_sequence,
        };
    }

    pub const fn get_raw_bytes(&self) -> &Vec<u8> {
        return &self.raw_bytes;
    }

    pub const fn get_num_bits(&self) -> isize {
        return self.num_bits;
    }
    
    pub fn set_num_bits(&mut self, num_bits: isize) {
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

    pub fn set_ec_level(&mut self, ec_level: &String) {
        self.ec_level = ec_level.clone();
    }

    pub const fn get_errors_corrected(&self) -> isize {
        return self.errors_corrected;
    }

    pub fn set_errors_corrected(&mut self, errors_corrected: isize) {
        self.errors_corrected = errors_corrected;
    }

    pub const fn get_erasures(&self) -> isize {
        return self.erasures;
    }

    pub fn set_erasures(&mut self, erasures: isize) {
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