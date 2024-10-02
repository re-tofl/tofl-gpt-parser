use super::Parse;
use crate::models::Parser;

#[derive(Debug)]
pub struct ParsedDataTRS {}

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
    type Output = ParsedDataTRS;
    fn parse(&mut self, data: &str) -> Result<(Self::Output), String> {
        Ok((ParsedDataTRS {}))
    }
}
