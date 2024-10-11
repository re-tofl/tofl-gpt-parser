use std::collections::{HashMap, HashSet};

#[cfg(test)]
mod tests {
    use std::collections::{HashMap, HashSet};
    use std::fmt::Debug;
    use tofl_gpt_parser::{models, server};
    use tofl_gpt_parser::models::data_structures::Model;
    use tofl_gpt_parser::models::ParsedData;
    use tofl_gpt_parser::parsers;
    use tofl_gpt_parser::parsers::{Parse, ParserInterpret};

    #[test]
    fn test_trs0() {
        let input = "variables = x,y\nf(x,h(y))=h(f(x,"; //EOF error
        let mut parser_trs = parsers::ParserTRS::new(input);
        match parser_trs.parse() {
            Ok(res) => {panic!("должна быть eof ошибка")}
            Err(e) => {}
        }
    }

    #[test]
    fn test_trs1() {
        let input = "variables = x,y\nf(x,h(y))=h(f(x,y))\ng = f";
        //let input = "variables = x\ng(x) = f(f)";
        let mut parser_trs = parsers::ParserTRS::new(input);
        match parser_trs.parse() {
            Ok(res) => {}
            Err(e) => { panic!("{:?}", e) }
        }
    }

    #[test]
    fn test_trs2() {
        let input = "variables = x\ng(x) = f(f)";
        let mut parser_trs = parsers::ParserTRS::new(input);
        match parser_trs.parse() {
            Ok(res) => { panic!("должна вернуться ошибка") }
            Err(e) => { assert_eq!(e, "Ошибка в строке 2, на позиции 9, ожидалась константа или переменная, считана функция") }
        }
    }

    #[test]
    fn test_trs3() {
        let input = "variables = x\nf(x) = g\nf(x,y) = k(x)";
        let mut parser_trs = parsers::ParserTRS::new(input);
        match parser_trs.parse() {
            Ok(res) => { panic!("должна вернуться ошибка") }
            Err(e) => { assert_eq!(e, "Не совпадает арность функции f, ожидаемое количество аргументов: 1 , получили: 2") }
        }
    }

    #[test]
    fn test_trs4() {
        let input = "variables = x,y,x\nf(x) = g\nf(x,y) = k(x)";
        let mut parser_trs = parsers::ParserTRS::new(input);
        match parser_trs.parse() {
            Ok(res) => { panic!("должна вернуться ошибка") }
            Err(e) => { assert_eq!(e, "Переменная x объявлена несколько раз") }
        }
    }

    #[test]
    fn test_interpret1() {
        let input = "variables = m,n,o\nF(m,n) = G(o)";
        let input1 = "F(m,n) = m+n\nG(o) = 2*o\n";
        let mut parser_trs = parsers::ParserTRS::new(input);
        parser_trs.parse().unwrap();
        let model = Model {
            variables: parser_trs.variables,
            constants: parser_trs.constants,
            functions: parser_trs.functions,
        };
        let mut parser_interpret = ParserInterpret::new(input1, model);
        let res = parser_interpret.parse();
        match res {
            Ok(res) => {}
            Err(e) => { panic!("{:?}", e) }
        }
    }

    #[test]
    fn test_interpret2() {
        let input = "f(x)=x+1";
        let mut variables = HashSet::new();
        variables.insert('x');
        let mut functions = HashMap::new();
        functions.insert('f', 1);

        let mut parser = parsers::ParserInterpret::new(input, Model{
            variables,
            constants: Default::default(),
            functions,
        });

        match parser.parse() {
            Ok(res) => {}
            Err(e) => { panic!("{:?}", e) }
        }
    }

    #[test]
    fn test_interpret3() {
        let input = "f(x)=x+1\n";
        let mut variables = HashSet::new();
        variables.insert('x');
        let mut functions = HashMap::new();
        functions.insert('f', 1);

        let mut parser = parsers::ParserInterpret::new(input, Model{
            variables,
            constants: Default::default(),
            functions,
        });

        match parser.parse() {
            Ok(res) => {}
            Err(e) => { panic!("{:?}", e) }
        }
    }

    #[test]
    fn test_interpret4() {
        let input = "f(x)=x+1\r\n c=4\rg(x)=x+4+2*xx{3}+1";
        let mut variables = HashSet::new();
        variables.insert('x');
        let mut functions = HashMap::new();
        functions.insert('f', 1);
        functions.insert('g', 1);
        let mut constants = HashSet::new();
        constants.insert('c');


        let mut parser = parsers::ParserInterpret::new(input, Model{
            variables,
            constants,
            functions,
        });

        match parser.parse() {
            Ok(res) => {}
            Err(e) => { panic!("{:?}", e) }
        }
    }

    #[test]
    fn test_interpret5() {
        let input = "f(x,y)=x";
        let mut variables = HashSet::new();
        variables.insert('x');
        variables.insert('y');
        let mut functions = HashMap::new();
        functions.insert('f', 1);


        let mut parser = parsers::ParserInterpret::new(input, Model{
            variables,
            constants: Default::default(),
            functions,
        });

        match parser.parse() {
            Ok(res) => { panic!("должна вернуться ошибка") }
            Err(e) => {}
        }
    }

    #[test]
    fn test_interpret6() {
        let input = "f(x)=z";
        let mut variables = HashSet::new();
        variables.insert('x');
        let mut functions = HashMap::new();
        functions.insert('f', 1);


        let mut parser = parsers::ParserInterpret::new(input, Model{
            variables,
            constants: Default::default(),
            functions,
        });

        match parser.parse() {
            Ok(res) => { panic!("должна вернуться ошибка") }
            Err(e) => {}
        }
    }

    #[test]
    fn test_interpret7() {
        let input = "f(z)=z";
        let mut variables = HashSet::new();
        variables.insert('x');
        let mut functions = HashMap::new();
        functions.insert('f', 1);

        let mut parser = parsers::ParserInterpret::new(input, Model{
            variables,
            constants: Default::default(),
            functions,
        });

        match parser.parse() {
            Ok(res) => {}
            Err(e) => { panic!("{:?}", e) }
        }
    }

    #[test]
    fn test_parse_eol() {
        let input = "variables = x,y\nf(x,h(y))=h(f(x,y))\n\ng = f";
        let mut parser_trs = parsers::ParserTRS::new(input);
        match parser_trs.parse() {
            Ok(res) => { panic!("должна вернуться ошибка") }
            Err(e) => {}
        }
    }

    #[test]
    fn test_complete() {
        let headers: Vec<(String, String)> = vec![("Content-Type".to_string(), "application/json".to_string())];
        let string = "{\r\n   \"TRS\":\"variables = m,n,o\\nF(m,n) = G(o)\",\r\n   \"Interpretation\":\"F(m,n) = m+n\\nG(o) = 2*o\"\r\n}";

        let req = rouille::Request::fake_http("GET", "", headers, Vec::from(string));
        let resp = server::handlers::handle_request(&req);
        assert_eq!(resp.status_code, 200);
    }
}