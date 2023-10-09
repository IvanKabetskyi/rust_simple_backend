use mongodb::bson::{doc, oid::ObjectId};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct UserSchema {
    #[serde(rename = "_id")]
    pub id: ObjectId,
    pub name: String,
    pub last_name: String,
}
