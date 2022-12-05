use crate::{handlers::auth::ResetPassData, schema::users};
use crate::schema::users::dsl::*;
use crate::diesel::prelude::*;
use crate::chrono::{Duration, Utc, NaiveDateTime};
use diesel::{PgConnection};
use serde::{Deserialize, Serialize};
use crate::errors::*;
use crate::app_conf::SECRET_KEY;
use crate::models::{forms::user::UserForm, ORMResult};
use diesel::result::Error;

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
    pub steam_id: String,
    pub first_name: String,
    pub last_name: String,
    pub birth_date: NaiveDateTime,
    #[serde(skip_serializing)]
    pub email_confirmation_required: bool,
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

    pub fn can_login(&self) -> bool {
        !self.email_confirmation_required
    }
}

pub fn get(
    i_d: &i32, 
    conn: &PgConnection
) -> ORMResult<User> {
    users.filter(id.eq(i_d))
        .get_result::<User>(conn)
}

pub fn get_by_email(
    em8l: &str, 
    conn: &PgConnection
) -> ORMResult<User> {
    users.filter(email.eq(em8l))
        .get_result::<User>(conn)
}

pub fn get_by_steam_id(
    i_d: &str, 
    conn: &PgConnection
) -> ORMResult<User> {
    users.filter(steam_id.eq(i_d))
        .get_result::<User>(conn)
}

pub fn get_by_reset_password_hash(
    h: &str, 
    conn: &PgConnection
) -> ORMResult<User> {
    users.filter(reset_password_hash.eq(h))
        .get_result::<User>(conn) 
}

pub fn create(
    data: UserForm,
    email_confirmation_hash: &str,
    conn: &PgConnection
) -> ORMResult<(User, i64)> {
    use crate::schema::users::dsl::users;
    let em8l = data.email;

    conn.transaction::<(User, i64), Error, _>(move || {
        diesel::insert_into(users)
            .values(data)
            .execute(conn)?;

        let expire_timestamp = update_reset_password_hash(em8l, email_confirmation_hash, conn)?;

        let user_id = users
            .select(id)
            .order(id.desc())
            .first(conn)?;

        let user = get(&user_id, conn)?;   

        Ok((user, expire_timestamp))
    })
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
            password_hash_expire_at.eq(NaiveDateTime::from_timestamp_opt(time_in_4_hours, 0))
        )).execute(conn)?;

    Ok(time_in_4_hours)
}

pub fn update_email(
    new_email: &str,
    ssteam_id: &u64,
    email_confirmation_hash: &str,
    conn: &PgConnection
) -> ORMResult<(User, i64)> {
    conn.transaction::<(User, i64), Error, _>(move || {
        diesel::update(users.filter(steam_id.eq(ssteam_id.to_string())))
            .set(email.eq(new_email)).execute(conn)?;

        let expire_timestamp = update_reset_password_hash(new_email, email_confirmation_hash, conn)?;

        let user = get_by_email(new_email, conn)?;   

        Ok((user, expire_timestamp))
    })
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

pub fn confirm_email(reset_pass_hash: &str, conn: &PgConnection) -> ORMResult<()> {
    let new_reset_password_hash: Option<String> = None; 
    let new_expire_at: Option<NaiveDateTime> = None;

    diesel::update(users.filter(reset_password_hash.eq(reset_pass_hash.clone())))
        .set((
            reset_password_hash.eq(new_reset_password_hash),
            password_hash_expire_at.eq(new_expire_at),
            email_confirmation_required.eq(false)
        )).execute(conn)?;

    Ok(())
}