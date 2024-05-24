use std::str::from_utf8;

use actix_web::{web::{self, Bytes}, HttpResponse, Responder, guard, http::StatusCode};
use sqlx::{Row, QueryBuilder};
use crate::{AppState, database::{query, DatabaseUtils, error}};
use crate::database::schema::account::Account;

async fn _login_handler(bytes: Bytes, state: web::Data<AppState>) -> impl Responder {
    let body = from_utf8(&bytes.to_vec())
        .unwrap_or_default()
        .to_string();

    let acc: serde_json::Value;
    match serde_json::from_str(body.as_str()) {
        Ok(v) => acc = v,
        Err(e) => {
            dbg!(e);
            return HttpResponse::BadRequest()
                .body("");
        }
    }
    let acc = Account::from(acc);
    if let Err(e) = &acc {
            return HttpResponse::BadRequest()
                .body(e.to_string());
    }
    let acc = acc.unwrap();

    let db = &mut (*state.database_instance.lock().expect(&error::Error::acquire_instance().to_string()));

    let mut query = query::QueryBuilder::new();
    let filter1 = query::Filter::If("email".into(), "=".into(), query::QueryValue::Varchar(acc.email.clone()));
    let filter2 = query::Filter::If("username".into(), "=".into(), query::QueryValue::Varchar(acc.username.clone()));
    let filter3 = query::Filter::If("password".into(), "=".into(), query::QueryValue::Varchar(acc.password.clone()));
    query.select(Account::table(), Some(Account::as_columns()))
            .filter(filter1)
            .or()
            .filter(filter2)
            .and()
            .filter(filter3);

    let res = db.fetch_one(query).await;
    if let Err(e) = res {
        dbg!(e);
        return HttpResponse::build(StatusCode::from_u16(400).unwrap())
            .body(error::Error::not_found().to_string());
    }

    let res = res.unwrap();
    if !res.is_empty() {
        let id: String = res.get_unchecked(0);
        let email: String = res.get_unchecked(1);
        let username: String = res.get_unchecked(2);
        let password: String = res.get_unchecked(3);
        println!("len({})::{}, {}, {}, {}", res.len(), id, email, username, password);
    }

    HttpResponse::Ok()
        .body("")
}

pub fn login(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("/login")
            .guard(guard::Header("Content-Type", "application/json"))
            .post(_login_handler)
            .head(HttpResponse::MethodNotAllowed)
    );
}

async fn _register_handler(bytes: Bytes, state: web::Data<AppState>) -> impl Responder {
    let body = from_utf8(&bytes.to_vec())
        .unwrap_or_default()
        .to_string();

    let acc: serde_json::Value;
    match serde_json::from_str(body.as_str()) {
        Ok(v) => acc = v,
        Err(e) => {
            dbg!(e);
            return HttpResponse::BadRequest()
                .body("");
        }
    }
    let acc = Account::from(acc);
    if let Err(e) = &acc {
        return HttpResponse::BadRequest()
            .body(e.to_string());
    }
    let mut acc = acc.unwrap();
    acc.get_uuid();

    let mut qb = query::QueryBuilder::new();
    qb.insert(Account::table(), Account::as_columns())
        .value(acc.as_insert_value());
    let db = &(*state.database_instance.lock().expect(&error::Error::acquire_instance().to_string()));

    let res = db.execute_insert(qb).await;
    if let Err(e) = res {
        return HttpResponse::BadRequest()
            .body(e.to_string());
    }

    let res = res.unwrap();
    println!("_register_handler::rows inserted {}", res.rows_affected());

    HttpResponse::Ok()
        .body("")
}

pub fn register(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("/register")
            .guard(guard::Header("Content-Type", "application/json"))
            .post(_register_handler)
            .head(HttpResponse::MethodNotAllowed)
    );
}
