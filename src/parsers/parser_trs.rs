use super::Parse;
use crate::models::{ParsedData, ParsedDataTRS, Parser};
use std::collections::{HashMap, HashSet};
use crate::models::data_structures::{Rule, Term, Types};

#[derive(Debug)]
pub struct ParserTRS {
    parser: Parser,
    pub variables: HashSet<char>,
    pub constants: HashSet<char>,
    pub functions: HashMap<char, i32>,
}

impl ParserTRS {
    pub fn new(input: &str) -> Self {
        ParserTRS {
            parser: Parser::new(input),
            variables: HashSet::new(),
            constants: HashSet::new(),
            functions: HashMap::new(),
        }
    }

    fn parse_variables(&mut self) -> Result<(), String> {
        let expected = "variables";
        match self.parser.peek(){
            Ok(_) => (),
            Err(_) => return Err(self.parser.format_eof_error("объявление переменных (variables=...)".to_string()))
        }

        for c in expected.chars() {
            let peeked: char;
            match self.parser.peek_without_skipping(){
                Ok(received) => peeked = received,
                Err(_) => return Err(self.parser.format_eof_error(c.to_string()))
            }
            if self.parser.peek_without_skipping()? != c {
                return Err(self.parser.format_error(c.to_string()));
            }
            self.parser.next()?;
        }
        // Non-fatal check (if = sign is missing => accumulate error and then
        // parse list of variables
        match self.parser.read_exact_char('='){
            Ok(_) => (),
            Err(e) =>{
                let pos = self.parser.format_position();

                self.parser.add_error(format!("{}Не хватает '=' в списке переменных",
                pos));
            }
        };
        loop {
            let peeked= match self.parser.peek(){
                Ok(received) => received,
                Err(_) => return Err(self.parser.format_eof_error("переменная".to_string())),
            };
            if peeked.is_alphabetic() {
                let current_variable = self.parser.next()?;
                // Non-fatal, accumulate error, no extra behaviour is necessary
                if !self.variables.insert(current_variable) {
                    return Err(format!("Переменная {} объявлена несколько раз", current_variable));
                }
            } else {
                break;
            }
            let after_var: char;
            match self.parser.peek(){
                Ok(received) => after_var = received,
                Err(_) => return Err(self.parser.format_eof_error("',' или конец строки".to_string())),
            }
            if after_var == ',' {
                self.parser.next()?;
            } else {
                break;
            }
        }
        self.parser.read_eol()?;
        if self.variables.is_empty() {
            return Err(format!("{}У функции не найдено ни одной переменной", self.parser.format_position()));
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
            // TODO(Переписать ошибку на русский)
            return Err("rules not found".to_string());
        }
        Ok((rules))
    }

    fn parse_rule(&mut self) -> Result<Rule, String> {
        let lhs = self.parse_term()?;

        self.parser.read_exact_char('=')?;

        let rhs = self.parse_term()?;

        self.parser.read_eol()?;

        Ok(Rule { left: lhs, right: rhs })
    }
    /*
    variables = x,y
    f(x,h(y))=h(f(x,y))
    g = f
     */
    fn parse_term(&mut self) -> Result<Term, String> {
        let c : char;
        match self.parser.peek(){
            Ok(received) => c = received,
            Err(_) => return Err(self.parser.format_eof_error("терм".to_string()))
        }

        if !c.is_alphabetic() {
            return Err(self.parser.format_error("буква".parse().unwrap()));
        }
        self.parser.next()?;
        let mut term = Term {
            value: c.to_string(),
            childs: Vec::new(),
        };

        let symbol = match self.parser.peek() {
            Ok(val) => { val }
            Err(_) => return Ok(term),
        };

        if symbol == '(' {
            if self.variables.contains(&c) {
                return Err(self.parser.format_type_error(Types::FUNCTION, Types::VARIABLE));
            }
            if self.constants.contains(&c) {
                return Err(self.parser.format_type_error(Types::FUNCTION, Types::CONSTANT));
            }
            self.parser.read_exact_char('(')?;
            if !self.functions.contains_key(&c) {
                self.functions.insert(c, -1);
            }
            let args = self.parse_arg_list()?;

            if *self.functions.get(&c).unwrap() == -1 {
                self.functions.insert(c, args.len() as i32);
            } else if *self.functions.get(&c).unwrap() != args.len() as i32 {
                return Err(self.parser.format_arity_error(c, self.functions.get(&c).unwrap().to_string(), args.len().to_string()));
            }
            self.parser.read_exact_char(')')?;
            term.childs = args;
        } else {
            if self.functions.contains_key(&c) {
                return Err(self.parser.format_type_error(Types::ConstantOrVariable, Types::FUNCTION));
            }
            if !self.variables.contains(&c) {
                self.constants.insert(c);
            }
        }

        Ok(term)
    }

    fn parse_arg_list(&mut self) -> Result<Vec<Term>, String> {
        let mut args: Vec<Term> = Vec::new();

        args.push(self.parse_term()?);

        while match self.parser.peek() {
            Ok(received) => received,
            Err(_) => return Err(self.parser.format_eof_error("','".to_string())),
        } == ',' {
            self.parser.next()?;
            args.push(self.parse_term()?);
        }

        Ok(args)
    }
}

impl Parse for ParserTRS {
    fn parse(&mut self) -> Result<(ParsedData), Vec<String>> {
        match self.parse_variables(){
            Ok(_) => (),
            Err(e) => {
                self.parser.add_error(e);
                return Err(self.parser.get_errors());
            },
        };

        let rules = match self.parse_rules(){
            Ok(rules) => rules,
            Err(e) => {
                self.parser.add_error(e);
                return Err(self.parser.get_errors());
            },
        };

        Ok((ParsedData::TRS(ParsedDataTRS {
            rules,
            variables: self.variables.clone(),
            constants: self.constants.clone(),
            functions: self.functions.clone(),
        })))
    }
}
