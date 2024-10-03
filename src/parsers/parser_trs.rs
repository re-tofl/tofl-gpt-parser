use super::Parse;
use crate::models::{ParsedData, ParsedDataTRS, Parser};

pub struct ParserTRS {
    parser: Parser,
}

impl ParserTRS {
    pub fn new(input: &str) -> Self {
        ParserTRS {
            parser: Parser::new(input),
        }
    }
}

impl Parse for ParserTRS {
    fn parse(&mut self) -> Result<(ParsedData), String> {
        Ok((ParsedData::TRS(ParsedDataTRS{})))
    }
}
