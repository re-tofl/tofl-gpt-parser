use super::Parse;
use crate::models::{ParsedData, ParsedDataTRS, Parser};
use std::collections::HashSet;
use crate::models::data_structures::{Rule, Term};

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

    fn parse_rules(&mut self) -> Result<Vec<Rule>, String> {
        let mut rules: Vec<Rule> = Vec::new();

        loop {
            match self.parser.peek() {
                Ok(_) => {
                    let rule = self.parse_rule()?;
                    rules.push(rule);
                }
                Err(_) => break,
            }
        }

        if rules.is_empty() {
            return Err("rules not found".to_string());
        }
        Ok((rules))
    }

    fn parse_rule(&mut self) -> Result<Rule, String> {
        let lhs = self.parse_term()?;

        self.parser.read_exact_char('=')?;

        let rhs = self.parse_term()?;

        self.parser.read_eol()?;

        Ok(Rule{left: lhs, right: rhs})
    }
    /*
    variables = x,y
    f(x,h(y))=h(f(x,y))
    g = f
     */
    fn parse_term(&mut self) -> Result<Term, String> {

        self.parser.peek()?;
        let c = self.parser.next()?;
        if !c.is_alphabetic() {
            return Err(self.parser.format_error("буква".parse().unwrap()))
        }

        let mut term = Term {
            value: c.to_string(),
            childs: Vec::new(),
        };

        let symbol = self.parser.peek()?;

        if symbol == '(' {
            self.parser.read_exact_char('(')?;
            self.functions.insert(c.to_string());
            let args = self.parse_arg_list()?;
            self.parser.read_exact_char(')')?;
            term.childs = args;
        } else {
            if !self.variables.contains(&c.to_string()) {
                self.constants.insert(c.to_string());
            }

        }

        Ok(term)
    }

    fn parse_arg_list(&mut self) -> Result<Vec<Term>, String> {
        let mut args: Vec<Term> = Vec::new();

        args.push(self.parse_term()?);

        while self.parser.peek()? == ',' {
            self.parser.next()?;
            args.push(self.parse_term()?);
        }

        Ok(args)
    }
}

impl Parse for ParserTRS {
    fn parse(&mut self) -> Result<(ParsedData), String> {

        self.parse_variables()?;

        let rules = self.parse_rules()?;

        Ok((ParsedData::TRS(ParsedDataTRS {
            rules,
            variables: self.variables.clone(),
            constants: self.constants.clone(),
            functions: self.functions.clone(),
        })))
    }
}
