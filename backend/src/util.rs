use actix_web::{dev::ServiceRequest, web::Data, Error, HttpMessage};
use actix_web_httpauth::extractors::bearer::BearerAuth;
use actix_web_httpauth::extractors::AuthenticationError;
use actix_web_httpauth::headers::www_authenticate::bearer::{Bearer, Error as TokenError};

use jsonwebtoken::{Algorithm::HS512, decode, DecodingKey, Validation};

use crate::config::AppConfig;
use crate::models::Claims;

pub async fn auth_validator(
    req: ServiceRequest,
    credentials: BearerAuth,
) -> Result<ServiceRequest, (Error, ServiceRequest)> {
    let config = req.app_data::<Data<AppConfig>>().unwrap().clone();
    let mut token_validation = Validation::new(HS512);
    token_validation.set_required_spec_claims(&["sub"]);
    token_validation.validate_exp = false;
    if let Ok(token) = decode::<Claims>(
        credentials.token(),
        &DecodingKey::from_secret(config.secret.as_ref()),
        &token_validation,
    ) {
        req.extensions_mut().insert(Data::new(token));
        Ok(req)
    } else {
        let challenge = Bearer::build().error(TokenError::InvalidToken).finish();
        Err((AuthenticationError::new(challenge).into(), req))
    }
}
