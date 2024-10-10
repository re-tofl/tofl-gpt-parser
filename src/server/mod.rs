pub mod handlers;

use std::io;
use rouille::{router};
use handlers::handle_request;

pub fn start_server() {
    let addr = "0.0.0.0:8090";
    println!("Now listening on {addr}");

    rouille::start_server(addr, move |request| {
        rouille::log(request, io::stdout(), || {
            router!(request,
                (POST) (/parse) => {
                    handle_request(request)
                },
                _ => rouille::Response::empty_404()
            )
        })
    });
}