use std::net::TcpListener;

use devbox::{configuration, startup::run};

pub fn spawn_app() -> String {
    let configuration = configuration::get_config().expect("error getting configurations");
    let address = format!("127.0.0.1:{}", configuration.application_port);
    let listener = TcpListener::bind(&address).expect("'falied to create TCP Listener");

    let server = run(listener).expect("something went wrong starting the server");
    tokio::spawn(server);

    format!("http://{}", address)
}
