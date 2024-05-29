use actix_session::Session;
use actix_web::{HttpResponse, Responder, guard, http::StatusCode};
use actix_web::web;
use sqlx::Row;
use crate::{AppState, database::{query, DatabaseUtils}, error};
use crate::database::schema::account::{Account, LoginRegisterInfo};

async fn _login_handler(user: web::Json<LoginRegisterInfo>, state: web::Data<AppState>, session: Session) -> impl Responder {
    let is_authorized = session.get::<bool>("authorized").unwrap_or_default().unwrap_or_default();
    if is_authorized {
        session.renew();
        return HttpResponse::Ok()
            .body("")
    }

    let acc = user.to_account();

    if (acc.email.is_empty() || acc.username.is_empty()) && acc.password.is_empty() {
        return HttpResponse::build(StatusCode::from_u16(401).unwrap())
            .body(error::Error::missing_credentials().to_string());
    }

    let db = &mut (*state.database_instance.lock().expect(&error::Error::acquire_instance().to_string()));

    let mut query = query::QueryBuilder::new();
    let filter1: query::Filter;

    if !acc.email.is_empty() {
        filter1 = query::Filter::If("email".into(), "=".into(), query::QueryValue::Varchar(acc.email.clone()));
    }
    else {
        filter1 = query::Filter::If("username".into(), "=".into(), query::QueryValue::Varchar(acc.username.clone()));
    }
    let filter2 = query::Filter::If("password".into(), "=".into(), query::QueryValue::Varchar(acc.password.clone()));
    query.select(Account::table(), Some(Account::as_columns()))
            .filter(filter1)
            .and()
            .filter(filter2);

    let res = db.fetch_one(query).await;
    if let Err(e) = res {
        dbg!(e);
        return HttpResponse::build(StatusCode::from_u16(400).unwrap())
            .body(error::Error::not_found().to_string());
    }
    let res = res.unwrap();
    if res.is_empty() {
        return HttpResponse::build(StatusCode::from_u16(401).unwrap())
            .body(error::Error::bad_credentials().to_string());
    }

    let id: String =        res.get_unchecked(0);
    let email: String =     res.get_unchecked(1);
    let username: String =  res.get_unchecked(2);
    let password: String =  res.get_unchecked(3);
    println!("len({})::{}, {}, {}, {}", res.len(), id, email, username, password);

    if let Err(e) = session.insert("user_id", id) {
        dbg!(e);
    }
    if let Err(e) = session.insert("authorized", true) {
        dbg!(e);
    }

    dbg!(&session.entries());

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

async fn _register_handler(user: web::Json<LoginRegisterInfo>, state: web::Data<AppState>) -> impl Responder {
    let mut acc = user.to_account();

    if (acc.email.is_empty() || acc.username.is_empty()) && acc.password.is_empty() {
        return HttpResponse::build(StatusCode::from_u16(401).unwrap())
            .body(error::Error::missing_credentials().to_string());
    }
    acc.gen_uuid();

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
