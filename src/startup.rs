use std::{io, net::TcpListener};

use actix_web::{App, HttpServer, dev::Server, web};

use crate::routes;

pub fn run(listener: TcpListener) -> Result<Server, io::Error> {
    let server =
        HttpServer::new(|| App::new().route("/health_check", web::get().to(routes::health_check)))
            .listen(listener)?
            .run();

    Ok(server)
}
