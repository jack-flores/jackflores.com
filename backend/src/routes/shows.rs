use actix_multipart::form::{MultipartForm, bytes::Bytes, text::Text};
use actix_web::{HttpResponse, web};
use chrono::prelude::*;
use sqlx::PgConnection;
use uuid::Uuid;

#[derive(MultipartForm)]
pub struct ShowData {
    date: Text<NaiveDateTime>,
    venue: Text<String>,
}

pub async fn post_show(
    MultipartForm(form): MultipartForm<ShowData>,
    connection: web::Data<PgConnection>,
) -> HttpResponse {
    sqlx::query!(
        r#"
        INSERT INTO shows (id, date, venue)
        VALUES ($1, $2, $3)
        "#,
        Uuid::new_v4(),
        *form.date,
        &form.venue
    )
    .execute(connection.get_ref())
    .await;
    HttpResponse::Ok().finish()
}
