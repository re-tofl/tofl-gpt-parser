use super::Parse;
use crate::models::{ParsedData, ParsedDataInterpret, Parser};


#[derive(Debug)]
pub struct ParserInterpret {
    parser: Parser,
}

impl ParserInterpret {
    pub fn new() -> Self {
        ParserInterpret {
            parser: Parser::new(),
        }
    }
}

impl Parse for ParserInterpret {
    fn parse(&mut self, data: &str) -> Result<(ParsedData), String> {
        Ok((ParsedData::Interpret(ParsedDataInterpret{})))
    }
}