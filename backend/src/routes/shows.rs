use actix_multipart::form::{MultipartForm, bytes::Bytes, text::Text};
use actix_web::{HttpResponse, web};
use chrono::prelude::*;
use sqlx::PgPool;
use uuid::Uuid;
use validator::ValidateUrl;

use serde::{Deserialize, Serialize};
use std::str::FromStr;
use strum_macros::EnumString;

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
    city: Text<String>,
    date: Text<NaiveDate>,
    poster: Option<Bytes>,
    state: Text<String>,
    ticket_link: Text<String>,
    venue: Text<String>,
}

#[derive(Serialize)]
pub struct ShowRecord {
    id: Uuid,
    city: String,
    date: NaiveDate,
    poster: Option<Vec<u8>>,
    state: String,
    ticket_link: Option<String>,
    venue: String,
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

async fn validate(form: ShowFormData) -> Result<ShowRecord, std::io::Error> {
    let city = form.city.into_inner();
    match validate_city(&city) {
        true => (),
        false => {
            return Err(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                "Empty city field.",
            ));
        }
    };
    let poster = match &form.poster {
        Some(poster) => match validate_poster(poster) {
            Ok(()) => Some(poster.data.to_vec()),
            Err(e) => return Err(std::io::Error::new(std::io::ErrorKind::InvalidData, e)),
        },
        None => None,
    };
    let state = form.state.into_inner();
    match validate_state(&state) {
        true => (),
        false => {
            return Err(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                "Invalid state.",
            ));
        }
    };
    let ticket_link = form.ticket_link.into_inner();
    match ticket_link.validate_url() {
        true => (),
        false => {
            return Err(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                "Invalid ticket link.",
            ));
        }
    };
    let venue = form.venue.into_inner();
    match validate_venue(&venue) {
        Ok(()) => (),
        Err(e) => return Err(std::io::Error::new(std::io::ErrorKind::InvalidData, e)),
    };
    let record = ShowRecord {
        id: Uuid::new_v4(),
        city,
        date: form.date.into_inner(),
        poster,
        state,
        ticket_link: Some(ticket_link),
        venue,
    };
    Ok(record)
}

fn validate_city(city: &str) -> bool {
    !city.is_empty()
}

fn validate_state(state: &str) -> bool {
    USState::from_str(state).is_ok()
}

fn validate_venue(venue: &str) -> Result<(), std::io::Error> {
    let venue = venue.trim();

    if venue.is_empty() {
        return Err(std::io::Error::new(
            std::io::ErrorKind::InvalidData,
            "Empty venue field.",
        ));
    }
    Result::Ok(())
}

fn validate_poster(file: &Bytes) -> Result<(), std::io::Error> {
    let kind = match infer::get(file.data.as_ref()) {
        Some(kind) => kind,
        None => {
            return Err(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                "File not a valid pdf.",
            ));
        }
    };

    match (kind.mime_type() == "application/pdf")
        && (file
            .content_type
            .as_ref()
            .unwrap_or(&mime::TEXT_PLAIN)
            .type_()
            == "application")
        && (file
            .content_type
            .as_ref()
            .unwrap_or(&mime::TEXT_PLAIN)
            .subtype()
            == "pdf")
        && (file
            .file_name
            .as_ref()
            .unwrap_or(&"".to_string())
            .ends_with(".pdf"))
        && (!file.data.is_empty())
    {
        true => Ok(()),
        false => Err(std::io::Error::new(
            std::io::ErrorKind::InvalidData,
            "File not a valid pdf.",
        )),
    }
}
