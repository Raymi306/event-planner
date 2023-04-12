use actix_web::{get, http::header::ContentType, post, put, web, HttpResponse, Responder};
use jsonwebtoken::TokenData;
use sqlx::{Pool, Postgres};

use crate::models::{Organization, OrganizationCategory, Claims};

#[get("/")]
pub async fn get_organizations(db_pool: web::Data<Pool<Postgres>>) -> impl Responder {
    let organizations_query = "SELECT * FROM organization;";
    let results = sqlx::query_as::<_, Organization>(organizations_query)
        .fetch_all(db_pool.get_ref())
        .await;
    if let Ok(organizations) = results {
        HttpResponse::Ok().json(organizations)
    } else {
        HttpResponse::Ok()
            .content_type(ContentType::json())
            .body("[]")
    }
}

#[post("/")]
pub async fn create_organization(
    // TODO
    // missing categories
    user: web::Data<TokenData<Claims>>,
    organization_full: web::Json<OrganizationFull>,
    db_pool: web::Data<Pool<Postgres>>,
) -> impl Responder {
    if user.claims.role != PersonRole::Admin {
        return HttpResponse::Forbidden().finish();
    }
    let organization_query = 
        "INSERT INTO organization
            (name, description, website, phone_number_prefix, phone_number, phone_number_extension) 
            VALUES ($1, $2, $3, $4, $5, $6, $7) RETURNING organization.id;";
    let organization = organization_full.organization;
    let organization_id_maybe = sqlx::query(organization_query)
        .bind(&organization.name)
        .bind(&organization.description)
        .bind(&organization.website)
        .bind(&organization.phone_number_prefix)
        .bind(&organization.phone_number)
        .bind(&organization.phone_number_extension)
        .fetch_one(db_pool.get_ref())
        .await;
    if organization_id_maybe.is_err() {
        HttpResponse::InternalServerError().finish()
    }
    let organization_id = organization_id_maybe.unwrap();
    let categories_query =
        "INSERT INTO organization_categories
            (organization_id, category)
            SELECT * FROM UNNEST(
                array_fill($1, ARRAY[$2]),
                $3::organization_category[]);";
    let result = sqlx::query(categories_query)
        .bind(&organization_id)
        .bind(&organization_full.categories.len())
        .bind(&organization_full.categories);
    if let Ok(_) = result {
        HttpResponse::Ok().finish()
    } else {
        HttpResponse::InternalServerError().finish()
    }
}

#[put("/")]
pub async fn update_organization(
    // TODO
    // missing categories
    user: web::Data<TokenData<Claims>>,
    organization_info: web::Json<Organization>,
    db_pool: web::Data<Pool<Postgres>>,
) -> impl Responder {
    if user.claims.role != PersonRole::Admin {
        return HttpResponse::Forbidden().finish();
    }
    let select_organization_query = "SELECT * FROM organization WHERE id = $1";
    let organization = sqlx::query_as::<_, Organization>(select_organization_query)
        .bind(&organization_info.id)
        .fetch_one(db_pool.get_ref())
        .await;
    if organization.is_err() {
        return HttpResponse::NotFound().finish();
    }
    let update_organization_query = "UPDATE organization SET (name, description, website, phone_number_prefix, phone_number, phone_number_extension) VALUES ($1, $2, $3, $4, $5, $6, $7);";
    let result = sqlx::query(update_organization_query)
        .bind(&organization_info.name)
        .bind(&organization_info.description)
        .bind(&organization_info.website)
        .bind(&organization_info.phone_number_prefix)
        .bind(&organization_info.phone_number)
        .bind(&organization_info.phone_number_extension)
        .execute(db_pool.get_ref())
        .await;
    if let Ok(_) = result {
        HttpResponse::Ok().finish()
    } else {
        HttpResponse::InternalServerError().finish()
    }
}

#[post("/{organization_id}/person/{person_id}")]
pub async fn add_person(
    path: web::Path<(u32, u32)>,
    user: web::Data<TokenData<Claims>>,
    db_pool: web::Data<Pool<Postgres>>,
) -> impl Responder {
    let organization_id = path.0;
    let person_id = path.1;
    todo!();
}
