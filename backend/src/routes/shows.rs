use actix_multipart::form::{MultipartForm, bytes::Bytes, text::Text};
use actix_web::{HttpResponse, web};
use chrono::prelude::*;
use sqlx::PgPool;
use uuid::Uuid;

#[derive(MultipartForm)]
pub struct ShowFormData {
    date: Text<NaiveDateTime>,
    poster: Option<Bytes>,
    venue: Text<String>,
}

pub struct ShowRecord {
    date: NaiveDateTime,
    poster: Option<Vec<u8>>,
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
        INSERT INTO shows (id, date, poster, venue)
        VALUES ($1, $2, $3, $4)
        "#,
        Uuid::new_v4(),
        record.date,
        record.poster,
        &record.venue
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

async fn validate(form: ShowFormData) -> Result<ShowRecord, std::io::Error> {
    let poster = match &form.poster {
        Some(poster) => match validate_pdf(poster) {
            Ok(()) => Some(poster.data.to_vec()),
            Err(e) => return Err(std::io::Error::new(std::io::ErrorKind::InvalidData, e)),
        },
        None => None,
    };
    let record = ShowRecord {
        date: form.date.into_inner(),
        poster,
        venue: form.venue.into_inner(),
    };
    Ok(record)
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
