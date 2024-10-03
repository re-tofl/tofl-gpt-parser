use std::collections::HashSet;
use std::string::ParseError;
use super::Parse;
use crate::models::{ParsedData, ParsedDataTRS, Parser};

pub struct ParserTRS {
    parser: Parser,
    variables: HashSet<String>,
    constants: HashSet<String>,
    functions: HashSet<String>,
}

impl ParserTRS {
    pub fn new(input: &str) -> Self {
        ParserTRS {
            parser: Parser::new(input),
            variables: HashSet::new(),
            constants: HashSet::new(),
            functions: HashSet::new(),
        }
    }

    fn parse_variables(&mut self) -> Result<(), ParseError> {
        loop {}
    }
}

impl Parse for ParserTRS {
    fn parse(&mut self) -> Result<(ParsedData), String> {
        Ok((ParsedData::TRS(ParsedDataTRS {
            rules: vec![],
            variables: *self.variables,
            constants: *self.constants,
            functions: *self.functions,
        })))
    }
}
