use mongodb::bson::oid::ObjectId;

pub struct User {
    id: ObjectId,
    name: String,
    last_name: String,
}

impl User {
    pub fn new(name: &str, last_name: &str, id: Option<ObjectId>) -> Self {
        Self {
            id: id.unwrap_or(ObjectId::new()),
            name: String::from(name),
            last_name: String::from(last_name),
        }
    }

    pub fn get_name(&self) -> String {
        self.name.clone()
    }

    pub fn get_last_name(&self) -> String {
        self.last_name.clone()
    }

    pub fn get_id(&self) -> ObjectId {
        self.id.clone()
    }

    pub fn update_name(&mut self, name: String) {
        self.name = name;
    }

    pub fn update_last_name(&mut self, last_name: String) {
        self.last_name = last_name;
    }
}
