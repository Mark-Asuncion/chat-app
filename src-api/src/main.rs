use std::fs::File;
use std::io::Read;
use std::{env, process::exit}; use actix_session::config::{BrowserSession, CookieContentSecurity, PersistentSession};
use actix_web::cookie::{Key, SameSite};
use actix_web::middleware::Logger;
use actix_web::{HttpServer, App, web};
use database::{ DatabaseInstance, init };
use utils::get_session_expire;
use std::sync::Mutex;
use std::path::PathBuf;
use actix_session::{SessionMiddleware, storage::CookieSessionStore};
use actix_cors::Cors;

mod routes;
mod database;
mod utils;
mod error;
mod session;
mod tests;

#[derive(Debug)]
struct AppState {
    database_instance: Mutex<DatabaseInstance>
}

fn cors() -> Cors {
    Cors::default()
        .allowed_origin_fn(|origin, _| {
            origin.as_bytes().starts_with(b"http://localhost")
        })
        .allowed_methods(vec!["GET", "POST"])
        .allowed_headers(vec![
            actix_web::http::header::AUTHORIZATION,
            actix_web::http::header::ACCEPT,
            actix_web::http::header::CONTENT_TYPE
        ])
        .supports_credentials()
        .max_age(3600)
}

fn middleware_session() -> SessionMiddleware<CookieSessionStore> {
    let debug = env::var("CHA_DEBUG").unwrap_or("false".into());
    let key = env::var("CHA_COOKIE_SESSION_KEY").expect("COOKIE_SESSION_KEY env is not set");
    let key = Key::from(key.as_bytes());
    let session_lifecycle = PersistentSession::default().session_ttl(get_session_expire());

    SessionMiddleware::builder(
        CookieSessionStore::default(), key
    )
    .cookie_name(String::from(utils::SESSION_NAME))
    .cookie_secure(debug == "true")
    .session_lifecycle(session_lifecycle)
    .cookie_same_site(SameSite::Lax)
    .cookie_content_security(CookieContentSecurity::Private)
    .cookie_http_only(true)
    .build()
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

    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    HttpServer::new(move || {
        App::new()
            .app_data(state.clone())
            // .wrap(Logger::default())
            .wrap(Logger::new(r#""%r" %s %bb %T"#))
            .wrap(middleware_session())
            .wrap(cors())
            .service(
                web::scope("/auth")
                    .configure(routes::auth::login)
                    .configure(routes::auth::register)
                    .configure(routes::auth::validate)
            )
            .service(
                web::scope("/user")
                    .configure(routes::user::info)
            )
    })
        .bind(server_domain)?
        .run()
    .await
}
