use actix_web::{HttpResponse, Responder};
use actix_web::web::{Data, Json, ServiceConfig};
use crate::db::DbPool;
use crate::user::{User, UserCreationError};
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct NewUser {
    email: String,
    username: String,
    password: String
}

#[derive(Serialize)]
pub struct HttpError {
    err: String
}

#[derive(Serialize)]
pub struct HttpResult {
    text: String
}

#[post("/register")]
async fn register(user_form: Json<NewUser>, pool: Data<DbPool>) -> impl Responder {
    let conn = pool.get().unwrap();

    match User::create(user_form.email.clone(), user_form.username.clone(), user_form.password.clone(), &conn) {
        Ok(_) => HttpResponse::Ok().json(HttpResult{text:"User has been created successfully!".to_string()}),
        Err(e) => match e {
            UserCreationError::DuplicatedUsername | UserCreationError::DuplicatedEmail => {
                HttpResponse::BadRequest().json(HttpError{err: e.to_string()})
            }
            _ => {HttpResponse::InternalServerError().json(HttpError{err: e.to_string()})}
        }
    }
}

pub fn init_routes(cfg: &mut ServiceConfig) {
    cfg.service(register);
}