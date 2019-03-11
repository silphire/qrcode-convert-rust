pub struct ReedSolomonDecoder {
    generic_gf: u8, // TODO GenericGF
}

impl ReedSolomonDecoder {
    pub fn new() -> ReedSolomonDecoder {
        return ReedSolomonDecoder {
            generic_gf: 0,
        }
    }
}