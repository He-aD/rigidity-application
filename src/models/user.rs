use crate::schema::users::dsl::*;
use crate::diesel::prelude::*;
use crate::chrono::{Duration, Utc, NaiveDateTime};
use diesel::{PgConnection};
use serde::{Deserialize, Serialize};
use crate::errors::*;
use crate::app_conf::SECRET_KEY;

#[derive(Serialize, Deserialize, Queryable)]
pub struct User {
    pub email: String,
    pub nickname: String,
    #[serde(skip_serializing)]
    pub hash: String,
    #[serde(skip_serializing)]
    pub reset_password_hash: Option<String>,
    #[serde(skip_serializing)]
    pub password_hash_expire_at: Option<NaiveDateTime>,
    #[serde(skip_serializing)]
    pub created_at: NaiveDateTime,
}

impl User {
    pub fn is_password_ok(&self, password: &str) -> AppResult<bool> {
        argon2::verify_encoded_ext(
            &self.hash, 
            password.as_bytes(), 
            SECRET_KEY.as_bytes(), 
            &[])
            .map_err(|err| {
                dbg!(err);
                AppError::Unauthorized
            })
    }
}

pub fn get(em8l: String, conn: &PgConnection) -> Result<User, diesel::result::Error> {
    users.filter(email.eq(em8l))
        .get_result::<User>(conn)
}

pub fn update_reset_password_hash(em8l: &str, h: &str, conn: &PgConnection) -> Result<i64, diesel::result::Error> {
    let time_in_4_hours = (Utc::now() + Duration::hours(4)).timestamp();

   diesel::update(users.filter(email.eq(em8l)))
        .set((
            reset_password_hash.eq(h),
            password_hash_expire_at.eq(NaiveDateTime::from_timestamp(time_in_4_hours, 0))
        )).execute(conn)?;

    Ok(time_in_4_hours)
}