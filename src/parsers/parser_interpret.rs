use std::string::String;
use std::collections::HashSet;
use super::Parse;
use crate::models::{ParsedData, ParsedDataInterpret, Parser};
use crate::models::data_structures::{Model, ParsedInterpretFunction};

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
        let mut result = Vec::new();

        loop {
            result.push(self.parse_function_or_const()?);

            match self.parser.peek() {
                Err(_) => break,
                Ok(_) => {
                    self.parser.read_eol()?;

                    match self.parser.peek() {
                        Err(_) => break,
                        Ok(_) => {}
                    }
                }
            }
        }

        Ok(ParsedData::Interpret(result))
        //TODO check that all of the constants and functions are declared
    }
}

impl ParserInterpret {
    fn parse_function_or_const(&mut self) -> Result<ParsedInterpretFunction, String> {
        let name = self.parser.peek()?;

        if self.model.functions.contains_key(&name) {
            return self.parse_function()
        } else if self.model.constants.contains(&name) {
            return self.parse_constant()
        }

        Err(format!("expected name of function or constant, got {}", name))
    }

    fn parse_function(&mut self) -> Result<ParsedInterpretFunction, String> {
        let name = self.parser.next()?;
        //TODO check func name

        //skip (
        self.parser.read_exact_char('(')?;

        let variables = self.parse_function_arguments()?;
        //TODO check that func has right number of arguments

        //skip =
        self.parser.read_exact_char('=')?;

        let expression = self.parse_polynomial_expression(&variables)?;

        Ok(ParsedInterpretFunction{
            name: name.to_string(),
            variables: variables.into_iter().collect(),
            expression: expression.to_string(),
        })
    }

    fn parse_constant(&mut self) -> Result<ParsedInterpretFunction, String> {
        let name = self.parser.next()?;
        //TODO check const name

        self.parser.read_exact_char('=')?;

        let number = self.parse_number_string()?;

        Ok(ParsedInterpretFunction{
            name: name.to_string(),
            variables: Vec::new(),
            expression: format!("({})", number.to_string()),
        })
    }

    fn parse_number_string(&mut self) -> Result<String, String> {
        let mut number = Vec::new();
        loop {
            match self.parser.peek() {
                Err(_) => break,
                Ok(digit) => {
                    if !digit.is_ascii_digit() {
                        break;
                    }

                    self.parser.next()?;
                    number.push(digit.to_string());
                }
            }
        }

        let number_string = number.join("");

        if number_string == "0" {
            return Err("Number can't be zero".to_string())
        };

        Ok(number_string)
    }

    fn parse_variable(&mut self) -> Result<String, String> {
        let name = self.parser.next()?;

        if !name.is_alphabetic(){
            return Err(format!("Expected alphabetic name, got {}", name))
        }

        Ok(name.to_string())
    }

    fn parse_function_arguments(&mut self) -> Result<HashSet<String>, String> {
        let mut variables = HashSet::new();

        loop {
            //TODO check that variable name doesn't match name of function or const
            variables.insert(self.parse_variable()?.to_string());

            let punctuation = self.parser.next()?;

            if punctuation == ')' {
                //TODO check the number or variables
                return Ok(variables);
            } else if punctuation != ',' {
                return Err(format!("Expected ',' or ')',  got '{}'", punctuation));
            }
        }
    }

    fn parse_polynomial_expression(&mut self, variables: &HashSet<String>) -> Result<String, String> {
        let mut polynomial_parts = Vec::new();

        loop {
            polynomial_parts.push(self.parse_monomial(variables)?);

            match self.parser.peek() {
                Err(_) => break,
                Ok(punctuation) => {
                    if punctuation == '\n' || punctuation == '\r' {
                        break;
                    } else if punctuation != '+' {
                        return Err(format!("Expected '+' or eol,  got '{}'", punctuation));
                    } else {
                        self.parser.read_exact_char('+')?;
                    }
                }
            }
        }

        Ok(format!("({})", polynomial_parts.join(" + ")))
    }

    fn parse_monomial(&mut self, variables: &HashSet<String>) -> Result<String, String> {
        let mut monomial_parts = Vec::new();
        let mut coefficient = String::new();
        let mut symbol = self.parser.peek()?;

        if symbol.is_ascii_digit() {
            coefficient = self.parse_number_string()?;

            match self.parser.peek() {
                Err(_) => return Ok(coefficient),
                Ok(symbol) => {
                    if symbol != '*' {
                        return Ok(coefficient);
                    }

                    self.parser.read_exact_char('*')?;
                }
            }
        }

        let mut variable = self.parse_variable()?;
        // TODO check arguments contain variable
        let mut degree = String::new();

        loop {
            match self.parser.peek() {
                Err(_) => return Ok(build_monomial(&coefficient, &variable, &degree)),
                Ok(symbol) => {
                    if symbol == '{' {
                        degree = self.parse_degree()?;
                    }
                }
            }

            monomial_parts.push(build_monomial(&coefficient, &variable, &degree));

            match self.parser.peek() {
                Err(_) => return Ok(monomial_parts.join(" * ")),
                Ok(picked_symbol) => {
                    symbol = picked_symbol;

                    if symbol == '+' || symbol == '\n' || symbol == '\r' {
                        return Ok(monomial_parts.join(" * "));
                    }
                }
            }

            coefficient.clear();
            degree.clear();

            if symbol.is_ascii_digit() {
                coefficient = self.parse_number_string()?;
                self.parser.read_exact_char('*')?;
            }

            variable = self.parse_variable()?;
            // TODO check arguments contains variable
        }
    }

    fn parse_degree(&mut self) -> Result<String, String> {
        self.parser.read_exact_char('{')?;
        let degree = self.parse_number_string()?;
        self.parser.read_exact_char('}')?;

        Ok(degree)
    }
}

fn build_monomial (coefficient: &String, variable: &String, degree: &String) -> String {
    let mut monomial = variable.clone();

    if coefficient != "" {
        monomial = format!("{} * {}", coefficient, monomial);
    }

    if degree != "" {
        monomial = format!("{}^{}", monomial, degree);
    }

    monomial
}

