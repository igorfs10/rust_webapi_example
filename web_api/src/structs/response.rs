use data::serde::{self, Deserialize, Serialize};
#[derive(Serialize, Deserialize)]
#[serde(crate = "self::serde")]
pub struct Response<T> {
    pub message: String,
    pub result: Option<T>,
}

impl<T> Response<T> {
    pub fn new_response(message: String, result: Option<T>) -> Self {
        Response { message, result }
    }
}
