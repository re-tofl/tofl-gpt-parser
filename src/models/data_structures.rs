use std::collections::{HashMap, HashSet};

#[derive(Debug)]
pub struct Parser {
    input: Vec<char>,
    pos: u32,
    line: u32,
    pos_in_line: u32,
    prev_line: u32,
    prev_pos_in_line: u32,
    errors: Vec<String>,
}

#[derive(Debug)]
pub enum ParsedData {
    Interpret(ParsedDataInterpret),
    TRS(ParsedDataTRS),
}

pub type ParsedDataInterpret = Vec<ParsedInterpretFunction>;

#[derive(Debug)]
#[derive(serde::Serialize)]
pub struct ParsedInterpretFunction {
    pub(crate) name: String,
    pub(crate) variables: Vec<String>,
    pub(crate) expression: String,
}

#[derive(Debug)]
pub struct ParsedDataTRS {
    pub rules: Vec<Rule>,
    pub variables: HashSet<char>,
    pub constants: HashSet<char>,
    pub functions: HashMap<char, i32>,
}

#[derive(Debug)]
pub struct Model {
    pub variables: HashSet<char>,
    pub constants: HashSet<char>,
    pub functions: HashMap<char, i32>,
}

#[derive(Debug)]
#[derive(serde::Serialize)]
#[derive(Clone)]
pub struct Rule {
    pub left: Term,
    pub right: Term,
}

#[derive(Debug, Clone)]
#[derive(serde::Serialize)]
pub struct Term {
    pub value: String,
    pub childs: Vec<Term>,
}

pub enum Types {
    CONSTANT,
    VARIABLE,
    FUNCTION,
    ConstantOrVariable,
}

impl Types {
    fn as_text(&self) -> &str {
        match self {
            Types::CONSTANT => "константа",
            Types::VARIABLE => "переменная",
            Types::FUNCTION => "функция",
            Types::ConstantOrVariable => "константа или переменная"
        }
    }
}

impl Parser {
    pub fn new(input: &str) -> Self {
        Parser {
            input: input.chars().collect(),
            pos: 0,
            line: 1,
            pos_in_line: 0,
            prev_pos_in_line: 0,
            prev_line: 0,
            errors: Vec::new(),
        }
    }

    fn advance(&mut self) {
        //ignore spaces
        while self.pos < self.input.len() as u32 {
            let current = self.input[self.pos as usize];
            self.pos += 1;

            match current {
                ' ' | '\t' => {
                    self.pos_in_line += 1;
                }
                '\r' => {
                    self.pos_in_line = 0;
                    break;
                }
                '\n' => {
                    self.line += 1;
                    self.pos_in_line = 0;
                    break;
                }
                _ => {
                    self.pos_in_line += 1;
                    break;
                }
            }
        }
    }

    pub fn peek(&mut self) -> Result<char, String> {
        while self.pos < self.input.len() as u32 {
            let current = self.input[self.pos as usize];

            if current == ' ' || current == '\t' {
                self.pos += 1;
                self.pos_in_line += 1;
            } else {
                return Ok(current);
            }
        }
        Err(format!("Unexpected EOF"))
    }

    pub fn peek_without_skipping(&mut self) -> Result<char, String> {
        if self.pos < self.input.len() as u32 {
            Ok(self.input[self.pos as usize])
        } else {
            Err(format!("Unexpected EOF"))
        }
    }

    pub fn next(&mut self) -> Result<char, String> {
        self.prev_pos_in_line = self.pos_in_line;
        self.prev_line = self.line;
        let current = self.peek();
        self.advance();
        current
    }

    pub fn read_exact_char(&mut self, expected: char) -> Result<(bool), String> {
        let start_pos = self.pos;
        let current: char;
        match self.peek() {
            Ok(c) => current = c,
            Err(e) => return Err(self.format_eof_error(expected.to_string())),
        }
        if current == expected {
            self.next()?;
            // Возвращаем true, если были считаны пробельные символы
            Ok(start_pos != self.pos - 1)
        } else {
            Err(self.format_error(expected.to_string()))
        }
    }

    pub fn read_eol(&mut self) -> Result<(), String> {
        let current = self.peek();
        match current {
            Ok(_) => {
                match current? {
                    '\n' => {
                        self.read_exact_char('\n')?;
                        Ok(())
                    }
                    '\r' => {
                        self.read_exact_char('\r')?;
                        match self.peek() {
                            Err(_) => Ok(()),
                            Ok(symbol) => {
                                if symbol == '\n' {
                                    self.read_exact_char('\n')?;
                                    return Ok(());
                                };

                                Ok({})
                            }
                        }
                    }
                    _ => Err(self.format_error("eol".parse().unwrap())),
                }
            }
            Err(_) => Ok(())
        }
    }

    pub fn get_errors(&mut self) -> Vec<String> {
        return self.errors.clone();
    }

    pub fn add_error(&mut self, message: String) {
        self.errors.push(message);
    }

    pub fn format_position(&mut self) -> String {
        format!("Ошибка в строке {}, на позиции {}. ",
                self.line, self.pos_in_line)
    }

    pub fn format_previous_position(&mut self) -> String {
        format!("Ошибка в строке {}, на позиции {}. ",
                self.prev_line, self.prev_pos_in_line)
    }

    pub fn format_error(&mut self, expected: String) -> String {
        format!("Ошибка в строке {}, на позиции {}, ожидалось {}, считано '{}'",
                self.line, self.pos_in_line, expected, self.input[self.pos as usize])
    }

    pub fn format_eof_error(&mut self, expected: String) -> String {
        format!("Ошибка в строке {}, на позиции {}, ожидалось: {}, считано EOF",
                self.line, self.pos_in_line, expected)
    }

    pub fn format_arity_error(&mut self, function: char, expected: String, received: String) -> String {
        format!("Не совпадает арность функции {}, ожидаемое количество аргументов: {} , считано: {}",
                function, expected, received)
    }

    pub fn format_type_error(&mut self, expected: Types, received: Types) -> String {
        format!("Ошибка в строке {}, на позиции {}, ожидалась {}, считана {}",
                self.prev_line, self.prev_pos_in_line, expected.as_text(), received.as_text())
    }

    pub fn format_variables_count_error(&mut self, wrong_variables: HashSet<char>) -> String {
        let wrong_variables_as_string: String = wrong_variables
            .iter()
            .map(|c| c.to_string())
            .collect::<Vec<String>>()
            .join(", ");
        format!("Ошибка в строке {}, следующие переменные входят в правую часть, но не входят в левую: {}", self.line, wrong_variables_as_string)
    }
}