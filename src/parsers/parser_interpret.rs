use super::Parse;
use crate::models::Parser;


#[derive(Debug)]
pub struct ParsedDataInterpret {

}
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
    type Output = ParsedDataInterpret;
    fn parse(&mut self, data: &str) -> Result<(Self::Output), String> {
        Ok((ParsedDataInterpret {}))
    }
}