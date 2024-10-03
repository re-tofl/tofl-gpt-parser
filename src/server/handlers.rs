use rouille::{router, try_or_400};
use crate::models::ParsedData;
use crate::parsers::{Parse, ParserTRS, ParserInterpret};

#[derive(Debug)]
#[derive(serde::Deserialize)]
struct InputJson {
    Interpretation: String,
    TRS: String,
}

#[derive(serde::Serialize)]
struct ErrorJson{
    #[serde(skip_serializing_if = "String::is_empty")]
    error_trs:String,

    #[serde(skip_serializing_if = "String::is_empty")]
    error_interpretation:String,
}

pub fn handle_request(request : &rouille::Request) -> rouille::Response {
    let json: InputJson = try_or_400!(rouille::input::json_input(request));

    let mut parser_trs = ParserTRS::new();
    let mut parser_interpret = ParserInterpret::new();
    let mut response : rouille::Response;

    let mut res : ParsedData;
    let mut err = ErrorJson{ error_trs: "".to_string(), error_interpretation: "".to_string() };
    
    match parser_trs.parse(&json.TRS[..]) {
        Ok(result) => println!("Парсинг TRS: {:?}", result),
        Err(e) => err.error_trs = e,
    };
    match parser_interpret.parse(&json.Interpretation[..]) {
        Ok(result) => println!("Парсинг Interpet: {:?}", result),
        Err(e) => err.error_interpretation = e,
    };


    if err.error_trs.len() > 0 || err.error_interpretation.len() > 0 {
        return rouille::Response::json(&err).with_status_code(400);
    }
    rouille::Response::text(format!("field1's value is {}", json.Interpretation))
}