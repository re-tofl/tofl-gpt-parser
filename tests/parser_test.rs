#[cfg(test)]
mod tests {
    use tofl_gpt_parser::models;
    use tofl_gpt_parser::parsers;
    use tofl_gpt_parser::parsers::Parse;

    #[test]
    fn test_trs1() {
        let input = "variables = x,y\nf(x,h(y))=h(f(x,y))\ng = f";
        let mut parser_trs = parsers::ParserTRS::new(input);

        assert_eq!(format!("{:?}", parser_trs.parse().unwrap()), string)
    }
}