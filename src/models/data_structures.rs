#[derive(Debug)]
pub struct Parser {
    pos : i32,
    line : i32,
    pos_in_line : i32,
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
    pub fn new() -> Self {
        Parser {
            pos: 0,
            line: 1,
            pos_in_line: 0
        }
    }
}