use actix_web::{post, web, HttpResponse, Responder};
use argon2::{
    password_hash::{PasswordHash, PasswordVerifier},
    Argon2,
};
use jsonwebtoken::{encode, Algorithm, EncodingKey, Header};
use sqlx::{Pool, Postgres};
use totp_rs::{Rfc6238, TOTP};

use crate::config::AppConfig;
use crate::models::{Claims, LoginInfo, LoginResponse, Person, PersonRole};

fn password_is_valid(password_raw: &str, password_digest_maybe: &Option<Vec<u8>>) -> bool {
    if let Some(password_digest) = password_digest_maybe {
        let user_password = std::str::from_utf8(password_digest).unwrap();
        let password_hash = PasswordHash::new(user_password).unwrap();
        Argon2::default()
            .verify_password(password_raw.as_bytes(), &password_hash)
            .is_ok()
    } else {
        false
    }
}

fn totp_is_valid(user_code: &str, totp_secret_maybe: Option<Vec<u8>>) -> bool {
    if let Some(totp_secret) = totp_secret_maybe {
        let rfc = Rfc6238::with_defaults(totp_secret).unwrap();
        let totp = TOTP::from_rfc6238(rfc).unwrap();
        let code = totp.generate_current().unwrap();
        user_code == code
    } else {
        false
    }
}

#[post("/login")]
pub async fn login(
    login_info: web::Json<LoginInfo>,
    config: web::Data<AppConfig>,
    db_pool: web::Data<Pool<Postgres>>,
) -> impl Responder {
    let user_query = "SELECT * FROM person WHERE email = $1;";
    let user_maybe = sqlx::query_as::<_, Person>(user_query)
        .bind(&login_info.email)
        .fetch_one(db_pool.get_ref())
        .await;
    if let Ok(user) = user_maybe {
        if user.role != PersonRole::Disabled
            && password_is_valid(&login_info.password_raw, &user.password_digest)
            && totp_is_valid(&login_info.totp, user.totp_secret)
        {
            let claims = Claims {
                sub: login_info.email.clone(),
                id: user.id,
                role: user.role,
            };
            let header = Header::new(Algorithm::HS512);
            let token = encode(
                &header,
                &claims,
                &EncodingKey::from_secret(config.secret.as_bytes()),
            )
            .unwrap();
            return HttpResponse::Ok().json(LoginResponse { token });
        }
    }
    HttpResponse::Unauthorized().finish()
}
