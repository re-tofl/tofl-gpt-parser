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

    pub fn peek(&mut self) -> Option<char> {
        while self.pos < self.input.len() as u32 {
            let current = self.input[self.pos as usize];

            if current == ' ' || current == '\t' {
                self.pos += 1;
            } else {
                return Some(current);
            }
        }
        None
    }

    pub fn next(&mut self) -> Option<char> {
        let current = self.peek();
        self.advance();
        current
    }
}