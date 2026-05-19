use crate::AdventError;
use std::{ops::RangeInclusive, str::FromStr};

pub trait ParseRangeInclusive<Idx> {
    fn parse(s: &str) -> Result<RangeInclusive<Idx>, AdventError>;
}

impl<Idx: FromStr> ParseRangeInclusive<Idx> for RangeInclusive<Idx> {
    fn parse(s: &str) -> Result<RangeInclusive<Idx>, AdventError> {
        let mut num_split = s.split("-").take(2);
        let (s, e) = (
            num_split
                .next()
                .ok_or_else(|| {
                    AdventError::InputParseError("missing numeric range start/end".into())
                })?
                .parse::<Idx>()
                .map_err(|_| AdventError::InputParseError("unable to parse start bound".into()))?,
            num_split
                .next()
                .ok_or_else(|| {
                    AdventError::InputParseError("missing numeric range start/end".into())
                })?
                .parse::<Idx>()
                .map_err(|_| AdventError::InputParseError("unable to parse end bound".into()))?,
        );

        Ok(RangeInclusive::new(s, e))
    }
}
