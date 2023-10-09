use actix_web::web;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct CreateUserRequestBody {
    pub name: String,
    pub last_name: String,
}

impl CreateUserRequestBody {
    pub fn transform_from_json(user: web::Json<Self>) -> Self {
        let name = user.name.clone();
        let last_name = user.last_name.clone();
        Self { name, last_name }
    }
}
