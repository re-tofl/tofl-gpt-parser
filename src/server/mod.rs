pub mod handlers;

use handlers::handle_request;

pub fn start_server() {
    println!("Сервер запущен...");

    handle_request();
}