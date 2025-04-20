use crate::helper::spawn_app;
use backend::routes::ShowRecord;

use chrono::NaiveDate;
use reqwest::{Client, multipart};

// fields for a valid show record
const CITY: &str = "Somerville";
const DATE: &str = "2025-04-14";
const STATE: &str = "MA";
const TICKET_LINK: &str = "https://jackflores.com/";
const VENUE: &str = "The Jungle";

#[tokio::test]
async fn get_shows_empty_db_returns_200() {
    let app = spawn_app().await;
    let client = Client::new();

    let response = client
        .get(&format!("{}/shows", &app.address))
        .send()
        .await
        .expect("Failed to execute request.");

    assert_eq!(200, response.status().as_u16());
    assert_eq!("[]", response.text().await.unwrap());
}

#[tokio::test]
async fn get_shows_returns_200_with_all_fields() {
    let app = spawn_app().await;
    let client = Client::new();

    let form = multipart::Form::new()
        .text("city", CITY)
        .text("date", DATE)
        .text("state", STATE)
        .text("ticket_link", TICKET_LINK)
        .text("venue", VENUE);

    client
        .post(&format!("{}/shows", &app.address))
        .header(
            "Content-Type",
            format!("multipart/form-data; boundary={}", form.boundary()),
        )
        .multipart(form)
        .send()
        .await
        .expect("Failed to execute request.");

    let response = client
        .get(&format!("{}/shows", &app.address))
        .send()
        .await
        .expect("Failed to execute request.");

    assert_eq!(200, response.status().as_u16());
    let shows = response.json::<Vec<ShowRecord>>().await.unwrap();
    assert_eq!(shows[0].city, CITY);
    assert_eq!(shows[0].date, NaiveDate::from_ymd_opt(2025, 4, 14).unwrap());
    assert_eq!(shows[0].state, STATE);
    assert_eq!(shows[0].ticket_link.clone().unwrap(), TICKET_LINK);
    assert_eq!(shows[0].venue, VENUE);
}

#[tokio::test]
async fn get_shows_no_optional_fields_returns_200() {
    let app = spawn_app().await;
    let client = Client::new();

    let form = multipart::Form::new()
        .text("city", CITY)
        .text("date", DATE)
        .text("state", STATE)
        .text("venue", VENUE);

    client
        .post(&format!("{}/shows", &app.address))
        .header(
            "Content-Type",
            format!("multipart/form-data; boundary={}", form.boundary()),
        )
        .multipart(form)
        .send()
        .await
        .expect("Failed to execute request.");

    let response = client
        .get(&format!("{}/shows", &app.address))
        .send()
        .await
        .expect("Failed to execute request.");

    assert_eq!(200, response.status().as_u16());
    let shows = response.json::<Vec<ShowRecord>>().await.unwrap();
    assert_eq!(shows[0].city, CITY);
    assert_eq!(shows[0].date, NaiveDate::from_ymd_opt(2025, 4, 14).unwrap());
    assert_eq!(shows[0].state, STATE);
    assert!(shows[0].ticket_link.clone().is_none());
    assert_eq!(shows[0].venue, VENUE);
}
