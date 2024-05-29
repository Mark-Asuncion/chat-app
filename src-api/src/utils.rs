use actix_web::{web::Bytes, cookie::{SameSite, time::{Duration, OffsetDateTime}, Cookie}};
use uuid::Uuid;
use std::{io, env};

pub static SESSION_NAME: &str =     "cha-session-id";

pub fn gen_uuid() -> String {
    // len 36 (including dashes)
    Uuid::new_v4().to_string()
}

pub fn json_from(bytes: Bytes) -> io::Result<serde_json::Value> {
    let body = std::str::from_utf8(&bytes.to_vec())
        .unwrap_or_default()
        .to_string();
    let json: serde_json::Value = serde_json::from_str(body.as_str())?;
    Ok(json)
}

pub fn get_session_expire() -> Duration {
    let debug = env::var("CHA_DEBUG").unwrap_or("false".into());
    if debug == "true" {
        return Duration::days(30);
    }
    Duration::minutes(10)
}

pub fn set_cookie(id: String) -> Cookie<'static> {
    let debug = env::var("CHA_DEBUG").unwrap_or("false".into());
    Cookie::build(SESSION_NAME, id)
        .path("/")
        .same_site(SameSite::Lax)
        .domain("localhost")
        .expires(OffsetDateTime::now_utc() + get_session_expire())
        .http_only(true)
        .secure(debug == "false")
        .finish()
}
