use rouille::{router, try_or_400};
use crate::models::data_structures::{Model, Rule};
use crate::models::{ParsedData, ParsedDataInterpret};
use crate::models::ParsedData::{Interpret, TRS};
use crate::parsers::{Parse, ParserTRS, ParserInterpret};

#[derive(Debug)]
#[derive(serde::Deserialize)]
struct InputJson {
    Interpretation: String,
    TRS: String,
}

#[derive(serde::Serialize)]
struct ErrorJson {
    #[serde(skip_serializing_if = "String::is_empty")]
    error_trs: String,

    #[serde(skip_serializing_if = "String::is_empty")]
    error_interpretation: String,
}

#[derive(serde::Serialize)]
struct ResponseJson {
    pub json_TRS: Vec<Rule>,
    pub json_interpret: ParsedDataInterpret,
}

pub fn handle_request(request: &rouille::Request) -> rouille::Response {
    let json: InputJson = try_or_400!(rouille::input::json_input(request));
    let mut err = ErrorJson { error_trs: "".to_string(), error_interpretation: "".to_string() };
    let mut res = ResponseJson{json_TRS: Vec::new(), json_interpret: Vec::new()};

    let mut parser_trs = ParserTRS::new(&json.TRS[..]);
    match parser_trs.parse() {
        Ok(result) => {
            println!("Парсинг TRS: {:?}", result);
            res.json_TRS = match result {
                TRS(trs) => {
                    trs.rules.clone()
                }
                _ => Vec::new()
            };
        }
        Err(e) => err.error_trs = e,
    };

    let mut parser_interpret = ParserInterpret::new(&json.Interpretation[..], Model {
        variables: parser_trs.variables,
        constants: parser_trs.constants,
        functions: parser_trs.functions,
    });
    match parser_interpret.parse() {
        Ok(result) => {
            println!("Парсинг Interpet: {:?}", result);
            res.json_interpret = match result {
                Interpret(interpret) => interpret,
                _ => Vec::new()
            };
        }
        Err(e) => err.error_interpretation = e,
    };
    if err.error_trs.len() > 0 || err.error_interpretation.len() > 0 {
        return rouille::Response::json(&err).with_status_code(400);
    }
    let resp = rouille::Response::json(&res);
    resp
    //rouille::Response::text(format!("field1's value is {}", json.Interpretation))
}