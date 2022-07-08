use crate::chrono::NaiveDateTime;
use crate::schema::users;
use crate::handlers::user::CreateUserData;

#[derive(Insertable, AsChangeset)]
#[table_name = "users"]
pub struct UserForm<'a> {
    pub email: &'a str,
    pub nickname: &'a str,
    pub steam_id: &'a str,
    pub first_name: &'a str,
    pub last_name: &'a str,
    pub hash: &'a str,
    pub birth_date: NaiveDateTime,
}

impl<'a> UserForm<'a> {
    pub fn new_from_data(create_data: &'a CreateUserData, steam_id: &'a str) -> Self {
        UserForm {
            email: &create_data.email,
            nickname: &create_data.nickname,
            steam_id: steam_id,
            first_name: &create_data.first_name,
            last_name: &create_data.last_name,
            hash: "Waiting for init",
            birth_date: create_data.birth_date.naive_utc(),
        }
    }
}