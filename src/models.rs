pub mod user;
pub mod custom_room;
pub mod forms;

pub type ORMResult<R> = Result<R, diesel::result::Error>;