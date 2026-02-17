use std::{io, net::TcpListener};

use devbox::run;

#[tokio::main]
async fn main() -> Result<(), io::Error> {
    let listener = TcpListener::bind("127.0.0.1:0").expect("failed to create TCP LIstener");
    run(listener)?.await
}
