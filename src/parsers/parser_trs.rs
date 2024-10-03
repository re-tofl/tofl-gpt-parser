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
        let expected = "variables";
        self.parser.peek()?;
        for c in expected.chars() {
            if self.parser.read_exact_char(c)? {
                return Err(self.parser.format_error(c.to_string()));
            }
        }
        self.parser.read_exact_char('=')?;
        loop {
            if self.parser.peek()?.is_alphabetic() {
                self.variables.insert(String::from(self.parser.next()?));
            } else {
                break;
            }
            let after_var = self.parser.peek()?;
            if after_var == ',' {
                self.parser.next()?;
            } else {
                break
            }
        }
        self.parser.read_eol()?;
        if self.variables.is_empty() {
            return Err("variables not found".to_string());
        }
        Ok(())
    }

}

impl Parse for ParserTRS {
    fn parse(&mut self) -> Result<(ParsedData), String> {

        self.parse_variables()?;
        Ok((ParsedData::TRS(ParsedDataTRS {
            rules: vec![],
            variables: self.variables.clone(),
            constants: self.constants.clone(),
            functions: self.functions.clone(),
        })))
    }
}
