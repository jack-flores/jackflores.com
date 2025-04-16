use actix_multipart::form::{MultipartForm, bytes::Bytes, text::Text};
use actix_web::{HttpResponse, web};
use chrono::prelude::*;
use sqlx::PgPool;
use uuid::Uuid;

#[derive(MultipartForm)]
pub struct ShowFormData {
    city: Text<String>,
    date: Text<NaiveDate>,
    poster: Option<Bytes>,
    state: Text<String>,
    ticket_link: Text<String>,
    venue: Text<String>,
}

pub struct ShowRecord {
    city: String,
    date: NaiveDate,
    poster: Option<Vec<u8>>,
    state: String,
    ticket_link: String,
    venue: String,
}

pub async fn post_show(
    MultipartForm(form): MultipartForm<ShowFormData>,
    pool: web::Data<PgPool>,
) -> HttpResponse {
    let record = match validate(form).await {
        Ok(record) => record,
        Err(_) => return HttpResponse::BadRequest().finish(),
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
        Ok(_) => HttpResponse::Ok().finish(),
        Err(e) => {
            println!("Failed to execute query: {}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
}

// TODO
// pub async fn get_all_shows() {

// }

async fn validate(form: ShowFormData) -> Result<ShowRecord, std::io::Error> {
    let city = form.city.into_inner();
    let poster = match &form.poster {
        Some(poster) => match validate_pdf(poster) {
            Ok(()) => Some(poster.data.to_vec()),
            Err(e) => return Err(std::io::Error::new(std::io::ErrorKind::InvalidData, e)),
        },
        None => None,
    };
    let state = form.state.into_inner();
    let ticket_link = form.ticket_link.into_inner();
    let venue = form.venue.into_inner();
    let venue = match validate_venue(&venue) {
        Ok(()) => venue,
        Err(e) => return Err(std::io::Error::new(std::io::ErrorKind::InvalidData, e)),
    };
    let record = ShowRecord {
        city,
        date: form.date.into_inner(),
        poster,
        state,
        ticket_link,
        venue,
    };
    Ok(record)
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

pub fn validate_pdf(file: &Bytes) -> Result<(), std::io::Error> {
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
