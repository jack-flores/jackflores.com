use crate::helper::spawn_app;

use chrono::NaiveDate;
use reqwest::{Client, multipart};

// fields for a valid show record
const CITY: &str = "Somerville";
const DATE: &str = "2025-04-14";
const STATE: &str = "MA";
const TICKET_LINK: &str = "https://jackflores.com/";
const VENUE: &str = "The Jungle";

#[tokio::test]
async fn post_shows_returns_201_for_valid_form_data() {
    let app = spawn_app().await;
    let client = Client::new();

    let file_bytes = tokio::fs::read("tests/test_poster.pdf")
        .await
        .expect("Failed to read test file");

    let test_file = multipart::Part::bytes(file_bytes.clone())
        .file_name("test_poster.pdf")
        .mime_str("application/pdf")
        .unwrap();

    let form = multipart::Form::new()
        .text("city", CITY)
        .text("date", DATE)
        .part("poster", test_file)
        .text("state", STATE)
        .text("ticket_link", TICKET_LINK)
        .text("venue", VENUE);

    let response = client
        .post(&format!("{}/shows", &app.address))
        .header(
            "Content-Type",
            format!("multipart/form-data; boundary={}", form.boundary()),
        )
        .multipart(form)
        .send()
        .await
        .expect("Failed to execute request.");

    assert_eq!(201, response.status().as_u16());

    let saved = sqlx::query!("SELECT city, date, poster, state, ticket_link, venue FROM shows",)
        .fetch_one(&app.db_pool)
        .await
        .expect("Failed to fetch saved show.");

    assert_eq!(saved.city, CITY);
    assert_eq!(saved.date, NaiveDate::from_ymd_opt(2025, 4, 14).unwrap());
    assert_eq!(saved.poster.unwrap(), file_bytes.clone());
    assert_eq!(saved.state, STATE);
    assert_eq!(saved.ticket_link.unwrap(), TICKET_LINK);
    assert_eq!(saved.venue, VENUE);
}

#[tokio::test]
async fn post_shows_returns_400_for_bad_part_types() {
    let app = spawn_app().await;
    let client = Client::new();

    // city field empty
    let form = multipart::Form::new()
        .text("date", DATE)
        .text("city", "")
        .text("state", STATE)
        .text("ticket_link", TICKET_LINK)
        .text("venue", VENUE);

    let response = client
        .post(&format!("{}/shows", &app.address))
        .header(
            "Content-Type",
            format!("multipart/form-data; boundary={}", form.boundary()),
        )
        .multipart(form)
        .send()
        .await
        .expect("Failed to execute request.");

    assert_eq!(400, response.status().as_u16());

    // date not a NaiveDate
    let bad_date = "04-15".to_string(); // missing year

    let form = multipart::Form::new()
        .text("date", bad_date)
        .text("city", CITY)
        .text("state", STATE)
        .text("ticket_link", TICKET_LINK)
        .text("venue", VENUE);

    let response = client
        .post(&format!("{}/shows", &app.address))
        .header(
            "Content-Type",
            format!("multipart/form-data; boundary={}", form.boundary()),
        )
        .multipart(form)
        .send()
        .await
        .expect("Failed to execute request.");

    assert_eq!(400, response.status().as_u16());

    // ticket_link not a valid link
    let form = multipart::Form::new()
        .text("date", DATE)
        .text("city", CITY)
        .text("state", STATE)
        .text("ticket_link", "not a link")
        .text("venue", VENUE); // passing a file instead of a string

    let response = client
        .post(&format!("{}/shows", &app.address))
        .header(
            "Content-Type",
            format!("multipart/form-data; boundary={}", form.boundary()),
        )
        .multipart(form)
        .send()
        .await
        .expect("Failed to execute request.");

    assert_eq!(400, response.status().as_u16());

    // bad file type
    let file_bytes = tokio::fs::read("tests/bad_file.txt")
        .await
        .expect("Failed to read test file");

    let test_file = multipart::Part::bytes(file_bytes)
        .file_name("bad_file.txt")
        .mime_str("text/plain")
        .unwrap();

    let form = multipart::Form::new()
        .text("date", DATE)
        .text("city", CITY)
        .part("poster", test_file)
        .text("state", STATE)
        .text("ticket_link", TICKET_LINK)
        .text("venue", VENUE);

    let response = client
        .post(&format!("{}/shows", &app.address))
        .header(
            "Content-Type",
            format!("multipart/form-data; boundary={}", form.boundary()),
        )
        .multipart(form)
        .send()
        .await
        .expect("Failed to execute request.");

    assert_eq!(400, response.status().as_u16());

    // invalid state
    let form = multipart::Form::new()
        .text("date", DATE)
        .text("city", CITY)
        .text("state", "AA") // fake state code
        .text("ticket_link", TICKET_LINK)
        .text("venue", VENUE);

    let response = client
        .post(&format!("{}/shows", &app.address))
        .header(
            "Content-Type",
            format!("multipart/form-data; boundary={}", form.boundary()),
        )
        .multipart(form)
        .send()
        .await
        .expect("Failed to execute request.");

    assert_eq!(400, response.status().as_u16());

    // venue not a string
    let file_bytes_clone = tokio::fs::read("tests/test_poster.pdf")
        .await
        .expect("Failed to read test file");

    let test_file_clone = multipart::Part::bytes(file_bytes_clone)
        .file_name("test_poster.pdf")
        .mime_str("application/pdf")
        .unwrap();

    let form = multipart::Form::new()
        .text("date", DATE)
        .text("city", CITY)
        .text("state", STATE)
        .text("ticket_link", TICKET_LINK)
        .part("venue", test_file_clone); // passing a file instead of a string

    let response = client
        .post(&format!("{}/shows", &app.address))
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

#[tokio::test]
async fn post_shows_excluding_optional_poster_file_returns_201() {
    let app = spawn_app().await;
    let client = Client::new();

    let form = multipart::Form::new()
        .text("city", CITY)
        .text("date", DATE)
        .text("state", STATE)
        .text("ticket_link", TICKET_LINK)
        .text("venue", VENUE);

    let response = client
        .post(&format!("{}/shows", &app.address))
        .header(
            "Content-Type",
            format!("multipart/form-data; boundary={}", form.boundary()),
        )
        .multipart(form)
        .send()
        .await
        .expect("Failed to execute request.");

    assert_eq!(201, response.status().as_u16());
}

#[tokio::test]
async fn post_shows_with_missing_data_returns_400() {
    let app = spawn_app().await;
    let client = Client::new();

    // no date
    let form = multipart::Form::new()
        .text("city", CITY)
        .text("state", STATE)
        .text("ticket_link", TICKET_LINK)
        .text("venue", VENUE);

    let response = client
        .post(&format!("{}/shows", &app.address))
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
    let form = multipart::Form::new()
        .text("city", CITY)
        .text("date", DATE)
        .text("state", STATE)
        .text("ticket_link", TICKET_LINK);

    let response = client
        .post(&format!("{}/shows", &app.address))
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
        .text("city", CITY)
        .text("date", "bad date")
        .text("state", STATE)
        .text("ticket_link", TICKET_LINK)
        .text("venue", VENUE);

    let response = client
        .post(&format!("{}/shows", &app.address))
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
