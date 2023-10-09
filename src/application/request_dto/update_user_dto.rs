use lazy_static::lazy_static;
use regex::Regex;
use serde::Deserialize;
use validator::Validate;

lazy_static! {
    static ref MONGO_OBJECT_ID: Regex = Regex::new(r"[a-fA-F0-9]{24}").unwrap();
}

#[derive(Deserialize, Validate)]
pub struct UpdateUserRequestBody {
    pub name: String,
    pub last_name: String,
    #[validate(regex(path = "MONGO_OBJECT_ID"))]
    pub id: String,
}
