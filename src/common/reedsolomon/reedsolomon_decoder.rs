pub struct ReedSolomonDecoder {
    generic_gf: u8, // TODO GenericGF
}

impl ReedSolomonDecoder {
    pub fn new() -> ReedSolomonDecoder {
        return ReedSolomonDecoder {
            generic_gf: 0,
        }
    }

    pub fn decode(&mut self, received: Vec<i32>, two_s: usize) {
        // TODO implement
    }
}