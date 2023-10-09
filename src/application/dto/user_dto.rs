use serde::Serialize;

use crate::domain::user::entities::User;

#[derive(Debug, Serialize)]
pub struct UserDto {
    name: String,
    last_name: String,
    id: String,
}

impl UserDto {
    pub fn transform_entity(user: User) -> Self {
        Self {
            name: user.get_name(),
            last_name: user.get_last_name(),
            id: user.get_id().to_hex(),
        }
    }
}
