use crate::application::error::DataError;
use crate::domain::user::entities::User;
use crate::infrastructure::repositories::user::UserRepository;

pub struct UserData {
    pub name: String,
    pub last_name: String,
}

pub struct UserService {
    user_repository: UserRepository,
}

impl UserService {
    pub fn new(user_repository: UserRepository) -> Self {
        Self { user_repository }
    }

    pub async fn update_user(&self, user: &mut User, data: UserData) -> Result<User, DataError> {
        user.update_name(data.name);
        user.update_last_name(data.last_name);

        let updated_user = self.user_repository.update_user(&user).await;

        updated_user
    }
}
