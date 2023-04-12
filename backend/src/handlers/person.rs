use actix_web::{get, http::header::ContentType, post, put, web, HttpResponse, Responder};
use jsonwebtoken::TokenData;
use sqlx::{Pool, Postgres};

use crate::models::{Person, PersonRole, Claims};

#[get("/")]
pub async fn get_persons(db_pool: web::Data<Pool<Postgres>>) -> impl Responder {
    let persons_query = "SELECT * FROM person;";
    let results = sqlx::query_as::<_, Person>(persons_query)
        .fetch_all(db_pool.get_ref())
        .await;
    if let Ok(persons) = results {
        HttpResponse::Ok().json(persons)
    } else {
        HttpResponse::Ok()
            .content_type(ContentType::json())
            .body("[]")
    }
}

#[post("/")]
pub async fn create_person(
    user: web::Data<TokenData<Claims>>,
    person: web::Json<Person>,
    db_pool: web::Data<Pool<Postgres>>,
) -> impl Responder {
    if user.claims.role != PersonRole::Admin {
        return HttpResponse::Forbidden().finish();
    }
    let person_query = "INSERT INTO person (full_name, email, phone_number_prefix, phone_number, phone_number_extension) VALUES ($1, $2, $3, $4, $5, $6);";
    let result = sqlx::query(person_query)
        .bind(&person.full_name)
        .bind(&person.email)
        .bind(&person.phone_number_prefix)
        .bind(&person.phone_number)
        .bind(&person.phone_number_extension)
        .execute(db_pool.get_ref())
        .await;
    if let Ok(_) = result {
        HttpResponse::Ok().finish()
    } else {
        HttpResponse::InternalServerError().finish()
    }
}

#[put("/")]
pub async fn update_person(
    user: web::Data<TokenData<Claims>>,
    person_info: web::Json<Person>,
    db_pool: web::Data<Pool<Postgres>>,
) -> impl Responder {
    if user.claims.role != PersonRole::Admin {
        return HttpResponse::Forbidden().finish();
    }
    let select_person_query = "SELECT * FROM person WHERE id = $1";
    let person = sqlx::query_as::<_, Person>(select_person_query)
        .bind(&person_info.id)
        .fetch_one(db_pool.get_ref())
        .await;
    if person.is_err() {
        return HttpResponse::NotFound().finish();
    }
    // some persons are accounts too
    if person.unwrap().password_digest.is_some() {
        return HttpResponse::Forbidden().finish();
    }
    let update_person_query = "UPDATE person SET (full_name, email, phone_number_prefix, phone_number, phone_number_extension) VALUES ($1, $2, $3, $4, $5, $6);";
    let result = sqlx::query(update_person_query)
        .bind(&person_info.full_name)
        .bind(&person_info.email)
        .bind(&person_info.phone_number_prefix)
        .bind(&person_info.phone_number)
        .bind(&person_info.phone_number_extension)
        .execute(db_pool.get_ref())
        .await;
    if let Ok(_) = result {
        HttpResponse::Ok().finish()
    } else {
        HttpResponse::InternalServerError().finish()
    }
}
