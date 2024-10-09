use super::Parse;
use crate::models::{ParsedData, ParsedDataInterpret, Parser};
use crate::models::data_structures::Model;

#[derive(Debug)]
pub struct ParserInterpret {
    parser: Parser,
    model: Model,
}

impl ParserInterpret {
    pub fn new(input: &str, model: Model) -> Self {
        ParserInterpret {
            parser: Parser::new(input),
            model,
        }
    }
}

impl Parse for ParserInterpret {
    fn parse(&mut self) -> Result<(ParsedData), String> {
        Ok((ParsedData::Interpret(ParsedDataInterpret::new())))
    }
}