use actix_web::{
    get, post, put,
    web::{Json, Path},
    Result,
};

use crate::application::dto::user_dto::UserDto;

use crate::application::error::DataError;

use crate::application::request_dto::create_user_dto::CreateUserRequestBody;
use crate::application::request_dto::update_user_dto::UpdateUserRequestBody;

use crate::application::usecases::create_user::create_user_use_case;
use crate::application::usecases::get_user::get_user_use_case;
use crate::application::usecases::update_user::update_user_use_case;

#[get("/user/{id}")]
pub async fn get_user(path: Path<String>) -> Result<Json<UserDto>, DataError> {
    let user_id = path.into_inner();

    let response = get_user_use_case(user_id);

    if response.is_err() {
        return Err(response.err().unwrap());
    }

    let user_dto = response.unwrap();

    Ok(Json(user_dto))
}

#[post("/user")]
pub async fn create_user(user: Json<CreateUserRequestBody>) -> Result<Json<UserDto>> {
    let user_dto = create_user_use_case(user);
    Ok(Json(user_dto))
}

#[put("/user")]
pub async fn update_user(user: Json<UpdateUserRequestBody>) -> Result<Json<UserDto>, DataError> {
    let response = update_user_use_case(user).await;

    if response.is_err() {
        return Err(response.err().unwrap());
    }

    let user_dto = response.unwrap();

    Ok(Json(user_dto))
}
