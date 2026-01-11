use crossterm::event::{KeyCode, KeyEvent};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ChoiceParseError {
    INVALID_KEY(KeyCode),
    INVALID_OPTION(String),
}

impl std::fmt::Display for ChoiceParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ChoiceParseError::INVALID_OPTION(v) => {
                write!(f, "value {} is not a valid input (allowed 0..=4, or q)", v)
            },
            ChoiceParseError::INVALID_KEY(key) => {
                write!(f, "value {} is not a valid input (allowed 0..=4, or q)", key)
            }
        }
    }
}

impl std::error::Error for ChoiceParseError {}

#[derive(PartialEq)]
pub enum Choice {
    OPTION(u8),
    FIGHT_WITH_WEAPON(bool),
    RUN,
    EXIT,
}

impl TryFrom<KeyEvent> for Choice {
    type Error = ChoiceParseError;

    fn try_from(value: KeyEvent) -> Result<Self, Self::Error> {
        match value.code {
            KeyCode::Char(char) => match char {
                'q' => Ok(Self::EXIT),
                'y' => Ok(Self::FIGHT_WITH_WEAPON(true)),
                'n' => Ok(Self::FIGHT_WITH_WEAPON(false)),
                '0' => Ok(Self::RUN),
                '1' | '2' | '3' | '4' => {
                    Ok(Self::OPTION(char.to_digit(10).unwrap() as u8))
                },
                _ => Err(ChoiceParseError::INVALID_KEY(value.code)),
            },
            _ => Err(ChoiceParseError::INVALID_KEY(value.code)),
        }
    }
}
