use std::net::TcpListener;

use devbox::run;

pub fn spawn_app() -> String {
    let listener = TcpListener::bind("127.0.0.1:0").expect("'falied to create TCP Listener");

    let port = listener.local_addr().unwrap().port();

    let server = run(listener).expect("something went wrong starting the server");
    tokio::spawn(server);

    format!("http://127.0.0.1:{}", port)
}
