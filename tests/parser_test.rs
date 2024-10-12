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
        let input = "variables = x,y\nf(x,h(y))=h(f(x,y))\ng = f(x,y)";
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
            Err(e) => { assert_eq!(e[0], "Ошибка в строке 2, на позиции 9, ожидалась константа или переменная, считана функция") }
        }
    }

    #[test]
    fn test_trs3() {
        let input = "variables = x\nf(x) = g\nf(x,y) = k(x)";
        let mut parser_trs = parsers::ParserTRS::new(input);
        match parser_trs.parse() {
            Ok(res) => { panic!("должна вернуться ошибка") }
            Err(e) => { assert_eq!(e[0], "Не совпадает арность функции f, ожидаемое количество аргументов: 1 , считано: 2") }
        }
    }

    #[test]
    fn test_trs4() {
        let input = "variables = x,y,x\nf(x) = g\nf(x,y) = k(x)";
        let mut parser_trs = parsers::ParserTRS::new(input);
        match parser_trs.parse() {
            Ok(res) => { panic!("должна вернуться ошибка") }
            Err(e) => { assert_eq!(e[0], "Переменная x объявлена несколько раз") }
        }
    }

    #[test]
    fn test_trs_no_equals_sign() {
        let input = "variables  x,y\nf(x) = g\nf(x,y) = k(x)";
        let mut parser_trs = parsers::ParserTRS::new(input);
        match parser_trs.parse() {
            Ok(res) => { panic!("должна вернуться ошибка") }
            Err(e) => {
                println!("{:?}", e)
            }
        }
    }

    #[test]
    fn test_trs_deep_nesting_correct_1() {
        let input = "variables = x\na(b(c(d(e(f(x)))))) = g(h(i(j(k(l(x))))))\n";
        let mut parser_trs = parsers::ParserTRS::new(input);
        match parser_trs.parse() {
            Ok(res) => {
            }
            Err(e) => {
                panic!("{:?}", e)
            }
        }
    }

    #[test]
    fn test_trs_deep_nesting_correct_2() {
        let input = "variables = x\na(b(a(b(a(b(a(b(x)))))))) = a(x)\n";
        let mut parser_trs = parsers::ParserTRS::new(input);
        match parser_trs.parse() {
            Ok(res) => {
            }
            Err(e) => {
                panic!("{:?}", e)
            }
        }
    }

    #[test]
    fn test_trs_deep_nesting_correct_3() {
        let input = "variables = x,y,z\nf(g(h(i(j(x)))),k(l(m(n(y)))),o(p(q(r(s(z)))))) = t(u(v(w(x))))\n";
        let mut parser_trs = parsers::ParserTRS::new(input);
        match parser_trs.parse() {
            Ok(res) => { }
            Err(e) => {
                panic!("{:?}", e)
            }
        }
    }

    #[test]
    fn test_trs_deep_nesting_error_1() {
        // Несоответствует арность
        let input = "variables = x\nf(g(h(i(j(k(l(m(n(o(p(x))))))))))) = q(x)\nf(a,b) = c\n";
        let mut parser_trs = parsers::ParserTRS::new(input);
        match parser_trs.parse() {
            Ok(res) => {panic!("должна быть ошибка")}
            Err(e) => { println!("{:?}", e) }
        }
    }

    #[test]
    fn test_trs_deep_nesting_error_2() {
        let input = "variables = x\na(b(c(d(e(f(g(h(i(j(k(x)))))))))) = x\n";
        // Здесь отсутствует закрывающая скобка для функции 'a'
        let mut parser_trs = parsers::ParserTRS::new(input);
        match parser_trs.parse() {
            Ok(res) => {panic!("должна быть ошибка")}
            Err(e) => { println!("{:?}", e) }
        }
    }

    #[test]
    fn test_interpret_function_constant_not_declared() { //Функция была объявлена в TRS, но её нет в интерпретации
        let input1 = "F(m,n) = m+n\n";
        let mut functions = HashMap::new();
        functions.insert('F', 2);
        functions.insert('A', 1);
        let mut variables = HashSet::new();
        variables.insert('m');
        variables.insert('n');
        let mut constants = HashSet::new();
        constants.insert('p');
        let mut parser_interpret = ParserInterpret::new(input1, Model{
            variables,
            constants,
            functions,
        });

        let res = parser_interpret.parse();
        match res {
            Ok(res) => {panic!("должна быть ошибка")}
            Err(e) => { println!("{:?}", e) }
        }
    }

    #[test]
    fn test_interpret_function_not_declared() { //Функция была объявлена в TRS, но её нет в интерпретации
        let input1 = "F(m,n) = m+n\n";
        let mut functions = HashMap::new();
        functions.insert('F', 2);
        functions.insert('A', 1);
        let mut variables = HashSet::new();
        variables.insert('m');
        variables.insert('n');
        let mut parser_interpret = ParserInterpret::new(input1, Model{
            variables,
            constants: HashSet::new(),
            functions,
        });

        let res = parser_interpret.parse();
        match res {
            Ok(res) => {panic!("должна быть ошибка")}
            Err(e) => {  }
        }
    }

    #[test]
    fn test_interpret_constant_not_declared() { //Константа была объявлена в TRS, но её нет в интерпретации
        let input1 = "F(m,n) = m+n\n";
        let mut functions = HashMap::new();
        functions.insert('F', 2);
        let mut variables = HashSet::new();
        variables.insert('m');
        variables.insert('n');
        let mut constants = HashSet::new();
        constants.insert('p');
        let mut parser_interpret = ParserInterpret::new(input1, Model{ variables, constants, functions, });

        let res = parser_interpret.parse();
        match res {
            Ok(res) => {panic!("должна быть ошибка")}
            Err(e) => { println!("{:?}", e) }
        }
    }

    #[test]
    fn test_interpret_eof_f_const() { //ожидалось: функция или константа, считано EOF
        let input1 = "";
        let mut parser_interpret = ParserInterpret::new(input1, Model{
            variables: HashSet::new(), constants: HashSet::new(), functions:HashMap::new(),
        });
        let res = parser_interpret.parse();
        match res {
            Ok(res) => {panic!("должна быть ошибка")}
            Err(e) => { println!("{:?}", e) }
        }
    }

    #[test]
    fn test_interpret_expected_f_const() { //ожидалось: функция или константа, считано что-то
        let input1 = "😎";
        let mut parser_interpret = ParserInterpret::new(input1, Model{
            variables: HashSet::new(), constants: HashSet::new(), functions:HashMap::new(),
        });
        let res = parser_interpret.parse();
        match res {
            Ok(res) => {panic!("должна быть ошибка")}
            Err(e) => { println!("{:?}", e) }
        }
    }

    #[test]
    fn test_interpret_f_not_declared_in_trs() { //Константа была объявлена в TRS, но её нет в интерпретации
        let input1 = "F(m,n) = 2m+n\n";
        let mut functions = HashMap::new();
        let mut variables = HashSet::new();
        let mut constants = HashSet::new();
        let mut parser_interpret = ParserInterpret::new(input1, Model{ variables, constants, functions, });

        let res = parser_interpret.parse();
        match res {
            Ok(res) => {panic!("должна быть ошибка")}
            Err(e) => { println!("{:?}", e) }
        }
    }

    #[test]
    fn test_interpret_0_coef() { //Коэффициент не может быть равен 0
        let input1 = "F(m) = 0*m\n";
        let mut functions = HashMap::new();
        functions.insert('F', 1);
        let mut variables = HashSet::new();
        variables.insert('m');
        let mut constants = HashSet::new();
        let mut parser_interpret = ParserInterpret::new(input1, Model{ variables, constants, functions, });
        let res = parser_interpret.parse();
        match res {
            Ok(res) => {panic!("должна быть ошибка")}
            Err(e) => { println!("{:?}", e) }
        }
    }

    #[test]
    fn test_interpret_expected_alphabetic_var() { //Ожидалась буква (в названии переменной)
        let input1 = "F(m,n) = m+🔥";
        let mut functions = HashMap::new();
        functions.insert('F', 2);
        let mut variables = HashSet::new();
        variables.insert('m');
        variables.insert('n');
        let mut constants = HashSet::new();
        let mut parser_interpret = ParserInterpret::new(input1, Model{ variables, constants, functions, });
        let res = parser_interpret.parse();
        match res {
            Ok(res) => {panic!("должна быть ошибка")}
            Err(e) => { println!("{:?}", e) }
        }
    }

    #[test]
    fn test_interpret_expected_bracket() { // Ожидалось ',' или ')', считано что-то
        let input1 = "F(m,n| = m+n";
        let mut functions = HashMap::new();
        functions.insert('F', 2);
        let mut variables = HashSet::new();
        variables.insert('m');
        variables.insert('n');
        let mut constants = HashSet::new();
        let mut parser_interpret = ParserInterpret::new(input1, Model{ variables, constants, functions, });
        let res = parser_interpret.parse();
        match res {
            Ok(res) => {panic!("должна быть ошибка")}
            Err(e) => { println!("{:?}", e) }
        }
    }

    #[test]
    fn test_interpret_expected_bracket_eof() { //ожидалось: ')' или ',', считано EOF
        let input1 = "F(m,n";
        let mut functions = HashMap::new();
        functions.insert('F', 2);
        let mut variables = HashSet::new();
        variables.insert('m');
        variables.insert('n');
        let mut constants = HashSet::new();
        let mut parser_interpret = ParserInterpret::new(input1, Model{ variables, constants, functions, });
        let res = parser_interpret.parse();
        match res {
            Ok(res) => {panic!("должна быть ошибка")}
            Err(e) => { println!("{:?}", e) }
        }
    }

    #[test]
    fn test_interpret_expected_plus() { //ожидалось: ')' или ',', считано EOF
        let input1 = "F(m) = m,";
        let mut functions = HashMap::new();
        functions.insert('F', 1);
        let mut variables = HashSet::new();
        variables.insert('m');
        let mut constants = HashSet::new();
        let mut parser_interpret = ParserInterpret::new(input1, Model{ variables, constants, functions, });
        let res = parser_interpret.parse();
        match res {
            Ok(res) => {panic!("должна быть ошибка")}
            Err(e) => { println!("{:?}", e) }
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
    fn test_function_arity_mismatch() { //Количество переменных в интерпретации функции f не совпадает с количеством переменных в TRS
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
            Err(e) => {println!("{:?}", e)}
        }
    }

    #[test]
    fn test_argument_repeat_interpretation() {
        let input = "f(x,x)=x";
        let mut variables = HashSet::new();
        variables.insert('x');
        let mut functions = HashMap::new();
        functions.insert('f', 2);
        let mut parser = parsers::ParserInterpret::new(input, Model{
            variables,
            constants: Default::default(),
            functions,
        });

        match parser.parse() {
            Ok(res) => { panic!("должна вернуться ошибка") }
            Err(e) => {println!("{:?}", e)}
        }
    }

    #[test]
    fn test_interpret6() { //Переменная z не указана в качестве аргумента функции
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
            Err(e) => {println!("{:?}", e)}
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
    fn test_interpret8() {
        let input = "f(x, y)=2*xyx";
        let mut variables = HashSet::new();
        variables.insert('x');
        let mut functions = HashMap::new();
        functions.insert('f', 2);

        let mut parser = parsers::ParserInterpret::new(input, Model{
            variables,
            constants: Default::default(),
            functions,
        });

        match parser.parse() {
            Ok(res) => {println!("{:?}", res)}
            Err(e) => { panic!("{:?}", e) }
        }
    }

    #[test]
    fn test_interpret9() {
        let input = "f(x, y)=2*xyx+4+2+x{3}xy4*xy3*x";
        let mut variables = HashSet::new();
        variables.insert('x');
        let mut functions = HashMap::new();
        functions.insert('f', 2);

        let mut parser = parsers::ParserInterpret::new(input, Model{
            variables,
            constants: Default::default(),
            functions,
        });

        match parser.parse() {
            Ok(res) => {println!("{:?}", res)}
            Err(e) => { panic!("{:?}", e) }
        }
    }

    #[test]
    fn test_interpret10() {
        let input = "f(x, y)=xy\nk=5";
        let mut variables = HashSet::new();
        variables.insert('x');
        let mut functions = HashMap::new();
        functions.insert('f', 2);
        let mut constants = HashSet::new();
        constants.insert('k');

        let mut parser = parsers::ParserInterpret::new(input, Model{
            variables,
            constants,
            functions,
        });

        match parser.parse() {
            Ok(res) => {println!("{:?}", res)}
            Err(e) => { panic!("{:?}", e) }
        }
    }

    #[test]
    fn test_interpret11() {
        let input = "f(x, y)=xy\n f(x,y)=4\nk=1\nk=2";
        let mut variables = HashSet::new();
        variables.insert('x');
        let mut functions = HashMap::new();
        functions.insert('f', 2);
        let mut constants = HashSet::new();
        constants.insert('k');

        let mut parser = parsers::ParserInterpret::new(input, Model{
            variables,
            constants,
            functions,
        });

        match parser.parse() {
            Ok(res) => {panic!("должна быть ошибка")}
            Err(e) => {  println!("{:?}", e)}
        }
    }

    #[test]
    fn test_parsers_interaction() {
        let input = "variables = x,y\nf(x,y) = k";
        let input1 = "f(x, y)=xy\nk=5";
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