use crate::db;
use crate::error_handler::CustomError;
use crate::schema::teachers;
use chrono::{DateTime, Duration, Utc};
use crypto::digest::Digest;
use crypto::sha2::Sha256;
use diesel::prelude::*;
use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, AsChangeset, Insertable)]
#[table_name = "teachers"]
pub struct Teacher {
    pub email: String,
    pub password: String,
    pub first_name: String,
    pub last_name: String,
    pub designation: String,
    pub department: String,
    pub salary: i32,
    pub age: i32,
}

#[derive(Serialize, Deserialize, Queryable)]
pub struct Teachers {
    pub id: i32,
    pub email: String,
    pub password: String,
    pub first_name: String,
    pub last_name: String,
    pub designation: String,
    pub department: String,
    pub salary: i32,
    pub age: i32,
}

#[derive(Deserialize)]
pub struct Login {
    email: String,
    password: String,
    #[serde(default)]
    remember_me: bool,
}

#[derive(Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,
    pub exp: usize,
}

#[derive(Serialize, Deserialize)]
pub struct LoginResponse {
    pub message: String,
    pub status: bool,
    pub token: String,
}

impl Teacher {
    fn from(teacher: Teacher) -> Teacher {
        Teacher {
            email: teacher.email,
            password: teacher.password,
            first_name: teacher.first_name,
            last_name: teacher.last_name,
            designation: teacher.designation,
            department: teacher.department,
            salary: teacher.salary,
            age: teacher.age,
        }
    }
}

impl Teachers {
    pub fn find_all() -> Result<Vec<Self>, CustomError> {
        let conn = db::connection()?;
        let teachers = teachers::table.load::<Teachers>(&conn)?;
        Ok(teachers)
    }

    pub fn find(id: i32) -> Result<Self, CustomError> {
        let conn = db::connection()?;
        let teacher = teachers::table.filter(teachers::id.eq(id)).first(&conn)?;
        Ok(teacher)
    }

    pub fn find_by_department(department: String) -> Result<Vec<Self>, CustomError> {
        let conn = db::connection()?;
        let teacher = teachers::table
            .filter(teachers::department.eq(department))
            .load::<Teachers>(&conn)?;
        Ok(teacher)
    }

    fn find_by_email(email: String) -> Result<Option<Self>, CustomError> {
        let conn = db::connection()?;
        let teacher = teachers::table
            .filter(teachers::email.eq(email))
            .first(&conn)?;
        Ok(Some(teacher))
    }

    pub fn login(user: Login) -> Result<LoginResponse, CustomError> {
        match Teachers::find_by_email(user.email.to_string()) {
            Ok(option) => {
                match option {
                    Some(x) => {
                        let mut sha = Sha256::new();
                        sha.input_str(user.password.as_str());
                        if x.password == sha.result_str() {
                            // JWT
                            let key = std::env::var("SECRET_KEY").unwrap();
                            let key = key.as_bytes();

                            let mut _date: DateTime<Utc>;
                            // Remember me
                            if !user.remember_me {
                                _date = Utc::now() + Duration::hours(1);
                            } else {
                                _date = Utc::now() + Duration::days(365);
                            }

                            let my_claims = Claims {
                                sub: user.email,
                                exp: _date.timestamp() as usize,
                            };

                            let token = encode(
                                &Header::default(),
                                &my_claims,
                                &EncodingKey::from_secret(key),
                            )
                            .unwrap();
                            Ok(LoginResponse {
                                status: true,
                                token,
                                message: "You have successfully logged in.".to_string(),
                            })
                        } else {
                            Err(CustomError {
                                error_status_code: 404,
                                error_message: "Wrong Credentials".to_string(),
                            })
                        }
                    }
                    None => Err(CustomError {
                        error_status_code: 404,
                        error_message: "Wrong Credentials".to_string(),
                    }),
                }
            }
            Err(err) => Err(err),
        }
    }

    pub fn user_informations(token: &str) -> Result<Option<Self>, CustomError> {
        let key = std::env::var("SECRET_KEY").unwrap();
        let key = key.as_bytes();
        let _decode = decode::<Claims>(
            token,
            &DecodingKey::from_secret(key),
            &Validation::new(Algorithm::HS256),
        );

        match _decode {
            Ok(decoded) => {
                match Teachers::find_by_email((decoded.claims.sub.to_string()).parse().unwrap()) {
                    Ok(user) => Ok(user),
                    Err(_) => Err(CustomError {
                        error_status_code: 401,
                        error_message: "Something Wrong".to_string(),
                    }),
                }
            }
            Err(_) => Err(CustomError {
                error_status_code: 401,
                error_message: "Invalid Token".to_string(),
            }),
        }
    }

    pub fn create(teacher: Teacher) -> Result<Self, CustomError> {
        let conn = db::connection()?;
        let mut teacher = Teacher::from(teacher);
        let mut sha = Sha256::new();
        sha.input_str(teacher.password.as_str());
        let hash_pw = sha.result_str();
        teacher.password = hash_pw;
        let teacher = diesel::insert_into(teachers::table)
            .values(teacher)
            .get_result(&conn)?;
        Ok(teacher)
    }

    pub fn update(id: i32, teacher: Teacher) -> Result<Self, CustomError> {
        let conn = db::connection()?;
        let teacher = diesel::update(teachers::table)
            .filter(teachers::id.eq(id))
            .set(teacher)
            .get_result(&conn)?;
        Ok(teacher)
    }

    pub fn delete(id: i32) -> Result<usize, CustomError> {
        let conn = db::connection()?;
        let res = diesel::delete(teachers::table)
            .filter(teachers::id.eq(id))
            .execute(&conn)?;
        Ok(res)
    }
}
