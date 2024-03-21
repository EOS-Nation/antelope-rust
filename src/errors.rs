#[derive(Debug, PartialEq, Clone)]
pub enum ParseError {
    BadFormat,
    BadAmount(String),
    BadSymbolCode(String),
    BadSymbol(String),
    BadPrecision(String),
    BadAsset(String),
    BadName(String),
}

impl std::fmt::Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            ParseError::BadFormat => write!(f, "bad format"),
            ParseError::BadSymbolCode(s) => write!(f, "bad symbol code: {}", s),
            ParseError::BadSymbol(s) => write!(f, "bad symbol: {}", s),
            ParseError::BadAmount(s) => write!(f, "bad amount: {}", s),
            ParseError::BadPrecision(s) => write!(f, "bad precision: {}", s),
            ParseError::BadAsset(s) => write!(f, "bad asset: {}", s),
            ParseError::BadName(s) => write!(f, "bad name: {}", s),
        }
    }
}
