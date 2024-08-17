//use axum::http::{request, Response, StatusCode};

//login
/* pub async fn middleware_header<T>(
    request: axum::http::Request<T>,
    next: axum::middleware::Next<T>,
) -> Result<Response, StatusCode> {
    let headers = request.headers();
    let message = headers.get("message").ok_or_else( || StatusCode::BAD_REQUEST)?;
    Ok(next.run(req: request).await)
} */
