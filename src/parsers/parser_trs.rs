use super::Parse;
use crate::models::{ParsedData, ParsedDataTRS, Parser};
use std::collections::HashSet;

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

    fn parse_variables(&mut self) -> Result<(), String> {
        let expected = "variables=";
        self.parser.peek()?;
        for c in expected.chars() {
            self.parser.read_exact_char(c)?;
        }
        loop {
            let var = self.parser.peek();
            if (!var?.is_alphabetic()) {

            }
        }
        if (self.variables.is_empty()) {
            return Err("variables not found".to_string());
        }
        Ok(())
    }

    fn parse_variable(&mut self) -> Result<String, String> {

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
