use actix_session::Session;
use actix_web::{web, HttpResponse, Responder, http::StatusCode};

use crate::{error, session::MSession, database::{query::{self, Filter, QueryValue}, schema::account::Account, DatabaseUtils}, AppState};


async fn _info_handler(state: web::Data<AppState>, session: Session) -> impl Responder {
    let session_st = MSession::from(&session);
    if !session_st.authorized {
        return HttpResponse::build(StatusCode::from_u16(401).unwrap())
            .body(error::Error::not_authenticated().to_string());
    }

    dbg!(&session_st);

    let uid = session_st.user_id.clone();
    let mut qb = query::QueryBuilder::new();
    qb.select(Account::table(), None)
        .filter(Filter::If("id".into(), "=".into(), QueryValue::Varchar(uid)));

    let db = &mut (*state.database_instance.lock().expect(&error::Error::acquire_instance().to_string()));

    let res = db.fetch_one(qb).await;
    if let Err(e) = res {
        dbg!(e);
        return HttpResponse::build(StatusCode::from_u16(401).unwrap())
            .body(error::Error::not_authenticated().to_string());
    }

    let row = res.unwrap();
    let acc = Account::from_row(&row);
    let ro_acc = acc.to_read_only();

    HttpResponse::Ok()
        .json(ro_acc)
}

pub fn info(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("/info")
            .get(_info_handler)
            .head(HttpResponse::MethodNotAllowed)
    );
}
