pub mod user;
pub mod custom_room;

pub type ORMResult<R> = Result<R, diesel::result::Error>;