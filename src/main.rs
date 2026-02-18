use std::{io, net::TcpListener};

use devbox::{configuration, startup::run};

#[tokio::main]
async fn main() -> Result<(), io::Error> {
    let configuration = configuration::get_config().expect("Error while generating config");
    let address = format!("127.0.0.1:{}", configuration.application_port);
    let listener = TcpListener::bind(address).expect("failed to create TCP LIstener");

    run(listener)?.await
}
