// TODO return 500 when failed to acquire database instance
use actix_session::Session;
use actix_web::{HttpResponse, Responder, guard, http::StatusCode};
use actix_web::web;
use serde_json::json;
use crate::database::query::{QueryValue, builder, filter, join};
use crate::database::schema::salt::Salt;
use crate::database::schema::{salt, ToQueryBuilder};
use crate::utils::password::Password;
use crate::{session::MSession, AppState, database::DatabaseUtils, error};
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
    let pass_to_hash = acc.password.clone();

    if acc.password.is_empty() {
        return HttpResponse::build(StatusCode::from_u16(401).unwrap())
            .body(error::Error::missing_credentials().to_string());
    }

    let db = &mut (*state.database_instance.lock().expect(&error::Error::acquire_instance().to_string()));

    let filter: Option<filter::Filter> = {
        if !acc.email.is_empty() {
            Some(filter::Filter::if_from(
                "email",
                "=",
                QueryValue::Varchar(acc.email.clone())
            ))
        }
        else if !acc.username.is_empty() {
            Some(filter::Filter::if_from(
                "username",
                "=",
                QueryValue::Varchar(acc.username.clone())
            ))
        }
        else {
            None
        }
    };
    if let None = filter {
        return HttpResponse::build(StatusCode::from_u16(401).unwrap())
            .body(error::Error::missing_credentials().to_string());
    }
    let filter = filter.unwrap();
    let mut qb = builder::QueryBuilder::new();
    let mut cols = Account::as_columns_alias();
    let mut salt_cols = Salt::as_columns();
    let salt_user_id = *salt_cols.get(1).unwrap();

    cols.append(&mut salt_cols);
    qb.select(Account::table(), Some(cols))
        .filter(filter)
        .join( join::Join::inner(Salt::table(), (Account::pkey(), salt_user_id)) );

    let row = db.fetch_one(qb).await;
    if let Err(e) = row {
        dbg!(e);
        return HttpResponse::build(StatusCode::from_u16(401).unwrap())
            .body(error::Error::bad_credentials().to_string());
    }
    let row = row.unwrap();
    let acc = Account::from_row_alias(&row);
    let salt = Salt::from_row(&row);
    if salt.is_empty() {
        return HttpResponse::InternalServerError()
            .body(error::Error::internal_server("Cannot find account information").to_string());
    }

    let hashed = Password::hash_from(&salt.id, &pass_to_hash);
    if !hashed.verify(&acc.password) {
        return HttpResponse::build(StatusCode::from_u16(401).unwrap())
            .body(error::Error::bad_credentials().to_string());
    }

    session_st.user_id = acc.id;
    session_st.authorized = true;
    if let Err(e) = session_st.insert(&session) {
        dbg!(e);
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

async fn _register_handler(user: web::Json<LoginRegisterInfo>, state: web::Data<AppState>) -> impl Responder {
    let acc = user.to_account();
    dbg!(&acc);
    if acc.email.is_empty() || acc.username.is_empty() || acc.password.is_empty() {
        return HttpResponse::build(StatusCode::from_u16(401).unwrap())
            .body(error::Error::missing_credentials().to_string());
    }
    let db = &(*state.database_instance.lock().expect(&error::Error::acquire_instance().to_string()));

    let hashed = Password::hash(&acc.password);
    if hashed.is_empty() {
        return HttpResponse::InternalServerError()
            .body(
                error::Error::internal_server("Error occured processing password")
                .to_string()
            );
    }
    let acc = Account::new(&acc.email,&acc.username, &hashed.hash);
    let salt = salt::Salt::new(&hashed.salt, &acc.id);

    let vqb = vec![ acc.insert_query(), salt.insert_query() ];

    let res = db.execute_insert_mult(vqb).await;
    if let Err(e) = res {
        dbg!(e);
        return HttpResponse::build(StatusCode::from_u16(401).unwrap())
            .body(error::Error::credentials_exists() .to_string());
    }
    println!("_register_handler::affected rows {:?}", res.unwrap().rows_affected());

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
        let mut qb = builder::QueryBuilder::new();
        qb.select(Account::table(), None)
            .filter(filter::Filter::if_from( "email", "=", QueryValue::Varchar(email) ));
        let rs = db.fetch_one(qb).await;
        if let Ok(_) = rs {
            email_resp = "exists";
        }
    }

    if let Some(username) = forms.username {
        let mut qb = builder::QueryBuilder::new();
        qb.select(Account::table(), None)
            .filter(filter::Filter::if_from( "username", "=", QueryValue::Varchar(username) ));
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
