use crate::common::bitsource::BitSource;
use crate::error::Error;

const ALPHANUMERIC_CHARS: &str = "0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZ $%*+-./:";
const GB2312_SUBSET: isize = 1;

fn to_alpha_numeric_char(value: usize) -> Result<char, Error> {
    if value >= ALPHANUMERIC_CHARS.len() {
        return Err(Error::FormatError);
    }

    return ALPHANUMERIC_CHARS.chars().nth(value).ok_or(Error::FormatError);
}

fn decode_numeric_segment(bits: &BitSource, result: &String, count: isize) -> Result<(), Error> {
    while count >= 3 {
        if bits.available() < 10 {
            return Err(Error::FormatError);
        }

        let three_digits_bits = bits.read_bits(10)? as usize;
        if three_digits_bits >= 1000 {
            return Err(Error::FormatError);
        }

        result.push(to_alpha_numeric_char(three_digits_bits / 100)?);
        result.push(to_alpha_numeric_char((three_digits_bits / 10) % 10)?);
        result.push(to_alpha_numeric_char(three_digits_bits % 10)?);
        count -= 3;
    }

    if count == 2 {
        if bits.available() < 7 {
            return Err(Error::FormatError);
        }

        let two_digits_bits = bits.read_bits(7)? as usize;
        if two_digits_bits >= 100 {
            return Err(Error::FormatError);
        }

        result.push(to_alpha_numeric_char(two_digits_bits / 10)?);
        result.push(to_alpha_numeric_char(two_digits_bits % 10)?);
    } else if count == 1 {
        if bits.available() < 4 {
            return Err(Error::FormatError);
        }

        let digits_bits = bits.read_bits(4)? as usize;
        if digits_bits >= 10 {
            return Err(Error::FormatError);
        }

        result.push(to_alpha_numeric_char(digits_bits)?);
    }

    return Ok(());
}

fn parse_eci_value(bits: &BitSource) -> Result<isize, Error> {
    return Err(Error::FormatError);
}