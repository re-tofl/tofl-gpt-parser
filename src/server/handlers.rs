use crate::parsers::{Parse, ParserTRS, ParserInterpret};
pub fn handle_request() {

    let data = "";

    let mut parser_trs = ParserTRS::new(data);
    let mut parser_interpret = ParserInterpret::new(data);

    match parser_trs.parse(data) {
        Ok(result) => println!("Парсинг TRS: {:?}", result),
        Err(e) => println!("Ошибка в парсере TRS: {}", e),
    }

    match parser_interpret.parse(data) {
        Ok(result) => println!("Парсинг Interpet: {:?}", result),
        Err(e) => println!("Ошибка в парсере Interpret: {}", e),
    }
}