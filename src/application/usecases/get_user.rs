use async_std::task;

use crate::infrastructure::repositories::user::UserRepository;

use crate::application::dto::user_dto::UserDto;
use crate::application::error::DataError;
use crate::application::mapper::transform_user_entity_to_dto::transform_user_entity_to_dto;

pub fn get_user_use_case(user_id: String) -> Result<UserDto, DataError> {
    let user_repository = UserRepository::new();
    let user_dto_response = task::block_on(user_repository.get_user(&user_id));

    if user_dto_response.is_err() {
        return Err(user_dto_response.err().unwrap());
    }
    let user = user_dto_response.unwrap();

    Ok(transform_user_entity_to_dto(user))
}
