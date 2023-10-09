use actix_web::web::Json;
use async_std::task;

use crate::domain::user::services::{UserData, UserService};

use crate::infrastructure::repositories::user::UserRepository;

use crate::application::dto::user_dto::UserDto;
use crate::application::error::DataError;
use crate::application::mapper::transform_user_entity_to_dto::transform_user_entity_to_dto;
use crate::application::request_dto::update_user_dto::UpdateUserRequestBody;

pub async fn update_user_use_case(user: Json<UpdateUserRequestBody>) -> Result<UserDto, DataError> {
    let user_repository = UserRepository::new();

    let is_exist_response = user_repository.is_exist(&user.id).await;

    if is_exist_response.is_err() {
        return Err(is_exist_response.err().unwrap());
    }

    let is_exist = is_exist_response.unwrap();

    if !is_exist {
        let error = DataError::new("user doesnt exist");

        return Err(error);
    }

    let user_response = task::block_on(user_repository.get_user(&user.id));

    if user_response.is_err() {
        return Err(user_response.err().unwrap());
    }

    let mut user_entity = user_response.unwrap();

    let data = UserData {
        name: String::from(&user.name),
        last_name: String::from(&user.last_name),
    };

    let updated_user_response = UserService::new(user_repository)
        .update_user(&mut user_entity, data)
        .await;

    if updated_user_response.is_err() {
        return Err(updated_user_response.err().unwrap());
    }

    let updated_user = updated_user_response.unwrap();

    Ok(transform_user_entity_to_dto(updated_user))
}
