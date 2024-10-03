use crate::ctx::Ctx;
use crate::error::Resultc;
use crate::model::ModelController;
use crate::model::Ticket;
use crate::web;
use crate::Error;
use axum::extract::State;
use axum::http::HeaderMap;
use axum::http::StatusCode;
use axum::routing::get;
use axum::routing::post;
use axum::Json;
use axum::Router;
use db_service::User;
use jsonwebtoken::decode;
use jsonwebtoken::encode;
use jsonwebtoken::DecodingKey;
use jsonwebtoken::EncodingKey;
use jsonwebtoken::Header;
use jsonwebtoken::Validation;
use serde::Deserialize;
use serde::Serialize;
use serde_json::{json, Value};
use tower_cookies::{Cookie, Cookies};

#[derive(Serialize)]
pub struct LoginResponse {
    pub token: String,
}

#[derive(Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,
    pub exp: usize,
}

#[derive(Debug, Deserialize)]
struct LoginPayload {
    username: String,
    mail: String,
    pwd: String,
}

#[derive(Debug, Deserialize)]
struct LoginInfo {
    username: String,
    password: String,
}

pub fn routes(mc: ModelController) -> Router {
    Router::new()
        .route("/api/login", post(api_login))
        .route("/api/logsin", post(login_handler))
        .route("/api/info", get(get_info_handler))
        .route("/api/signup", post(user_signup))
        .with_state(mc)
}

async fn api_login(
    State(mc): State<ModelController>,
    ctx: Ctx,
    cookies: Cookies,
    Json(payload): Json<User>,
) -> Resultc<Json<Value>> {
    // TODO: Implement real db/auth logic.

    let jwt = mc.user_signin(ctx, payload).await?;
    /* if payload.mail != "admin@test.de" || payload.pw != "welcome" {
        return Err(Error::LoginFail);
    } */

    // FIXME: Implement real auth-token generation/signature.
    // GET IT FROM SRLDB

    let mut cookie = Cookie::new(web::AUTH_TOKEN, "user-1.exp.sign");
    cookie.set_http_only(true);
    cookie.set_path("/");
    cookies.add(cookie);

    // Create the success body.
    let body = Json(json!({
        "result": {
            "success": true
        }
    }));

    Ok(body)
}
async fn user_signup(
    State(mc): State<ModelController>,
    ctx: Ctx,
    Json(user): Json<User>,
) -> Resultc<Json<String>> {
    let ticket = mc.user_signup(ctx, user).await?;

    Ok(Json(ticket))
}

async fn login_handler(
    Json(login_info): Json<LoginInfo>,
) -> Result<Json<LoginResponse>, StatusCode> {
    let username = &login_info.username;
    let password = &login_info.password;
    let is_valid = is_valid_user(username, password);
    if is_valid {
        let claims = Claims {
            sub: username.to_string(),
            exp: (chrono::Utc::now() + chrono::Duration::hours(1)).timestamp() as usize,
        };

        let token = match encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret("secret".as_ref()),
        ) {
            Ok(tok) => tok,
            Err(e) => {
                eprint!("Error Generating Token {}", e);
                return Err(StatusCode::INTERNAL_SERVER_ERROR);
            }
        };
        Ok(Json(LoginResponse { token }))
    } else {
        return Err(StatusCode::UNAUTHORIZED);
    }
}
async fn get_info_handler(header_map: HeaderMap) -> Result<Json<String>, StatusCode> {
    if let Some(auth_header) = header_map.get("Authorization") {
        if let Ok(auth_header_str) = auth_header.to_str() {
            if auth_header_str.trim().starts_with("Bearer ") {
                let token = auth_header_str.trim_start_matches("Bearer ");

                match decode::<Claims>(
                    token,
                    &DecodingKey::from_secret("secret".as_ref()),
                    &Validation::default(),
                ) {
                    Ok(_) => {
                        return Ok(Json("You are authorized".to_string()));
                    }
                    Err(_) => {
                        return Err(StatusCode::UNAUTHORIZED);
                    }
                }
            }
        }
    }

    Err(StatusCode::UNAUTHORIZED)
}

pub fn is_valid_user(username: &str, password: &str) -> bool {
    //TODO check in DB
    username != "" && password != "";
    true
}
