use std::string::String;
use std::collections::{HashMap, HashSet};
use super::Parse;
use crate::models::{ParsedData, Parser};
use crate::models::data_structures::{Model, ParsedInterpretFunction, Types};

#[derive(Debug)]
pub struct ParserInterpret {
    parser: Parser,
    model_from_trs: Model,
    own_functions: HashMap<char,i32>,
    own_constants: HashSet<char>,
}

impl ParserInterpret {
    pub fn new(input: &str, model: Model) -> Self {
        ParserInterpret {
            parser: Parser::new(input),
            model_from_trs: model,
            own_functions: HashMap::new(),
            own_constants: HashSet::new(),
        }
    }
}

impl Parse for ParserInterpret {
    fn parse(&mut self) -> Result<(ParsedData), Vec<String>> {
        let mut result = Vec::new();

        loop {
            let res1 = match self.parse_function_or_const(){
                Ok(r) => r,
                Err(e) => {
                    self.parser.add_error(e);
                    return Err(self.parser.get_errors());
                }
            };
            result.push(res1);

            match self.parser.peek() {
                Err(_) => break,
                Ok(_) => {
                    match self.parser.read_eol(){
                        Ok(_) => (),
                        Err(e) => {
                            self.parser.add_error(e);
                            return Err(self.parser.get_errors());
                        }
                    }

                    match self.parser.peek() {
                        Err(_) => break,
                        Ok(_) => {}
                    }
                }
            }
        }

        for (k, v) in &self.model_from_trs.functions {
            match self.own_functions.get(k) {
                None => {
                    self.parser.add_error(format!("Функция {} была объявлена в TRS, но её нет в интерпретации", k));
                }
                Some(_) => ()
            }
        } //non fatal

        for v in &self.model_from_trs.constants {
            if !self.own_constants.contains(v){
                self.parser.add_error(format!("Константа {} была объявлена в TRS, но её нет в интерпретации", v))
            }
        } //non fatal

        if !self.parser.get_errors().is_empty() {
            return Err(self.parser.get_errors());
        }

        Ok(ParsedData::Interpret(result))
    }
}

impl ParserInterpret {
    fn parse_function_or_const(&mut self) -> Result<ParsedInterpretFunction, String> {
        let name= match self.parser.peek(){
            Ok(received) => received,
            Err(_) => return Err(self.parser.format_eof_error("функция или константа".to_string()))
        };
        if self.model_from_trs.functions.contains_key(&name) {
            return self.parse_function()
        } else if self.model_from_trs.constants.contains(&name) {
            return self.parse_constant()
        }

        Err(format!("{}Ожидалась функция или константа, считано {}", self.parser.format_position(), name))
    }

    fn parse_function(&mut self) -> Result<ParsedInterpretFunction, String> {
        let name = match self.parser.next(){
            Ok(received) => received,
            Err(_) => return Err(self.parser.format_eof_error("функция".to_string()))
        };
        if !self.model_from_trs.functions.contains_key(&name) {
            self.parser.add_error(format!("Функция {} не объявлена в TRS", name));
        } // non fatal

        //skip (
        self.parser.read_exact_char('(')?;

        let (variables, num_of_variables) = self.parse_function_arguments()?;
        if num_of_variables != *self.model_from_trs.functions.get(&name).unwrap() {
            let pos = self.parser.format_position();
            self.parser.add_error(format!("{}Количество переменных в интерпретации функции {} не совпадает с количеством переменных в TRS",
                                          pos, name));
        } // non fatal

        //skip =
        self.parser.read_exact_char('=')?;

        let expression = self.parse_polynomial_expression(&variables)?;

        self.own_functions.insert(name, num_of_variables);

        Ok(ParsedInterpretFunction{
            name: name.to_string(),
            variables: variables.into_iter().collect(),
            expression: expression.to_string(),
        })
    }

    fn parse_constant(&mut self) -> Result<ParsedInterpretFunction, String> {
        let name = match self.parser.next(){
            Ok(received) => received,
            Err(_) => return Err(self.parser.format_eof_error("константа".to_string()))
        };
        if !self.model_from_trs.constants.contains(&name) {
            self.parser.add_error(format!("Константы {} нет в TRS, но она присутствует в интерпретации", name));
        } //non fatal

        self.parser.read_exact_char('=')?;

        let number = self.parse_number_string()?;

        self.own_constants.insert(name);

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
            return Err(format!("{}Коэффициент не может быть равен 0", self.parser.format_position()));
        };

        Ok(number_string)
    }

    fn parse_variable(&mut self) -> Result<String, String> {
        let name = match self.parser.next(){
            Ok(received) => received,
            Err(_) => return Err(self.parser.format_eof_error("переменная".to_string()))
        };

        if !name.is_alphabetic() {
            return Err(format!("{}Ожидался символ алфавита (буква) в названии переменной, считано: {}",
                               self.parser.format_position(), name))
        }

        Ok(name.to_string())
    }

    fn parse_function_arguments(&mut self) -> Result<(HashSet<String>, i32), String> {
        let mut variables = HashSet::new();
        let mut num_of_variables = 0;
        loop {
            let current = self.parse_variable()?.to_string();
            if self.model_from_trs.functions.contains_key(&current.chars().nth(0).unwrap()){
                let e = self.parser.format_type_error(Types::VARIABLE, Types::FUNCTION);
                self.parser.add_error(e);
            } else if self.own_constants.contains(&current.chars().nth(0).unwrap()){
                let e = self.parser.format_type_error(Types::VARIABLE, Types::CONSTANT);
                self.parser.add_error(e);
            } //non fatal
            variables.insert(current);
            num_of_variables += 1;
            let punctuation = match self.parser.next(){
                Ok(received) => received,
                Err(_) => return Err(self.parser.format_eof_error("')' или ','".to_string()))
            };

            if punctuation == ')' {
                return Ok((variables, num_of_variables));
            } else if punctuation != ',' {
                return Err(format!("{}Ожидалось ',' или ')', считано '{}'",
                                    self.parser.format_position(), punctuation));
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
                        return Err(format!("{}Ожидалось '+' or eol, считано '{}'",
                                           self.parser.format_position(), punctuation));
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
        let mut symbol : char;
        match self.parser.peek() {
            Ok(c) => symbol = c,
            Err(_) => return Err(self.parser.format_eof_error("описание монома".to_string())),
        }

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

        let mut variable = String::new();
        let mut degree = String::new();
        loop {
            match self.parse_variable() {
                Ok(name) => variable = name,
                Err(e) => if coefficient == "" {
                    return Err(format!("{}Ожидался коэффицент, имя переменной, '+' или перевод строки", self.parser.format_position()))
                } else {
                    return Err(e)
                }
            }

            if !variables.contains(&variable) {
                return Err(format!("{}Переменная {} не указана в качестве аргумента функции", self.parser.format_position(), variable));
            }

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

