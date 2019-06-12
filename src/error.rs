#[derive(Debug)]
pub enum Error {
    ChecksumError,
    FormatError,
    NotFoundError,
    ReaderError,
    WriterError,
    IllegalArgumentError,
    ArithmeticError,
}