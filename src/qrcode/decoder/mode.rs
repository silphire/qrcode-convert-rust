pub enum Mode {
    TERMINATOR {
        character_count_bits_for_versions: Vec<isize> = vec![0, 0, 0],
        bits: isize = 0x00,
    },
    NUMERIC,
    ALPHANUMERIC,
    STRUCTURED_APPEND,
    BYTE,
    ECI,
    KANJI,
    FCNI1_FIRST_POSITION,
    FCNI1_SECOND_POSITION,
    HANZI,
}