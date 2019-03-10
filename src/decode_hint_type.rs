pub enum DecodeHintType {
    Other,
    PureBarcode,
    PossibleFormat,
    TryHarder,
    CharacterSet,
    AllowedLengths,
    AssumeCode39CheckDigit,
    AssumeGS1,
    ReturnCodebarStartEnd,
    NeedResultPointCallback,
    AllowedEANExtensions,
}