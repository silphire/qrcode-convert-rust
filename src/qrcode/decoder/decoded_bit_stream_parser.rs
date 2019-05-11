use crate::common::bitsource::BitSource;
use crate::error::Error;

fn parse_eci_value(bits: &BitSource) -> Result<isize, Error> {
    return Err(Error::FormatError);
}