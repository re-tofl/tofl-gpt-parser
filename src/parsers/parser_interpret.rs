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
        result.push(self.parse_function_or_const()?);
        self.parser.read_eol()?;

        loop {
            match self.parser.peek() {
                Ok(_) => {
                    let func = self.parse_function_or_const()?;
                    result.push(func);
                    self.parser.read_eol()?;
                }
                Err(_) => break,
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

        //skip =
        self.parser.read_exact_char('=')?;

        let expression = self.parse_polynomial(&variables)?;

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
            let digit = self.parser.peek()?;

            if !digit.is_ascii_digit() {
                let number_string = number.join("");

                if number_string == "0" {
                    return Err("Number can't be zero".to_string())
                };

                return Ok(number_string);
            }

            let _ = self.parser.next();
            number.push(digit.to_string());
        }
    }

    fn parse_variable(&mut self) -> Result<String, String> {
        let name = self.parser.next()?;

        //TODO check name is variable
        Ok(name.to_string())
    }

    fn parse_function_arguments(&mut self) -> Result<HashSet<String>, String> {
        let mut variables = HashSet::new();
        variables.insert(self.parse_variable()?.to_string());

        loop {
            let punctuation = self.parser.next()?;

            if punctuation == ')' {
                //TODO check the number or variables
                return Ok(variables);
            } else if punctuation != ',' {
                return Err(format!("Expected ',' or ')',  got '{}'", punctuation));
            }

            //TODO check that variables don't repeat
            variables.insert(self.parse_variable()?.to_string());
        }
    }

    fn parse_polynomial(&mut self, variables: &HashSet<String>) -> Result<String, String> {
        let mut polynomial_parts = Vec::new();
        polynomial_parts.push(self.parse_monomial(variables)?.to_string());

        loop {
            let punctuation = self.parser.peek()?.to_string();

            if punctuation == "\r" || punctuation == "\n" {
                return Ok(polynomial_parts.join(" + "));
            } else if punctuation != "+" {
                return Err(format!("Expected '+' or eol,  got '{}'", punctuation));
            } else {
                self.parser.read_exact_char('+')?;
                polynomial_parts.push(self.parse_monomial(variables)?);
            }
        }
    }

    fn parse_monomial(&mut self, variables: &HashSet<String>) -> Result<String, String> {
        let mut monomial_parts = Vec::new();
        let mut coefficient = String::new();  // Теперь это будет String
        let mut symbol = self.parser.peek()?;

        if symbol.is_ascii_digit() {
            coefficient = self.parse_number_string()?;  // Присваиваем String, а не ссылку

            symbol = self.parser.peek()?;
            if symbol != '*' {
                return Ok(coefficient);  // Возвращаем строку напрямую
            }

            self.parser.read_exact_char('*')?;
        }

        let variable = self.parse_variable()?;
        // TODO check variables contains variable
        let mut degree = String::new();  // Используем String

        symbol = self.parser.peek()?;
        if symbol == '{' {
            degree = self.parse_degree()?;  // Присваиваем String
        }

        monomial_parts.push(build_monomial(&coefficient, &variable, &degree));  // Передаём ссылки на строки

        loop {
            symbol = self.parser.peek()?;
            if symbol == '+' || symbol == '\n' || symbol == '\r' {
                return Ok(monomial_parts.join(" * "));  // Возвращаем соединённые части монома
            }

            coefficient.clear();  // Очищаем String вместо переприсваивания
            degree.clear();  // Очищаем degree

            if symbol.is_ascii_digit() {
                coefficient = self.parse_number_string()?;  // Присваиваем новое значение
                self.parser.read_exact_char('*')?;
            }

            let variable = self.parse_variable()?;
            // TODO check variables contains variable

            symbol = self.parser.peek()?;
            if symbol == '{' {
                degree = self.parse_degree()?;  // Присваиваем новое значение для degree
            }

            monomial_parts.push(build_monomial(&coefficient, &variable, &degree));  // Передаём ссылки
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

