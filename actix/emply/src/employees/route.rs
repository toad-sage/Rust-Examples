use super::model::{Employee, Employees};
use crate::error_handler::CustomError;
use actix_web::{delete, get, post, put, web, HttpResponse};
use serde_json::json;

type ResponseType = Result<HttpResponse, CustomError>;

#[get("/employees")]
async fn find_all() -> ResponseType {
    let employees = Employees::find_all()?;
    Ok(HttpResponse::Ok().json(employees))
}

#[get("/employees/{id}")]
async fn find(id: web::Path<i32>) -> ResponseType {
    let employee = Employees::find(id.into_inner())?;
    Ok(HttpResponse::Ok().json(employee))
}

#[post("/employees")]
async fn create(employee: web::Json<Employee>) -> ResponseType {
    let employee = Employees::create(employee.into_inner())?;
    Ok(HttpResponse::Ok().json(employee))
}

#[put("/employees/{id}")]
async fn update(id: web::Path<i32>, employee: web::Json<Employee>) -> ResponseType {
    let employee = Employees::update(id.into_inner(), employee.into_inner())?;
    Ok(HttpResponse::Ok().json(employee))
}

#[delete("/employees/{id}")]
async fn delete(id: web::Path<i32>) -> ResponseType {
    let deleted_employee = Employees::delete(id.into_inner())?;
    Ok(HttpResponse::Ok().json(json!({ "deleted": deleted_employee })))
}

pub fn init_routes(config: &mut web::ServiceConfig) {
    config.service(find);
    config.service(find_all);
    config.service(create);
    config.service(update);
    config.service(delete);
}
