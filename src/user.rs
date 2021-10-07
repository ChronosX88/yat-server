use argon2::{
    password_hash::{
        rand_core::OsRng,
        PasswordHasher, SaltString
    },
    Argon2
};
use diesel::{RunQueryDsl, SqliteConnection};
use crate::schema::users;
use crate::schema::users::dsl::users as users_dsl;
use serde::{Serialize, Deserialize};
use diesel::prelude::*;
use diesel::result::{DatabaseErrorKind, Error as DieselError};
use crate::db::last_insert_rowid;
use custom_error::custom_error;

#[derive(Debug, Deserialize, Serialize, Insertable, Queryable)]
#[table_name = "users"]
pub struct User {
    pub id: Option<i32>,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: Option<chrono::NaiveDateTime>,
    pub username: String,
    pub email: String,
    pub password: String
}


custom_error!{ #[derive(Serialize)] pub UserCreationError
    DuplicatedEmail      = "duplicated email",
    DuplicatedUsername = "duplicated username",
    EmptyData = "empty data",
    UnknownError{text: String} = "{text}"
}

impl From<DieselError> for UserCreationError {
    fn from(err: DieselError) -> UserCreationError {
        if let DieselError::DatabaseError(DatabaseErrorKind::UniqueViolation, info) = &err {
            return match info.message() {
                "UNIQUE constraint failed: users.username" => UserCreationError::DuplicatedUsername,
                "UNIQUE constraint failed: users.email" => UserCreationError::DuplicatedEmail,
                _ => UserCreationError::UnknownError { text: err.to_string() }
            }
        }
        return UserCreationError::UnknownError{ text: err.to_string() }
    }
}

impl User {
    pub fn by_id(id: i32, conn: &SqliteConnection) -> Option<Self> {
        users_dsl.find(id).get_result::<User>(conn).ok()
    }

    pub fn create(email: String, username: String, password: String, conn: &SqliteConnection) -> Result<Self, UserCreationError> {
        if email.is_empty() || username.is_empty() || password.is_empty() {
            return Result::Err(UserCreationError::EmptyData.into())
        }

        match Self::insert_user(email, username, password, conn) {
            Err(e) => return Err(e.into()),
            _ => {}
        }

        let r = diesel::select(last_insert_rowid).first(conn).unwrap();
        Result::Ok(Self::by_id(r, conn).unwrap())
    }

    fn insert_user(email: String, username: String, password: String, conn: &SqliteConnection) -> Result<usize, UserCreationError> {
        let salt = SaltString::generate(&mut OsRng);
        let argon2 = Argon2::default();
        let password_hash = argon2.hash_password_simple(password.as_bytes(), &salt).unwrap();

        let new_user = Self::new_user_struct(email, username, password_hash.to_string());

        diesel::insert_into(users_dsl)
            .values(&new_user)
            .execute(conn)
            .map_err(Into::into)
    }

    fn new_user_struct(email: String, username: String, password: String) -> Self {
        User {
            id: None,
            email,
            username,
            password,
            created_at: chrono::Local::now().naive_local(),
            updated_at: Option::from(chrono::Local::now().naive_local())
        }
    }
}