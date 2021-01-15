use crate::{handlers::auth::ResetPassData, schema::users};
use crate::schema::users::dsl::*;
use crate::diesel::prelude::*;
use crate::chrono::{Duration, Utc, NaiveDateTime};
use diesel::{PgConnection};
use serde::{Deserialize, Serialize};
use crate::errors::*;
use crate::app_conf::SECRET_KEY;
use crate::models::ORMResult;

#[derive(Serialize, Deserialize, Queryable, AsChangeset)]
#[changeset_options(treat_none_as_null="true")]
pub struct User {
    pub id: i32,
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

pub fn get(
    em8l: &str, 
    conn: &PgConnection
) -> ORMResult<User> {
    users.filter(email.eq(em8l))
        .get_result::<User>(conn)
}

pub fn get_by_reset_password_hash(
    h: &str, 
    conn: &PgConnection
) -> ORMResult<User> {
    users.filter(reset_password_hash.eq(h))
        .get_result::<User>(conn) 
}

pub fn update_reset_password_hash(
    em8l: &str, 
    h: &str, 
    conn: &PgConnection
) -> ORMResult<i64> {
    let time_in_4_hours = (Utc::now() + Duration::hours(4)).timestamp();

    diesel::update(users.filter(email.eq(em8l)))
        .set((
            reset_password_hash.eq(h),
            password_hash_expire_at.eq(NaiveDateTime::from_timestamp(time_in_4_hours, 0))
        )).execute(conn)?;

    Ok(time_in_4_hours)
}

pub fn update_password(
    data: &ResetPassData,
    new_hash: &str,
    conn: &PgConnection
) -> ORMResult<()> {
    let new_reset_password_hash: Option<String> = None; 
    let new_expire_at: Option<NaiveDateTime> = None;

    diesel::update(users.filter(reset_password_hash.eq(data.hash.clone())))
        .set((
            reset_password_hash.eq(new_reset_password_hash),
            password_hash_expire_at.eq(new_expire_at),
            hash.eq(new_hash)
        )).execute(conn)?;

    Ok(())
}

pub fn cancel_reset_password_hash(
    h: &str, 
    conn: &PgConnection
) -> ORMResult<()> {   
    let new_hash: Option<String> = None; 
    let new_expire_at: Option<NaiveDateTime> = None;
    diesel::update(users.filter(reset_password_hash.eq(h)))
        .set((
            reset_password_hash.eq(new_hash),
            password_hash_expire_at.eq(new_expire_at)
        )).execute(conn)?;

    Ok(())
} 