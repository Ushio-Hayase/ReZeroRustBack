use std::net::TcpListener;

use re_zero_rust_back::run;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let lst = TcpListener::bind("127.0.0.1:0").unwrap();
    run(lst)?.await
}
