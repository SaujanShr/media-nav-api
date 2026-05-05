use actix_web::{
    Error, HttpMessage,
    web::Data, HttpRequest, dev::ServiceRequest
};
use actix_web::error::ErrorUnauthorized;
use actix_web_httpauth::extractors::{AuthenticationError, bearer::BearerAuth};
use actix_web_httpauth::headers::www_authenticate::bearer::Bearer;

use crate::auth::{validate_token, Claims};
use crate::state::AppState;

pub fn claims(req: &HttpRequest) -> Result<Claims, Error> {
    req.extensions()
        .get::<Claims>()
        .cloned()
        .ok_or_else(|| ErrorUnauthorized("unauthorized"))
}

pub fn user_id(req: &HttpRequest) -> Result<String, Error> {
    claims(req).map(|c| c.sub)
}

pub async fn bearer_validator(
    req: ServiceRequest,
    creds: BearerAuth,
) -> Result<ServiceRequest, (Error, ServiceRequest)> {
    let secret = req
        .app_data::<Data<AppState>>()
        .map(|s| s.jwt_secret.clone())
        .unwrap_or_default();

    match validate_token(creds.token(), &secret) {
        Ok(claims) => {
            req.extensions_mut().insert(claims);
            Ok(req)
        }
        Err(_) => {
            let error = AuthenticationError::new(Bearer::default());
            Err((error.into(), req))
        }
    }
}
