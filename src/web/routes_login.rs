use axum::{routing::{post, Route}, Json, Router};
use serde::Deserialize;
use serde_json::{json, value, Value};
use tower_cookies::{Cookie, Cookies};

use crate::{web, Error, Result};

pub fn routes() -> Router {
    Router::new().route("/api/login", post(api_login))
}

async fn api_login(cookies: Cookies, payload: Json<LoginPayLoad>) -> Result<Json<Value>> {
    println!("->> {:<12} - api_login ", "HANDLER");

    //TODO: Implement real db/auth logic
    if payload.username != "demo1" || payload.pwd != "welcome" {
        return Err(Error::LoginFail);
    }

    cookies.add(Cookie::new(web::AUTH_TOKEN, "DDDDuser-1.exp.sign"));


    // Crate the success body
    let body = Json(json!({
        "result": {
            "success": true
        }
    }));


    Ok(body)
}

#[derive(Debug, Deserialize)]
struct LoginPayLoad {
    username: String,
    pwd: String,
}