use crate::error::ApiError;
use actix_web::http::StatusCode;
use actix_web::{web, HttpRequest, HttpResponse, Scope};
use http_auth_basic::Credentials;
use serde_json::json;

async fn login(req: HttpRequest) -> Result<HttpResponse, ApiError> {
    if let Some(authorization) = get_authorization(&req) {
        match Credentials::from_header(authorization.to_string()) {
            Ok(creds) => {
                let body = json!({
                    "username": creds.user_id,
                    "password": creds.password,
                });

                let res = HttpResponse::build(StatusCode::OK).json(body);

                Ok(res)
            }
            Err(err) => Err(ApiError::from(err)),
        }
    } else {
        Err(ApiError::new(
            400,
            "Missing Authorization Header".to_string(),
        ))
    }
}

fn get_authorization<'a>(req: &'a HttpRequest) -> Option<&'a str> {
    req.headers().get("authorization")?.to_str().ok()
}

/// "auth/" routes
pub fn auth() -> Scope {
    let scope = Scope::new("/auth");

    scope.route("/login", web::get().to(login))
}
