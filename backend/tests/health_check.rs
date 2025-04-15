use backend::configuration::get_configuration;
use backend::startup::run;

use actix_multipart::form::{MultipartForm, bytes::Bytes, text::Text};
use chrono::{NaiveDate, NaiveDateTime, NaiveTime};
use reqwest::{Client, multipart};
use sqlx::{Connection, PgConnection};
use std::net::TcpListener;

fn spawn_app() -> String {
    let listener = TcpListener::bind("127.0.0.1:0").expect("failed to bind to random port");
    let port = listener.local_addr().unwrap().port();
    let server = run(listener).expect("failed to bind address");
    let _ = tokio::spawn(server);

    format!("http://127.0.0.1:{}", port)
}

#[tokio::test]
async fn health_check_works() {
    let app_address = spawn_app();
    let client = Client::new();

    let response = client
        .get(&format!("{}/health_check", &app_address))
        .send()
        .await
        .expect("failed to make request");

    assert!(response.status().is_success());
    assert_eq!(response.content_length(), Some(0));
}

#[tokio::test]
async fn post_shows_returns_200_for_valid_form_data() {
    let app_address = spawn_app();
    let configuration = get_configuration().expect("Failed to read configuration");
    let connection_string = configuration.database.connection_string();
    let mut connection = PgConnection::connect(&connection_string)
        .await
        .expect("Failed to connect to Postgres.");
    let client = Client::new();

    let d = NaiveDate::from_ymd_opt(2025, 4, 14).unwrap();
    let t = NaiveTime::from_hms_opt(23, 17, 0).unwrap();
    let dt = NaiveDateTime::new(d, t);
    let date = dt.format("%Y-%m-%dT%H:%M:%S").to_string();
    let venue = "The Jungle";

    // let file_bytes = tokio::fs::read("tests/test_poster.pdf")
    //     .await
    //     .expect("Failed to read test file");

    // let test_file = multipart::Part::bytes(file_bytes)
    //     .file_name("test_poster.pdf")
    //     .mime_str("application/pdf")
    //     .unwrap();

    let form = multipart::Form::new()
        .text("date", date)
        .text("venue", venue);
    // .part("poster_file", test_file);

    let response = client
        .post(&format!("{}/shows", &app_address))
        .header(
            "Content-Type",
            format!("multipart/form-data; boundary={}", form.boundary()),
        )
        .multipart(form)
        .send()
        .await
        .expect("Failed to execute request.");

    assert_eq!(200, response.status().as_u16());

    let saved = sqlx::query!("SELECT date, venue FROM shows",)
        .fetch_one(&mut connection)
        .await
        .expect("Failed to fetch saved show.");

    assert_eq!(saved.date, dt);
    assert_eq!(saved.venue, venue);
}

// #[tokio::test]
// async fn post_shows_excluding_optional_poster_file_returns_200() {
//     let app_address = spawn_app();
//     let client = Client::new();

//     let form = multipart::Form::new()
//         .text("date", "2024-04-14T16:42:00")
//         .text("venue", "The Jungle");

//     let response = client
//         .post(&format!("{}/shows", &app_address))
//         .header(
//             "Content-Type",
//             format!("multipart/form-data; boundary={}", form.boundary()),
//         )
//         .multipart(form)
//         .send()
//         .await
//         .expect("Failed to execute request.");

//     assert_eq!(200, response.status().as_u16());
// }

#[tokio::test]
async fn post_shows_with_missing_data_returns_400() {
    let app_address = spawn_app();
    let client = Client::new();

    // no date
    let form = multipart::Form::new().text("venue", "The Jungle");

    let response = client
        .post(&format!("{}/shows", &app_address))
        .header(
            "Content-Type",
            format!("multipart/form-data; boundary={}", form.boundary()),
        )
        .multipart(form)
        .send()
        .await
        .expect("Failed to execute request.");

    assert_eq!(400, response.status().as_u16());

    // no venue
    let form = multipart::Form::new().text("date", "2024-04-14T16:42:00");

    let response = client
        .post(&format!("{}/shows", &app_address))
        .header(
            "Content-Type",
            format!("multipart/form-data; boundary={}", form.boundary()),
        )
        .multipart(form)
        .send()
        .await
        .expect("Failed to execute request.");

    assert_eq!(400, response.status().as_u16());

    // date bad format
    let form = multipart::Form::new()
        .text("venue", "The Jungle")
        .text("date", "2024-04-14");

    let response = client
        .post(&format!("{}/shows", &app_address))
        .header(
            "Content-Type",
            format!("multipart/form-data; boundary={}", form.boundary()),
        )
        .multipart(form)
        .send()
        .await
        .expect("Failed to execute request.");

    assert_eq!(400, response.status().as_u16());
}
