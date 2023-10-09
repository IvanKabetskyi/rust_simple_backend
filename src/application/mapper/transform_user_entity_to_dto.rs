use crate::domain::user::entities::User;

use crate::application::dto::user_dto::UserDto;

pub fn transform_user_entity_to_dto(user: User) -> UserDto {
    UserDto::transform_entity(user)
}
