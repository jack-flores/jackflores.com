use crate::util::validate;

use actix_multipart::form::{MultipartForm, bytes::Bytes, text::Text};
use actix_web::{HttpResponse, web};
use chrono::prelude::*;
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use strum_macros::EnumString;
use uuid::Uuid;

#[derive(Debug, EnumString, Serialize, Deserialize)]
#[strum(serialize_all = "UPPERCASE")]
pub enum USState {
    AL,
    AK,
    AZ,
    AR,
    CA,
    CO,
    CT,
    DE,
    FL,
    GA,
    HI,
    ID,
    IL,
    IN,
    IA,
    KS,
    KY,
    LA,
    ME,
    MD,
    MA,
    MI,
    MN,
    MS,
    MO,
    MT,
    NE,
    NV,
    NH,
    NJ,
    NM,
    NY,
    NC,
    ND,
    OH,
    OK,
    OR,
    PA,
    RI,
    SC,
    SD,
    TN,
    TX,
    UT,
    VT,
    VA,
    WA,
    WV,
    WI,
    WY,
}

#[derive(MultipartForm)]
pub struct ShowFormData {
    pub city: Text<String>,
    pub date: Text<NaiveDate>,
    pub poster: Option<Bytes>,
    pub state: Text<String>,
    pub ticket_link: Option<Text<String>>,
    pub venue: Text<String>,
}

#[derive(Deserialize, Serialize)]
pub struct ShowRecord {
    pub id: Uuid,
    pub city: String,
    pub date: NaiveDate,
    pub poster: Option<Vec<u8>>,
    pub state: String,
    pub ticket_link: Option<String>,
    pub venue: String,
}

pub async fn post_show(
    MultipartForm(form): MultipartForm<ShowFormData>,
    pool: web::Data<PgPool>,
) -> HttpResponse {
    let record = match validate(form).await {
        Ok(record) => record,
        Err(e) => return HttpResponse::BadRequest().body(format!("{}", e)),
    };
    match sqlx::query!(
        r#"
        INSERT INTO shows (id, city, date, poster, state, ticket_link, venue)
        VALUES ($1, $2, $3, $4, $5, $6, $7)
        "#,
        Uuid::new_v4(),
        record.city,
        record.date,
        record.poster,
        record.state,
        record.ticket_link,
        record.venue
    )
    .execute(pool.get_ref())
    .await
    {
        Ok(_) => HttpResponse::Created().body("Show posted."),
        Err(e) => {
            println!("Failed to execute query: {}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
}

pub async fn get_all_shows(pool: web::Data<PgPool>) -> HttpResponse {
    match sqlx::query_as!(
        ShowRecord,
        r#"
        SELECT 
            id,
            city,
            date,
            poster,
            state,
            ticket_link,
            venue
        FROM shows;
        "#
    )
    .fetch_all(pool.get_ref())
    .await
    {
        Ok(records) => HttpResponse::Ok().json(records),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}
