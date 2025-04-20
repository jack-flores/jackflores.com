use crate::routes::ShowFormData;
use crate::routes::ShowRecord;
use crate::routes::USState;

use actix_multipart::form::bytes::Bytes;
use std::str::FromStr;
use uuid::Uuid;
use validator::ValidateUrl;

pub async fn validate(form: ShowFormData) -> Result<ShowRecord, std::io::Error> {
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
    let ticket_link = match form.ticket_link {
        Some(ticket_link) => match ticket_link.clone().validate_url() {
            true => Some(ticket_link.into_inner()),
            false => {
                return Err(std::io::Error::new(
                    std::io::ErrorKind::InvalidData,
                    "Invalid ticket link.",
                ));
            }
        },
        None => None,
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
        ticket_link,
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
    let err_msg = "Poster file not a valid pdf.";
    let kind = match infer::get(file.data.as_ref()) {
        Some(kind) => kind,
        None => {
            return Err(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                err_msg,
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
            err_msg,
        )),
    }
}
