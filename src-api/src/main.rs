use std::{env, process::exit};
use actix_web::{HttpServer, App};

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
                        println!("Error Occured {:?}", e);
                        exit(1);
                    }

                }
            }
        }
    }

    if port == 0 {
        println!("Port not specified.\n Specify with [--port|-p <PORT>] option");
        exit(1);
    }

    let server_host = ("127.0.0.1", port);
    println!("Server listening on http://{}:{}", server_host.0, server_host.1);
    HttpServer::new(|| {
        App::new()
    })
        .bind(server_host)?
        .run()
    .await
}
