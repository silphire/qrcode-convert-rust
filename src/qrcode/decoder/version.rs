use crate::error::Error;
use crate::qrcode::decoder::format_information::FormatInformation;

static VERSION_DECODE_INFO: [isize; 34] = [
    0x07C94, 0x085BC, 0x09A99, 0x0A4D3, 0x0BBF6,
    0x0C762, 0x0D847, 0x0E60D, 0x0F928, 0x10B78,
    0x1145D, 0x12A17, 0x13532, 0x149A6, 0x15683,
    0x168C9, 0x177EC, 0x18EC4, 0x191E1, 0x1AFAB,
    0x1B08E, 0x1CC1A, 0x1D33F, 0x1ED75, 0x1F250,
    0x209D5, 0x216F0, 0x228BA, 0x2379F, 0x24B0B,
    0x2542E, 0x26A64, 0x27541, 0x28C69
];

static VERSIONS: Vec<Version> = build_versions();

pub struct Version {
    version_number: isize,
    alignment_pattern_centers: Vec<isize>,
    ec_blocks: Vec<ECBlocks>,
    total_codewords: isize,
}

impl Version {
    pub const fn get_version_number(&self) -> isize {
        return self.version_number;
    }

    pub const fn get_alignment_pattern_centers(&self) -> Vec<isize> {
        return self.alignment_pattern_centers;
    }

    pub fn get_provisional_version_for_dimension(dimension: isize) -> Result<Version, Error> {
        if dimension % 4 != 1 {
            return Err(Error::FormatError);
        }

        return match Self::get_version_for_number((dimension - 17) / 4) {
            Ok(version) => Ok(version),
            Err(Error::IllegalArgumentError) => Err(Error::FormatError),
        };
    }

    pub fn get_version_for_number(version_number: isize) -> Result<Version, Error> {
        if version_number < 1 || version_number > 40 {
            return Err(Error::IllegalArgumentError);
        }

        return Ok(VERSIONS[(version_number - 1) as usize]);
    }

    fn decode_version_information(version_bits: isize) -> Option<Version> {
        let best_difference;   // TODO
        let best_version: isize = 0;
        for i in 0..VERSION_DECODE_INFO.len() {
            let target_version = VERSION_DECODE_INFO[i];
            if target_version == version_bits {
                return match Self::get_version_for_number((i + 7) as isize) {
                    Ok(version) => Some(version),
                    Err(error) => None,
                };
            }

            let bits_difference = FormatInformation::num_bits_differing(version_bits, target_version);
            if bits_difference < best_difference {
                best_version = i as isize + 7;
                best_difference = bits_difference;
            }
        }

        if best_difference <= 3 {
            return match Self::get_version_for_number(best_version) {
                Ok(version) => Some(version),
                Err(error) => None,
            };
        }

        return None;
    }

    pub const fn get_total_codewords(&self) -> isize {
        return self.total_codewords;
    }

    pub const fn get_dimension_for_version(&self) -> isize {
        return 17 + 4 * self.get_version_number();
    }

    pub fn new(version_number: isize, alignment_pattern_centers: Vec<isize>, ec_blocks: Vec<ECBlocks>) -> Version {
        let total = 0;
        let ec_codewords = ec_blocks[0].get_ec_codewords_per_block();
        let ecb_array = ec_blocks[0].get_ec_blocks();
        for ec_block in ecb_array {
            total += ec_block.get_count() * (ec_block.get_data_codewords() + ec_codewords);
        }

        return Version {
            version_number: version_number,
            alignment_pattern_centers: alignment_pattern_centers,
            ec_blocks: ec_blocks,
            total_codewords: total,
        };
    }
}

struct ECBlocks {
    ec_codewords_per_block: isize,
    ec_blocks: Vec<ECB>,
}

impl ECBlocks {
    pub fn new(ec_codewords_per_block: isize, ec_blocks: Vec<ECB>) -> ECBlocks {
        return ECBlocks {
            ec_codewords_per_block: ec_codewords_per_block,
            ec_blocks: ec_blocks,
        };
    }

    pub const fn get_ec_codewords_per_block(&self) -> isize {
        return self.ec_codewords_per_block;
    }

    pub const fn get_num_blocks(&self) -> isize {
        let total = 0;
        for ec_block in self.get_ec_blocks() {
            total += ec_block.get_count();
        }
        return total;
    }

    pub const fn get_total_ec_codewords(&self) -> isize {
        return self.get_ec_codewords_per_block() * self.get_num_blocks();
    }

    pub const fn get_ec_blocks(&self) -> &Vec<ECB> {
        return &self.ec_blocks;
    }
}

struct ECB {
    count: isize,
    data_codewords: isize,
}

impl ECB {
    pub fn new(count: isize, data_codewords: isize) -> ECB {
        return ECB {
            count: count,
            data_codewords: data_codewords,
        };
    }

    pub const fn get_count(&self) -> isize {
        return self.count;
    }

    pub const fn get_data_codewords(&self) -> isize {
        return self.data_codewords;
    }
}

fn build_versions() -> Vec<Version> {
    return vec![
        Version::new(1, vec![], vec![
            ECBlocks::new(7, vec![ECB::new(1, 19)]),
            ECBlocks::new(10, vec![ECB::new(1, 16)]),
            ECBlocks::new(13, vec![ECB::new(1, 13)]),
            ECBlocks::new(17, vec![ECB::new(1, 9)]),
        ]),
        Version::new(2, vec![6, 18], vec![
            ECBlocks::new(10, vec![ECB::new(1, 34)]),
            ECBlocks::new(16, vec![ECB::new(1, 28)]),
            ECBlocks::new(22, vec![ECB::new(1, 22)]),
            ECBlocks::new(28, vec![ECB::new(1, 16)]),
        ]),
        Version::new(3, vec![6, 22], vec![
            ECBlocks::new(15, vec![ECB::new(1, 55)]),
            ECBlocks::new(26, vec![ECB::new(1, 44)]),
            ECBlocks::new(18, vec![ECB::new(2, 17)]),
            ECBlocks::new(22, vec![ECB::new(2, 13)]),
        ]),
        Version::new(4, vec![6, 26], vec![
            ECBlocks::new(20, vec![ECB::new(1, 80)]),
            ECBlocks::new(18, vec![ECB::new(2, 32)]),
            ECBlocks::new(26, vec![ECB::new(2, 24)]),
            ECBlocks::new(16, vec![ECB::new(4, 9)]),
        ]),
        Version::new(5, vec![6, 30], vec![
            ECBlocks::new(26, vec![ECB::new(1, 108)]),
            ECBlocks::new(24, vec![ECB::new(2, 43)]),
            ECBlocks::new(18, vec![ECB::new(2, 15), ECB::new(2, 16)]),
            ECBlocks::new(22, vec![ECB::new(2, 11), ECB::new(2, 12)]),
        ]),
        Version::new(6, vec![6, 34], vec![
            ECBlocks::new(18, vec![ECB::new(2, 68)]),
            ECBlocks::new(16, vec![ECB::new(4, 27)]),
            ECBlocks::new(24, vec![ECB::new(4, 19)]),
            ECBlocks::new(28, vec![ECB::new(4, 15)]),
        ]),
        Version::new(7, vec![6, 22, 38], vec![
            ECBlocks::new(20, vec![ECB::new(2, 78)]),
            ECBlocks::new(18, vec![ECB::new(4, 31)]),
            ECBlocks::new(18, vec![ECB::new(2, 14), ECB::new(4, 15)]),
            ECBlocks::new(26, vec![ECB::new(4, 13), ECB::new(1, 14)]),
        ]),
        Version::new(8, vec![6, 24, 42], vec![
            ECBlocks::new(24, vec![ECB::new(2, 97)]),
            ECBlocks::new(22, vec![ECB::new(2, 38), ECB::new(2, 39)]),
            ECBlocks::new(22, vec![ECB::new(4, 18), ECB::new(2, 19)]),
            ECBlocks::new(26, vec![ECB::new(4, 14), ECB::new(2, 15)]),
        ]),
        Version::new(9, vec![6, 26, 46], vec![
            ECBlocks::new(30, vec![ECB::new(2, 116)]),
            ECBlocks::new(22, vec![ECB::new(3, 36), ECB::new(2, 37)]),
            ECBlocks::new(20, vec![ECB::new(4, 16), ECB::new(4, 17)]),
            ECBlocks::new(24, vec![ECB::new(4, 12), ECB::new(4, 13)]),
        ]),
        Version::new(10, vec![6, 28, 50], vec![
            ECBlocks::new(18, vec![ECB::new(2, 68), ECB::new(2, 69)]),
            ECBlocks::new(26, vec![ECB::new(4, 43), ECB::new(1, 44)]),
            ECBlocks::new(24, vec![ECB::new(6, 19), ECB::new(2, 20)]),
            ECBlocks::new(28, vec![ECB::new(6, 15), ECB::new(2, 16)]),
        ]),
        Version::new(11, vec![6, 30, 54], vec![
            ECBlocks::new(20, vec![ECB::new(4, 81)]),
            ECBlocks::new(30, vec![ECB::new(1, 50), ECB::new(4, 51)]),
            ECBlocks::new(28, vec![ECB::new(4, 22), ECB::new(4, 23)]),
            ECBlocks::new(24, vec![ECB::new(3, 12), ECB::new(8, 13)]),
        ]),
        Version::new(12, vec![6, 32, 58], vec![
            ECBlocks::new(24, vec![ECB::new(2, 92), ECB::new(2, 93)]),
            ECBlocks::new(22, vec![ECB::new(6, 36), ECB::new(2, 37)]),
            ECBlocks::new(26, vec![ECB::new(4, 20), ECB::new(6, 21)]),
            ECBlocks::new(28, vec![ECB::new(7, 14), ECB::new(4, 15)]),
        ]),
        Version::new(13, vec![6, 34, 62], vec![
            ECBlocks::new(26, vec![ECB::new(4, 107)]),
            ECBlocks::new(22, vec![ECB::new(8, 37), ECB::new(1, 38)]),
            ECBlocks::new(24, vec![ECB::new(8, 20), ECB::new(4, 21)]),
            ECBlocks::new(22, vec![ECB::new(12, 11), ECB::new(4, 12)]),
        ]),
        Version::new(14, vec![6, 26, 46, 66], vec![
            ECBlocks::new(30, vec![ECB::new(3, 115), ECB::new(1, 116)]),
            ECBlocks::new(24, vec![ECB::new(4, 40), ECB::new(5, 41)]),
            ECBlocks::new(20, vec![ECB::new(11, 16), ECB::new(5, 17)]),
            ECBlocks::new(24, vec![ECB::new(11, 12), ECB::new(5, 13)]),
        ]),
        Version::new(15, vec![6, 26, 48, 70], vec![
            ECBlocks::new(22, vec![ECB::new(5, 87), ECB::new(1, 88)]),
            ECBlocks::new(24, vec![ECB::new(5, 41), ECB::new(5, 42)]),
            ECBlocks::new(30, vec![ECB::new(5, 24), ECB::new(7, 25)]),
            ECBlocks::new(24, vec![ECB::new(11, 12), ECB::new(7, 13)]),
        ]),
        Version::new(16, vec![6, 26, 50, 74], vec![
            ECBlocks::new(24, vec![ECB::new(5, 98), ECB::new(1, 99)]),
            ECBlocks::new(28, vec![ECB::new(7, 45), ECB::new(3, 46)]),
            ECBlocks::new(24, vec![ECB::new(15, 19), ECB::new(2, 20)]),
            ECBlocks::new(30, vec![ECB::new(3, 15), ECB::new(13, 16)])
        ]),
        Version::new(17, vec![6, 30, 54, 78], vec![
            ECBlocks::new(28, vec![ECB::new(1, 107), ECB::new(5, 108)]),
            ECBlocks::new(28, vec![ECB::new(10, 46), ECB::new(1, 47)]),
            ECBlocks::new(28, vec![ECB::new(1, 22), ECB::new(15, 23)]),
            ECBlocks::new(28, vec![ECB::new(2, 14), ECB::new(17, 15)]),
        ]),
        Version::new(18, vec![6, 30, 56, 82], vec![
            ECBlocks::new(30, vec![ECB::new(5, 120), ECB::new(1, 121)]),
            ECBlocks::new(26, vec![ECB::new(9, 43), ECB::new(4, 44)]),
            ECBlocks::new(28, vec![ECB::new(17, 22), ECB::new(1, 23)]),
            ECBlocks::new(28, vec![ECB::new(2, 14), ECB::new(19, 15)]),
        ]),
        Version::new(19, vec![6, 30, 58, 86], vec![
            ECBlocks::new(28, vec![ECB::new(3, 113), ECB::new(4, 114)]),
            ECBlocks::new(26, vec![ECB::new(3, 44), ECB::new(11, 45)]),
            ECBlocks::new(26, vec![ECB::new(17, 21), ECB::new(4, 22)]),
            ECBlocks::new(26, vec![ECB::new(9, 13), ECB::new(16, 14)]),
        ]),
        Version::new(20, vec![6, 34, 62, 90], vec![
            ECBlocks::new(28, vec![ECB::new(3, 107), ECB::new(5, 108)]),
            ECBlocks::new(26, vec![ECB::new(3, 41), ECB::new(13, 42)]),
            ECBlocks::new(30, vec![ECB::new(15, 24), ECB::new(5, 25)]),
            ECBlocks::new(28, vec![ECB::new(15, 15), ECB::new(10, 16)]),
        ]),
        Version::new(21, vec![6, 28, 50, 72, 94], vec![
            ECBlocks::new(28, vec![ECB::new(4, 116), ECB::new(4, 117)]),
            ECBlocks::new(26, vec![ECB::new(17, 42)]),
            ECBlocks::new(28, vec![ECB::new(17, 22), ECB::new(6, 23)]),
            ECBlocks::new(30, vec![ECB::new(19, 16), ECB::new(6, 17)]),
        ]),
        Version::new(22, vec![6, 26, 50, 74, 98], vec![
            ECBlocks::new(28, vec![ECB::new(2, 111), ECB::new(7, 112)]),
            ECBlocks::new(28, vec![ECB::new(17, 46)]),
            ECBlocks::new(30, vec![ECB::new(7, 24), ECB::new(16, 25)]),
            ECBlocks::new(24, vec![ECB::new(34, 13)]),
        ]),
        Version::new(23, vec![6, 30, 54, 78, 102], vec![
            ECBlocks::new(30, vec![ECB::new(4, 121), ECB::new(5, 122)]),
            ECBlocks::new(28, vec![ECB::new(4, 47), ECB::new(14, 48)]),
            ECBlocks::new(30, vec![ECB::new(11, 24), ECB::new(14, 25)]),
            ECBlocks::new(30, vec![ECB::new(16, 15), ECB::new(14, 16)]),
        ]),
        Version::new(24, vec![6, 28, 54, 80, 106], vec![
            ECBlocks::new(30, vec![ECB::new(6, 117), ECB::new(4, 118)]),
            ECBlocks::new(28, vec![ECB::new(6, 45), ECB::new(14, 46)]),
            ECBlocks::new(30, vec![ECB::new(11, 24), ECB::new(16, 25)]),
            ECBlocks::new(30, vec![ECB::new(30, 16), ECB::new(2, 17)]),
        ]),
        Version::new(25, vec![6, 32, 58, 84, 110], vec![
            ECBlocks::new(26, vec![ECB::new(8, 106), ECB::new(4, 107)]),
            ECBlocks::new(28, vec![ECB::new(8, 47), ECB::new(13, 48)]),
            ECBlocks::new(30, vec![ECB::new(7, 24), ECB::new(22, 25)]),
            ECBlocks::new(30, vec![ECB::new(22, 15), ECB::new(13, 16)]),
        ]),
        Version::new(26, vec![6, 30, 58, 86, 114], vec![
            ECBlocks::new(28, vec![ECB::new(10, 114), ECB::new(2, 115)]),
            ECBlocks::new(28, vec![ECB::new(19, 46), ECB::new(4, 47)]),
            ECBlocks::new(28, vec![ECB::new(28, 22), ECB::new(6, 23)]),
            ECBlocks::new(30, vec![ECB::new(33, 16), ECB::new(4, 17)]),
        ]),
        Version::new(27, vec![6, 34, 62, 90, 118], vec![
            ECBlocks::new(30, vec![ECB::new(8, 122), ECB::new(4, 123)]),
            ECBlocks::new(28, vec![ECB::new(22, 45), ECB::new(3, 46)]),
            ECBlocks::new(30, vec![ECB::new(8, 23), ECB::new(26, 24)]),
            ECBlocks::new(30, vec![ECB::new(12, 15), ECB::new(28, 16)]),
        ]),
        Version::new(28, vec![6, 26, 50, 74, 98, 122], vec![
            ECBlocks::new(30, vec![ECB::new(3, 117), ECB::new(10, 118)]),
            ECBlocks::new(28, vec![ECB::new(3, 45), ECB::new(23, 46)]),
            ECBlocks::new(30, vec![ECB::new(4, 24), ECB::new(31, 25)]),
            ECBlocks::new(30, vec![ECB::new(11, 15), ECB::new(31, 16)]),
        ]),
        Version::new(29, vec![6, 30, 54, 78, 102, 126], vec![
            ECBlocks::new(30, vec![ECB::new(7, 116), ECB::new(7, 117)]),
            ECBlocks::new(28, vec![ECB::new(21, 45), ECB::new(7, 46)]),
            ECBlocks::new(30, vec![ECB::new(1, 23), ECB::new(37, 24)]),
            ECBlocks::new(30, vec![ECB::new(19, 15), ECB::new(26, 16)]),
        ]),
        Version::new(30, vec![6, 26, 52, 78, 104, 130], vec![
            ECBlocks::new(30, vec![ECB::new(5, 115), ECB::new(10, 116)]),
            ECBlocks::new(28, vec![ECB::new(19, 47), ECB::new(10, 48)]),
            ECBlocks::new(30, vec![ECB::new(15, 24), ECB::new(25, 25)]),
            ECBlocks::new(30, vec![ECB::new(23, 15), ECB::new(25, 16)]),
        ]),
        Version::new(31, vec![6, 30, 56, 82, 108, 134], vec![
            ECBlocks::new(30, vec![ECB::new(13, 115), ECB::new(3, 116)]),
            ECBlocks::new(28, vec![ECB::new(2, 46), ECB::new(29, 47)]),
            ECBlocks::new(30, vec![ECB::new(42, 24), ECB::new(1, 25)]),
            ECBlocks::new(30, vec![ECB::new(23, 15), ECB::new(28, 16)]),
        ]),
        Version::new(32, vec![6, 34, 60, 86, 112, 138], vec![
            ECBlocks::new(30, vec![ECB::new(17, 115)]),
            ECBlocks::new(28, vec![ECB::new(10, 46), ECB::new(23, 47)]),
            ECBlocks::new(30, vec![ECB::new(10, 24), ECB::new(35, 25)]),
            ECBlocks::new(30, vec![ECB::new(19, 15), ECB::new(35, 16)]),
        ]),
        Version::new(33, vec![6, 30, 58, 86, 114, 142], vec![
            ECBlocks::new(30, vec![ECB::new(17, 115), ECB::new(1, 116)]),
            ECBlocks::new(28, vec![ECB::new(14, 46), ECB::new(21, 47)]),
            ECBlocks::new(30, vec![ECB::new(29, 24), ECB::new(19, 25)]),
            ECBlocks::new(30, vec![ECB::new(11, 15), ECB::new(46, 16)]),
        ]),
        Version::new(34, vec![6, 34, 62, 90, 118, 146], vec![
            ECBlocks::new(30, vec![ECB::new(13, 115), ECB::new(6, 116)]),
            ECBlocks::new(28, vec![ECB::new(14, 46), ECB::new(23, 47)]),
            ECBlocks::new(30, vec![ECB::new(44, 24), ECB::new(7, 25)]),
            ECBlocks::new(30, vec![ECB::new(59, 16), ECB::new(1, 17)]),
        ]),
        Version::new(35, vec![6, 30, 54, 78, 102, 126, 150], vec![
            ECBlocks::new(30, vec![ECB::new(12, 121), ECB::new(7, 122)]),
            ECBlocks::new(28, vec![ECB::new(12, 47), ECB::new(26, 48)]),
            ECBlocks::new(30, vec![ECB::new(39, 24), ECB::new(14, 25)]),
            ECBlocks::new(30, vec![ECB::new(22, 15), ECB::new(41, 16)]),
        ]),
        Version::new(36, vec![6, 24, 50, 76, 102, 128, 154], vec![
            ECBlocks::new(30, vec![ECB::new(6, 121), ECB::new(14, 122)]),
            ECBlocks::new(28, vec![ECB::new(6, 47), ECB::new(34, 48)]),
            ECBlocks::new(30, vec![ECB::new(46, 24), ECB::new(10, 25)]),
            ECBlocks::new(30, vec![ECB::new(2, 15), ECB::new(64, 16)]),
        ]),
        Version::new(37, vec![6, 28, 54, 80, 106, 132, 158], vec![
            ECBlocks::new(30, vec![ECB::new(17, 122), ECB::new(4, 123)]),
            ECBlocks::new(28, vec![ECB::new(29, 46), ECB::new(14, 47)]),
            ECBlocks::new(30, vec![ECB::new(49, 24), ECB::new(10, 25)]),
            ECBlocks::new(30, vec![ECB::new(24, 15), ECB::new(46, 16)]),
        ]),
        Version::new(38, vec![6, 32, 58, 84, 110, 136, 162], vec![
            ECBlocks::new(30, vec![ECB::new(4, 122), ECB::new(18, 123)]),
            ECBlocks::new(28, vec![ECB::new(13, 46), ECB::new(32, 47)]),
            ECBlocks::new(30, vec![ECB::new(48, 24), ECB::new(14, 25)]),
            ECBlocks::new(30, vec![ECB::new(42, 15), ECB::new(32, 16)]),
        ]),
        Version::new(39, vec![6, 26, 54, 82, 110, 138, 166], vec![
            ECBlocks::new(30, vec![ECB::new(20, 117), ECB::new(4, 118)]),
            ECBlocks::new(28, vec![ECB::new(40, 47), ECB::new(7, 48)]),
            ECBlocks::new(30, vec![ECB::new(43, 24), ECB::new(22, 25)]),
            ECBlocks::new(30, vec![ECB::new(10, 15), ECB::new(67, 16)]),
        ]),
        Version::new(40, vec![6, 30, 58, 86, 114, 142, 170], vec![
            ECBlocks::new(30, vec![ECB::new(19, 118), ECB::new(6, 119)]),
            ECBlocks::new(28, vec![ECB::new(18, 47), ECB::new(31, 48)]),
            ECBlocks::new(30, vec![ECB::new(34, 24), ECB::new(34, 25)]),
            ECBlocks::new(30, vec![ECB::new(20, 15), ECB::new(61, 16)]),
        ]),
    ];
}
