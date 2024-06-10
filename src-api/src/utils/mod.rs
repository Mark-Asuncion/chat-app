use actix_web::{web::Bytes, cookie::{SameSite, time::{Duration, OffsetDateTime}, Cookie}};
use base64::{engine::general_purpose, Engine as _};
use pbkdf2::Params;
use uuid::Uuid;
use std::{io, env, str::from_utf8};

pub mod password;

pub static SESSION_NAME: &str = "cha-session-id";

pub fn gen_uuid() -> String {
    // len 36 (including dashes)
    Uuid::new_v4().to_string()
}

// pub fn json_from(bytes: Bytes) -> io::Result<serde_json::Value> {
//     let body = std::str::from_utf8(&bytes.to_vec())
//         .unwrap_or_default()
//         .to_string();
//     let json: serde_json::Value = serde_json::from_str(body.as_str())?;
//     Ok(json)
// }

pub fn get_session_expire() -> Duration {
    let debug = env::var("CHA_DEBUG").unwrap_or("false".into());
    if debug == "true" {
        return Duration::days(30);
    }
    Duration::minutes(10)
}

// pub fn set_cookie(id: String) -> Cookie<'static> {
//     let debug = env::var("CHA_DEBUG").unwrap_or("false".into());
//     Cookie::build(SESSION_NAME, id)
//         .path("/")
//         .same_site(SameSite::Lax)
//         .domain("localhost")
//         .expires(OffsetDateTime::now_utc() + get_session_expire())
//         .http_only(true)
//         .secure(debug == "false")
//         .finish()
// }

pub fn base64_encode(v: &str) -> String {
    let mut buf = Vec::new();
    buf.resize(v.len() * 4 / 3 + 4, 0);
    let bytes_written = general_purpose::STANDARD.encode_slice(
        v.as_bytes(),
        &mut buf
    ).unwrap();
    buf.truncate(bytes_written);

    let encoded = from_utf8(&buf).unwrap_or_default();
    encoded.to_string()
}

pub fn base64_decode(v: &str) -> String {
    let buf = general_purpose::STANDARD
        .decode(v.as_bytes()).unwrap_or_default();

    let decoded = from_utf8(&buf).unwrap_or_default();
    decoded.to_string()
}

pub fn get_pbkdf2_params() -> Params {
    let mut params = Params::default();
    params.rounds = 1_000;
    params
}
