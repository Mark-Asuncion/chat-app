// TODO return 500 when failed to acquire database instance
// TODO salting and hashing password
use actix_session::Session;
use actix_web::{HttpResponse, Responder, guard, http::StatusCode};
use actix_web::web;
use serde_json::json;
use sqlx::Row;
use crate::database::query::QueryValue;
use crate::{session::MSession, AppState, database::{query, DatabaseUtils}, error};
use crate::database::schema::account::{Account, LoginRegisterInfo, ValidateForms};

async fn _login_handler(user: Option<web::Json<LoginRegisterInfo>>, state: web::Data<AppState>, session: Session) -> impl Responder {
    dbg!(&session.entries());
    let mut session_st = MSession::from(&session);
    if session_st.authorized {
        session.renew();
        return HttpResponse::Ok()
            .body("")
    }

    if let None = user {
        return HttpResponse::build(StatusCode::from_u16(401).unwrap())
            .body(error::Error::missing_credentials().to_string());
    }

    let acc = user.unwrap().to_account();

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
        return HttpResponse::build(StatusCode::from_u16(401).unwrap())
            .body(error::Error::bad_credentials().to_string());
    }
    let res = res.unwrap();

    // if res.is_empty() {
    //     return HttpResponse::build(StatusCode::from_u16(401).unwrap())
    //         .body(error::Error::bad_credentials().to_string());
    // }

    let id: String =        res.get_unchecked(0);
    let email: String =     res.get_unchecked(1);
    let username: String =  res.get_unchecked(2);
    let password: String =  res.get_unchecked(3);
    println!("len({})::{}, {}, {}, {}", res.len(), id, email, username, password);

    session_st.user_id = id;
    session_st.authorized = true;
    if let Err(e) = session_st.insert(&session) {
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
    let db = &(*state.database_instance.lock().expect(&error::Error::acquire_instance().to_string()));
    {
        let mut username_resp = "";
        let mut email_resp = "";
        let mut qb = query::QueryBuilder::new();
        if !acc.username.is_empty() {
            qb.select(Account::table(), Some(Account::as_columns()))
                .filter(query::Filter::If("username".into(), "=".into(), QueryValue::Varchar(acc.username.clone())));
            let rs = db.fetch_one(qb).await;
            if let Ok(_) = rs {
                username_resp = "exists";
            }
        }
        qb = query::QueryBuilder::new();
        if !acc.email.is_empty() {
            qb.select(Account::table(), Some(Account::as_columns()))
                .filter(query::Filter::If("email".into(), "=".into(), QueryValue::Varchar(acc.email.clone())));
            let rs = db.fetch_one(qb).await;
            if let Ok(_) = rs {
                email_resp = "exists";
            }
        }

        if !email_resp.is_empty() || !username_resp.is_empty() {
            let mut remail = Some(email_resp);
            if email_resp.is_empty() {
                remail = None;
            }
            let mut rusername = Some(username_resp);
            if username_resp.is_empty() {
                rusername = None;
            }
            return HttpResponse::BadRequest()
                .json(json!({
                    "email": remail,
                    "username": rusername
                }));
        }
    }

    acc.gen_uuid();
    let mut qb = query::QueryBuilder::new();
    qb.insert(Account::table(), Account::as_columns())
        .value(acc.as_insert_value());

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

async fn _validate_handler(body: web::Json<ValidateForms>, state: web::Data<AppState>) -> impl Responder {
    let db = &(*state.database_instance.lock().expect(&error::Error::acquire_instance().to_string()));
    let forms = body.into_inner();
    let mut email_resp = "";
    let mut username_resp = "";
    if let Some(email) = forms.email {
        let mut qb = query::QueryBuilder::new();
        qb.select(Account::table(), None)
            .filter(query::Filter::If( "email".into(), "=".into(), QueryValue::Varchar(email) ));
        let rs = db.fetch_one(qb).await;
        if let Ok(_) = rs {
            email_resp = "exists";
        }
    }

    if let Some(username) = forms.username {
        let mut qb = query::QueryBuilder::new();
        qb.select(Account::table(), None)
            .filter(query::Filter::If( "username".into(), "=".into(), QueryValue::Varchar(username) ));
        let rs = db.fetch_one(qb).await;
        if let Ok(_) = rs {
            username_resp = "exists";
        }
    }

    let mut remail = Some(email_resp);
    if email_resp.is_empty() {
        remail = None;
    }
    let mut rusername = Some(username_resp);
    if username_resp.is_empty() {
        rusername = None;
    }
    HttpResponse::Ok()
        .json(json!({
            "email": remail,
            "username": rusername
        }))
}

pub fn validate(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("/validate")
            .guard(guard::Header("Content-Type", "application/json"))
            .post(_validate_handler)
            .head(HttpResponse::MethodNotAllowed)
    );
}
