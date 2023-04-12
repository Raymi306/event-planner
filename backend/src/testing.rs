use actix_web::{get, web, HttpResponse, Responder};
use sqlx::{Pool, Postgres};

use crate::models::Person;

#[get("/test")]
pub async fn test(db_pool: web::Data<Pool<Postgres>>) -> impl Responder {
    println!("BEGIN TEST");
    let persons_query = "SELECT * FROM person;";
    let results = sqlx::query_as::<_, Person>(persons_query)
        .fetch_all(db_pool.get_ref())
        .await;
    println!("{:?}", results);
    if let Ok(persons) = results {
        HttpResponse::Ok().json(persons)
    } else {
        HttpResponse::NotFound().finish()
    }
}
