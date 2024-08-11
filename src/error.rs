use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};
use serde::Serialize;

pub type Resultc<T> = core::result::Result<T, Error>;

#[derive(thiserror::Error, Clone, Debug, Serialize)]
#[serde(tag = "type", content = "data")]
pub enum Error {
    #[error("Database error")]
    LoginFail,

    // -- Auth errors.
    #[error("Database error")]
    AuthFailNoAuthTokenCookie,
    #[error("Database error")]
    AuthFailTokenWrongFormat,
    #[error("Database error")]
    AuthFailCtxNotInRequestExt,

    // -- Model errors.
    #[error("Database error")]
    TicketDeleteFailIdNotFound { id: u64 },
}
impl IntoResponse for Error {
    fn into_response(self) -> Response {
        (StatusCode::INTERNAL_SERVER_ERROR, "").into_response()
    }
}

impl Error {
    pub fn client_status_and_error(&self) -> (StatusCode, ClientError) {
        #[allow(unreachable_patterns)]
        match self {
            Self::LoginFail => (StatusCode::FORBIDDEN, ClientError::LOGIN_FAIL),

            // -- Auth.
            Self::AuthFailNoAuthTokenCookie
            | Self::AuthFailTokenWrongFormat
            | Self::AuthFailCtxNotInRequestExt => (StatusCode::FORBIDDEN, ClientError::NO_AUTH),

            // -- Model.
            Self::TicketDeleteFailIdNotFound { .. } => {
                (StatusCode::BAD_REQUEST, ClientError::INVALID_PARAMS)
            }

            // -- Fallback.
            _ => (
                StatusCode::INTERNAL_SERVER_ERROR,
                ClientError::SERVICE_ERROR,
            ),
        }
    }
}
#[derive(Debug)]
#[allow(non_camel_case_types)]
pub enum ClientError {
    LOGIN_FAIL,
    NO_AUTH,
    INVALID_PARAMS,
    SERVICE_ERROR,
}
