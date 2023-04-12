use serde::{Deserialize, Serialize};
use sqlx::{FromRow, Type};

#[derive(Debug, Deserialize)]
pub struct LoginInfo {
    pub email: String,
    pub password_raw: String,
    pub totp: String,
}

#[derive(Debug, Serialize)]
pub struct LoginResponse {
    pub token: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Claims {
    pub sub: String, // email
    pub id: i32,
    pub role: PersonRole,
}

#[derive(Debug, Deserialize)]
pub struct NewAccountForm {
    pub email: String,
    pub full_name: String,
    pub password_raw: String,
}

#[derive(Debug, Serialize, Deserialize, Type, PartialEq, Eq)]
#[sqlx(rename_all = "lowercase")]
#[sqlx(type_name = "person_role")]
pub enum PersonRole {
    Disabled,
    User,
    Admin,
}

#[derive(Debug, Serialize, Deserialize, Type)]
#[sqlx(rename_all = "lowercase")]
#[sqlx(type_name = "organization_person_role")]
pub enum OrganizationPersonRole {
    Read,
    ReadWrite,
    Admin,
}

#[derive(Debug, Serialize, Deserialize, Type)]
#[sqlx(rename_all = "snake_case")]
#[sqlx(type_name = "organization_category")]
pub enum OrganizationCategory {
    Activism,
    Bipoc,
    Lgbtq,
    Art,
    Music,
    Community,
    Labor,
    MutualAid,
}

#[derive(Debug, Serialize, Deserialize, Type)]
#[sqlx(rename_all = "lowercase")]
#[sqlx(type_name = "organizer_role")]
pub enum OrganizerRole {
    Coordinator,
    Teammember,
}

#[derive(Debug, Serialize, Deserialize, Type)]
#[sqlx(rename_all = "lowercase")]
#[sqlx(type_name = "event_organization_role")]
pub enum EventOrganizationRole {
    Vending,
    Performing,
    Tabling,
    Literature,
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Person {
    pub id: i32,
    pub full_name: Option<String>,
    pub email: Option<String>,
    pub phone_number_prefix: Option<String>,
    pub phone_number: Option<String>,
    pub phone_number_extension: Option<String>,
    #[serde(skip)]
    pub password_digest: Option<Vec<u8>>,
    #[serde(skip)]
    pub totp_secret: Option<Vec<u8>>,
    pub role: PersonRole,
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct OrganizationFull {
    pub organization: Organization,
    pub categories: Vec<OrganizationCategories>,
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Organization {
    pub id: i32,
    pub name: String,
    pub description: Option<String>,
    pub website: Option<String>,
    pub phone_number_prefix: Option<String>,
    pub phone_number: Option<String>,
    pub phone_number_extension: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct OrganizationCategories {
    pub organization_id: i32,
    pub category: OrganizationCategory,
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct OrganizationPersonJct {
    pub organization_id: i32,
    pub person_id: i32,
    pub organization_person_role: OrganizationPersonRole,
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Event {
    pub id: i32,
    pub name: String,
    pub theme: Option<String>,
    pub description: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct EventOrganizationJct {
    pub id: i32,
    pub event_id: i32,
    pub organizer_id: i32,
    pub role: Option<EventOrganizationRole>,
    pub contacted_on: Option<chrono::NaiveDateTime>,
    pub will_attend: bool,
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct EventPersonJct {
    pub id: i32,
    pub event_id: i32,
    pub person_id: i32,
    pub contacted_on: Option<chrono::NaiveDateTime>,
    pub will_attend: bool,
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct EventOrganizationAnnotation {
    pub id: i32,
    pub author_id: i32,
    pub event_organization_id: i32,
    pub content: String,
    pub is_deleted: bool,
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct EventPersonAnnotation {
    pub id: i32,
    pub author_id: i32,
    pub event_person_id: i32,
    pub content: String,
    pub is_deleted: bool,
}
