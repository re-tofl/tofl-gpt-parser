use tofl_gpt_parser;

fn main() {
    println!("Запуск сервера парсера!");
    tofl_gpt_parser::server::start_server()
}