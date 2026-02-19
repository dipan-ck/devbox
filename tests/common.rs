use std::net::TcpListener;

use devbox::{
    configuration::{self, Settings},
    startup::run,
};
use sqlx::{Connection, PgConnection, PgPool};
use uuid::Uuid;

pub async fn spawn_app() -> (String, PgPool) {
    let mut configuration = configuration::get_config().expect("error getting configurations");
    //we will let the TcpListener pick random ports for us so that we dont hit port conflict
    let listener = TcpListener::bind("127.0.0.1:0").expect("'falied to create TCP Listener");
    let address = format!("http://127.0.0.1:{}", listener.local_addr().unwrap().port());
    let connection_pool = configure_db(&mut configuration).await;

    let server =
        run(listener, connection_pool.clone()).expect("something went wrong starting the server");
    tokio::spawn(server);

    (address, connection_pool)
}

pub async fn configure_db(configs: &mut Settings) -> PgPool {
    configs.database.database_name = Uuid::new_v4().to_string();
    // Create database
    let mut connection = PgConnection::connect(&configs.database.connection_string_without_db())
        .await
        .expect("Failed to connect to Postgres");

    //Create database
    sqlx::query(&format!(
        r#"CREATE DATABASE "{}";"#,
        configs.database.database_name
    ))
    .execute(&mut connection)
    .await
    .expect("Failed to create database");

    let connection = PgPool::connect(&configs.database.connection_string())
        .await
        .expect("failed to connect to Database");
    sqlx::migrate!("./migrations")
        .run(&connection)
        .await
        .expect("Something went wrong in migrations");

    connection
}
