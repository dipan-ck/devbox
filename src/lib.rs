use std::{io, net::TcpListener};

use actix_web::{App, HttpResponse, HttpServer, Responder, dev::Server, web};
pub mod configuration;
pub mod routes;
pub mod startup;
pub fn run(listener: TcpListener) -> Result<Server, io::Error> {
    let server = HttpServer::new(|| App::new().route("/health_check", web::get().to(health_check)))
        .listen(listener)?
        .run();

    Ok(server)
}
