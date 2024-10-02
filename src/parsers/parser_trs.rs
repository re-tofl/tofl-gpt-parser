use super::Parse;
use crate::models::{ParsedData, ParsedDataTRS, Parser};

pub struct ParserTRS {
    parser: Parser,
}

impl ParserTRS {
    pub fn new() -> Self {
        ParserTRS {
            parser: Parser::new(),
        }
    }
}

impl Parse for ParserTRS {
    fn parse(&mut self, data: &str) -> Result<(ParsedData), String> {
        Ok((ParsedData::TRS(ParsedDataTRS{})))
    }
}
