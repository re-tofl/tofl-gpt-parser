use crate::parsers::{Parse, ParserTRS, ParserInterpret};
pub fn handle_request() {

    let data = "";

    // Инициализация парсеров
    let mut parser_trs = ParserTRS::new();
    let mut parser_interpret = ParserInterpret::new();

    // Парсер TRS
    match parser_trs.parse(data) {
        Ok(result) => println!("Результат парсинга TRS: {:?}", result),
        Err(e) => println!("Ошибка в парсере TRS: {}", e),
    }

    // Парсер Interpret
    match parser_interpret.parse(data) {
        Ok(result) => println!("Результат парсинга Interpret: {:?}", result),
        Err(e) => println!("Ошибка в парсере Interpret: {}", e),
    }
}