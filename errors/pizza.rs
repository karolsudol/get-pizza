use actix_web::{http::header::ContentType, HttpResponse, ResponseError};
use serde_json::json;

use derive_more::Display;
#[derive(Debug, Display)]
pub enum PizzaError {
    NoPizzasFound,
    PizzaCreationFailure,
    NoSuchPizzaFound,
}

impl ResponseError for PizzaError {
    fn error_response(&self) -> HttpResponse {
        let status_code = self.status_code();
        let error_message = self.to_string();

        HttpResponse::build(status_code)
            .insert_header(ContentType::json())
            .json(json!({
                "error": error_message,
            }))
    }

    fn status_code(&self) -> StatusCode {
        match self {
            PizzaError::NoPizzasFound => StatusCode::NOT_FOUND,
            PizzaError::PizzaCreationFailure => StatusCode::INTERNAL_SERVER_ERROR,
            PizzaError::NoSuchPizzaFound => StatusCode::NOT_FOUND,
        }
    }
}
