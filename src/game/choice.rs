#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ChoiceParseError {
    OutOfRange(u8),
}

impl std::fmt::Display for ChoiceParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            ChoiceParseError::OutOfRange(v) => {
                write!(f, "value {} is out of range (allowed 0..=4)", v)
            }
        }
    }
}

impl std::error::Error for ChoiceParseError {}

pub enum Choice {
    OPTION(u8),
    EXIT,
}

impl TryFrom<u8> for Choice {
    type Error = ChoiceParseError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::EXIT),
            1..=4 => Ok(Self::OPTION(value)),
            _ => Err(ChoiceParseError::OutOfRange(value)),
        }
    }
}
