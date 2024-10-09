#[cfg(test)]
mod tests {
    use std::fmt::Debug;
    use tofl_gpt_parser::{models, server};
    use tofl_gpt_parser::models::data_structures::Model;
    use tofl_gpt_parser::models::ParsedData;
    use tofl_gpt_parser::parsers;
    use tofl_gpt_parser::parsers::{Parse, ParserInterpret};

    #[test]
    fn test_trs1() {
        let input = "variables = x,y\nf(x,h(y))=h(f(x,y))\ng = f";
        let mut parser_trs = parsers::ParserTRS::new(input);
        assert!(parser_trs.parse().is_ok());
    }

    #[test]
    fn test_interpret1(){
        let input = "variables = m,n,o\nF(m,n) = G(o)";
        let input1 = "F(m,n) = m+n\nG(o) = 2*o\n";
        let mut parser_trs = parsers::ParserTRS::new(input);
        parser_trs.parse().unwrap();
        let model = Model{variables: parser_trs.variables,
            constants: parser_trs.constants,
            functions: parser_trs.functions};
        let mut parser_interpret = ParserInterpret::new(input1, model);
        let res = parser_interpret.parse();
        match res {
            Ok(res) => {}
            Err(e) => {panic!("{:?}", e)}
        }
    }

    #[test]
    fn test_complete(){
        let headers : Vec<(String, String)> = vec![("Content-Type".to_string(), "application/json".to_string())];
        let string = "{\r\n   \"TRS\":\"variables = m,n,o\\nF(m,n) = G(o)\",\r\n   \"Interpretation\":\"F(m,n) = m+n\\nG(o) = 2*o\"\r\n}";

        let req = rouille::Request::fake_http("GET", "", headers, Vec::from(string));
        let resp = server::handlers::handle_request(&req);
        assert_eq!(resp.status_code, 200);
    }
}