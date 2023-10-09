mod schema;

use async_std::task;
use mongodb::{
    bson::{doc, oid::ObjectId},
    Collection,
};

use crate::domain::user::entities::User;

use crate::infrastructure::services::database::DB;

use crate::application::error::DataError;

pub use schema::UserSchema;

pub struct UserRepository {
    collection: Collection<UserSchema>,
}

impl UserRepository {
    pub fn new() -> Self {
        let database = task::block_on(DB.lock());

        Self {
            collection: database.db.collection::<UserSchema>("user"),
        }
    }

    pub async fn save_user(&self, user: &User) -> User {
        let document = UserSchema {
            id: user.get_id(),
            name: user.get_name(),
            last_name: user.get_last_name(),
        };

        let result = self.collection.insert_one(document, None).await.unwrap();

        let user = self
            .collection
            .find_one(Some(doc! {"_id": result.inserted_id.as_object_id()}), None)
            .await
            .unwrap()
            .unwrap();

        User::new(user.name.as_str(), user.last_name.as_str(), Some(user.id))
    }

    pub async fn update_user(&self, user: &User) -> Result<User, DataError> {
        let filter = doc! {"_id": &user.get_id()};
        let update = doc! {"$set": {"name": user.get_name(),
        "last_name": user.get_last_name(),}};

        let response = self.collection.update_one(filter, update, None).await;

        if response.is_err() {
            let error = DataError::new("doesnt update data");
            return Err(error);
        }

        let user = self
            .collection
            .find_one(Some(doc! {"_id": &user.get_id()}), None)
            .await
            .unwrap()
            .unwrap();

        Ok(User::new(
            user.name.as_str(),
            user.last_name.as_str(),
            Some(user.id),
        ))
    }

    pub async fn get_user(&self, id: &String) -> Result<User, DataError> {
        let object_id = ObjectId::parse_str(id);

        if object_id.is_err() {
            let error = DataError::new("not valid id");
            return Err(error);
        }

        let user = self
            .collection
            .find_one(Some(doc! {"_id": object_id.unwrap()}), None)
            .await
            .unwrap()
            .unwrap();

        Ok(User::new(
            user.name.as_str(),
            user.last_name.as_str(),
            Some(user.id),
        ))
    }

    pub async fn is_exist(&self, id: &String) -> Result<bool, DataError> {
        let id_response = ObjectId::parse_str(id);

        if id_response.is_err() {
            let error = DataError::new("not valid id");
            return Err(error);
        }

        let id = id_response.unwrap();

        let user = self.collection.find_one(Some(doc! {"_id": id}), None).await;

        if user.is_err() {
            let error = DataError::new("Database error");
            return Err(error);
        }

        Ok(user.unwrap().is_some())
    }
}
