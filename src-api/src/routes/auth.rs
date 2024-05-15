use std::str::from_utf8;

use actix_web::{web::{self, Bytes}, HttpResponse, Responder, guard, http::StatusCode};
use crate::AppState;
use crate::database::schema::Account;

async fn _login_handler(bytes: Bytes, state: web::Data<AppState>) -> impl Responder {
    let body = from_utf8(&bytes.to_vec())
        .unwrap_or_default()
        .to_string();

    let acc: Account;
    match serde_json::from_str(body.as_str()) {
        Ok(v) => acc = v,
        Err(e) => {
            dbg!(e);
            return HttpResponse::BadRequest()
                .body("");
        }
    }
    let db = &(*state.database_instance.lock().expect(""));
    dbg!(&acc);

    Account::find(db, acc.clone());

    HttpResponse::Ok()
        .body(body)
}

pub fn login(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("/login")
            .guard(guard::Header("Content-Type", "application/json"))
            .post(_login_handler)
            .head(HttpResponse::MethodNotAllowed)
    );
}
