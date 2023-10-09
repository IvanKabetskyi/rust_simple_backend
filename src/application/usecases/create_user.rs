use actix_web::web::Json;
use async_std::task;

use crate::domain::user::entities::User;

use crate::infrastructure::repositories::user::UserRepository;

use crate::application::dto::user_dto::UserDto;
use crate::application::mapper::transform_user_entity_to_dto::transform_user_entity_to_dto;
use crate::application::request_dto::create_user_dto::CreateUserRequestBody;

pub fn create_user_use_case(user: Json<CreateUserRequestBody>) -> UserDto {
    let request = CreateUserRequestBody::transform_from_json(user);

    let user = User::new(request.name.as_str(), request.last_name.as_str(), None);

    let user_repository = UserRepository::new();
    let user_entity = task::block_on(user_repository.save_user(&user));

    transform_user_entity_to_dto(user_entity)
}
