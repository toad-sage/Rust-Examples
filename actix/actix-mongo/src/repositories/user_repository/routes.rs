use super::user_repository::{IUserRepository, UserRepository};
use crate::db::db::{Connection, IConnnection};
use crate::models::user::Register;
use actix_web::{post, web, HttpResponse};

#[post("/register")]
async fn register(user: web::Json<Register>) -> HttpResponse {
    let _connection: Connection = Connection {};
    let _repository: UserRepository = UserRepository {
        connection: _connection.init(),
    };
    println!("{:?}", _repository);
    HttpResponse::Ok().json(_repository.register(user.into_inner()))
}

pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(register);
}
