use std::{env, process::exit};
use actix_web::{HttpServer, App, web};
use database::{ DatabaseInstance, init };
use std::sync::Mutex;

mod routes;
mod database;
mod utils;

#[derive(Debug)]
struct AppState {
    database_instance: Mutex<DatabaseInstance>
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let mut args = env::args();
    let mut port: u16 = 0;
    while let Some(arg) = args.next() {
        if arg == "-p" || arg == "--port" {
            if let Some(v) = args.next() {
                match v.parse() {
                    Ok(arg_port) => port = arg_port,
                    Err(e) => {
                        println!("Error Occured {:?}", e); exit(1);
                    }

                }
            }
        } }

    if port == 0 {
        println!("Port not specified.\n Specify with [--port|-p <PORT>] option");
        exit(1);
    }

    let db_instance = init().await
        .expect("Error creating DatabaseInstance");

    let server_domain = ("127.0.0.1", port);
    println!("Server listening on http://{}:{}", server_domain.0, server_domain.1);
    let state = web::Data::new( AppState {
        database_instance: Mutex::new(db_instance)
    });

    HttpServer::new(move || {
        App::new()
            .app_data(state.clone())
            .service(
                web::scope("/auth").configure(routes::auth::login)
            )
    })
        .bind(server_domain)?
        .run()
    .await
}
