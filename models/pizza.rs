use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Validate, Deserialize, Serialize)]
pub struct BuyPizzaRequest {
    #[validate(length(min = 1, message = "uuid required"))]
    pub pizza: String,
}

#[derive(Validate, Deserialize, Serialize)]
pub struct UpdatePizzaURL {
    pub uuid: String,
}

#[derive(Validate, Deserialize, Serialize, Debug)]
pub struct Pizza {
    pub uuid: String,
    pub name: String,
}

impl Pizza {
    pub fn new(uuid: String) -> Pizza {
        Pizza { uuid }
    }
}