mod server;
mod parsers;
mod models;

use server::start_server;
fn main() {
    println!("Запуск сервера парсера!");
    start_server();
}