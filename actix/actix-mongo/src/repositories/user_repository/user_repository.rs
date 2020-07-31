use crate::config::{Config, IConfig};
use crate::models::response::Response;
use crate::models::user::Register;
use bson::doc;
use crypto::digest::Digest;
use crypto::sha2::Sha256;
use mongodb::Client;

pub trait IUserRepository {
    fn register(&self, user: Register) -> Response;
}
#[derive(Debug)]
pub struct UserRepository {
    pub connection: &'static Client,
}

impl IUserRepository for UserRepository {
    fn register(&self, user: Register) -> Response {
        let _config: Config = Config {};
        let database_name = _config.get_config_with_key("DATABASE_NAME");
        let collection_name = _config.get_config_with_key("USER_COLLECTION_NAME");
        println!("{} {}", database_name, collection_name);
        let db = self.connection.database(database_name.as_str());

        let mut sha = Sha256::new();
        sha.input_str(user.password.as_str());
        let hash_pw = sha.result_str();
        let user_id = uuid::Uuid::new_v4().to_string();
        let _ex = db.collection(collection_name.as_str()).insert_one(doc! {"user_id": user_id, "name": user.name, "surname": user.surname, "email": user.email, "password": hash_pw, "phone": "", "birth_date": "" }, None);

        match _ex {
            Ok(_) => Response {
                status: true,
                message: "Register Successful".to_string(),
            },
            Err(_) => Response {
                status: false,
                message: "Something Went Wrong".to_string(),
            },
        }
    }
}
