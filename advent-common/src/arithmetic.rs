use crate::AdventError;

#[derive(Debug, Clone, Copy, Eq, Hash, PartialEq)]
pub enum Operator {
    Add,
    Subtract,
    Multiply,
    Divide,
}

impl TryFrom<char> for Operator {
    type Error = AdventError;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '+' => Ok(Self::Add),
            '-' => Ok(Self::Subtract),
            '/' => Ok(Self::Divide),
            '*' => Ok(Self::Multiply),
            _ => Err(AdventError::InputParseError(
                format!("invalid input operator {value}").into(),
            )),
        }
    }
}

impl<'a> TryFrom<&'a str> for Operator {
    type Error = AdventError;

    fn try_from(value: &'a str) -> Result<Self, Self::Error> {
        let c = value
            .chars()
            .next()
            .ok_or(AdventError::InputParseError(format!("not a char").into()))?;
        Operator::try_from(c)
    }
}
