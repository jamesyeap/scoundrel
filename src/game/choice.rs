#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ChoiceParseError {
    INVALID_OPTION(String),
}

impl std::fmt::Display for ChoiceParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ChoiceParseError::INVALID_OPTION(v) => {
                write!(f, "value {} is not a valid input (allowed 0..=4, or q)", v)
            }
        }
    }
}

impl std::error::Error for ChoiceParseError {}

pub enum Choice {
    OPTION(u8),
    RUN,
    EXIT,
}

impl TryFrom<&str> for Choice {
    type Error = ChoiceParseError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let trimmed_value = value.trim();
        match trimmed_value {
            "q" => Ok(Self::EXIT),
            "0" => Ok(Self::RUN),
            "1" | "2" | "3" | "4" => {
                let card_num = trimmed_value
                    .parse::<u8>()
                    .map_err(|_| ChoiceParseError::INVALID_OPTION(value.into()))?;
                Ok(Self::OPTION(card_num))
            }
            _ => Err(ChoiceParseError::INVALID_OPTION(value.into())),
        }
    }
}
