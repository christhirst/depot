use crate::ctx::Ctx;
use crate::error::Resultc;
use crate::model::ModelController;
use crate::model::Ticket;
use crate::web;
use crate::Error;
use axum::extract::State;
use axum::routing::post;
use axum::Json;
use axum::Router;
use db_service::User;
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

pub fn routes(mc: ModelController) -> Router {
    Router::new()
        .route("/api/login", post(api_login))
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
) -> Resultc<Json<Value>> {
    let ticket = mc.user_signup(ctx, user).await?;

    //Ok(Json(ticket))
    todo!()
}

#[derive(Debug, Deserialize)]
struct LoginPayload {
    username: String,
    mail: String,
    pwd: String,
}
