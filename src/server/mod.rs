pub mod handlers;
mod page;
use std::io;
use rouille::{router};
use handlers::handle_request;
use page::handle_page;

pub fn start_server() {
    let addr = "0.0.0.0:8090";
    println!("Now listening on {addr}");

    rouille::start_server(addr, move |request| {
        rouille::log(request, io::stdout(), || {
            router!(request,
                (POST) (/parse) => {
                    handle_request(request)
                },
                (GET) (/) => {
                    handle_page(request)
                },
                _ => rouille::Response::empty_404()
            )
        })
    });
}