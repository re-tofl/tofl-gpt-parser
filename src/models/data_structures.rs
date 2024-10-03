#[derive(Debug)]
pub struct Parser {
    input: Vec<char>,
    pos : u32,
    line : u32,
    pos_in_line : u32,
}

#[derive(Debug)]
pub enum ParsedData {
    Interpret(ParsedDataInterpret),
    TRS(ParsedDataTRS)
}

#[derive(Debug)]
pub struct ParsedDataInterpret {

}

#[derive(Debug)]
pub struct ParsedDataTRS {

}


impl Parser {
    pub fn new(input: &str) -> Self {
        Parser {
            input: input.chars().collect(),
            pos: 0,
            line: 1,
            pos_in_line: 0
        }
    }

    fn advance(&mut self) {
        //ignore spaces
        while self.pos < self.input.len() as u32 {
            let current = self.input[self.pos as usize];
            self.pos += 1;

            if current == ' ' || current == '\t' {
                self.pos_in_line += 1;
            } else {
                if current == '\n' {
                    self.line += 1;
                    self.pos_in_line = 0;
                } else {
                    self.pos_in_line += 1;
                }

                break;
            }
        }
    }

    pub fn peek(&mut self) -> Result<char, String> {
        while self.pos < self.input.len() as u32 {
            let current = self.input[self.pos as usize];

            if current == ' ' || current == '\t' {
                self.pos += 1;
            } else {
                return Ok(current);
            }
        }
        Err(format!("Unexpected EOF"))
    }

    pub fn next(&mut self) -> Result<char, String> {
        let current = self.peek();
        self.advance();
        current
    }

    pub fn read_exact_char(&mut self, expected: char) -> Result<(), String> {
        let current = self.next();
        if current.is_err() {
            return Err(format!("Unexpected EOF"));
        }

        let current_symbol = current.ok().unwrap();
        if  current_symbol == expected {
           Ok(())
        } else {
            Err(format!(
               "Ошибка на строке {}, позиции {}: ожидался символ '{}', но был '{}'",
                self.line, self.pos_in_line, expected, current_symbol
            ))
        }
    }
}