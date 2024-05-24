use std::fs::File;
use std::io::Read;
use std::{env, process::exit};
use actix_web::{HttpServer, App, web};
use database::{ DatabaseInstance, init };
use std::sync::Mutex;
use std::path::PathBuf;

mod routes;
mod database;
mod utils;

#[derive(Debug)]
struct AppState {
    database_instance: Mutex<DatabaseInstance>
}

fn load_env(path: String) {
    let mut f = File::options()
            .read(true)
            .open(PathBuf::from(&path))
            .expect(&format!("Error Opening {}", &path));
    let mut buf = String::new();
    let res = f.read_to_string(&mut buf);
    if let Err(e) = res {
        print!("Error reading {}\nerr: {}", &path, e);
    }
    let envs: Vec<&str> = buf.split('\n').collect();
    let err: &str = "Env Wrong format";
    for line in envs.iter() {
        if line.is_empty() {
            continue;
        }
        let kv: Vec<&str> = line.split('=').collect();
        env::set_var(kv.get(0).expect(err), kv.get(1).expect(err));
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let mut args = env::args();
    let mut port: u16 = 0;
    let mut p_env_file = String::new();
    args.next();
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
        else {
            p_env_file = arg;
        }
    }

    if p_env_file.is_empty() {
        println!("Env file not specified. Specify with ./server <file_name>");
        exit(1);
    }
    load_env(p_env_file);

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
                web::scope("/auth")
                    .configure(routes::auth::login)
                    .configure(routes::auth::register)
            )
    })
        .bind(server_domain)?
        .run()
    .await
}
