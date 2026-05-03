use actix_web::dev::ServiceRequest;
use actix_web::{web, Error, HttpMessage};
use actix_web_httpauth::extractors::bearer::BearerAuth;
use actix_web_httpauth::extractors::AuthenticationError;
use actix_web_httpauth::headers::www_authenticate::bearer::Bearer;

use crate::auth::validate_token;
use crate::state::AppState;

pub async fn bearer_validator(
    req: ServiceRequest,
    creds: BearerAuth,
) -> Result<ServiceRequest, (Error, ServiceRequest)> {
    let secret = req
        .app_data::<web::Data<AppState>>()
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
