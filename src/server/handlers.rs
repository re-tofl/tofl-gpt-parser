use rouille::{try_or_400};
use crate::models::data_structures::{Model, Rule};
use crate::models::{ParsedDataInterpret};
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
    #[serde(skip_serializing_if = "Vec::is_empty")]
    error_trs: Vec<String>,

    #[serde(skip_serializing_if = "Vec::is_empty")]
    error_interpretation: Vec<String>,
}

#[derive(serde::Serialize)]
struct ResponseJson {
    pub json_TRS: Vec<Rule>,
    pub json_interpret: functions,
}

#[derive(serde::Serialize)]
struct functions {
    pub functions: ParsedDataInterpret
}

pub fn handle_request(request: &rouille::Request) -> rouille::Response {
    let json: InputJson = try_or_400!(rouille::input::json_input(request));
    let mut err = ErrorJson { error_trs: Vec::new(), error_interpretation: Vec::new() };
    let mut res = ResponseJson { json_TRS: Vec::new(), json_interpret: functions{ functions: vec![] } };

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
        Err(e) => err.error_trs.push(e),
    };

    let model = Model {
        variables: parser_trs.variables,
        constants: parser_trs.constants,
        functions: parser_trs.functions,
    };
    if  err.error_trs.len() > 0{
        return rouille::Response::json(&err).with_status_code(400);
    }

    let mut parser_interpret = ParserInterpret::new(&json.Interpretation[..], model);
    match parser_interpret.parse() {
        Ok(result) => {
            println!("Парсинг Interpet: {:?}", result);
            res.json_interpret = match result {
                Interpret(interpret) => functions{functions: interpret},
                _ => functions{functions: ParsedDataInterpret::default()}
            };
        }
        Err(e) => err.error_interpretation.push(e),
    };
    if err.error_trs.len() > 0 || err.error_interpretation.len() > 0 {
        return rouille::Response::json(&err).with_status_code(400);
    }
    let resp = rouille::Response::json(&res);
    resp
    //rouille::Response::text(format!("field1's value is {}", json.Interpretation))
}