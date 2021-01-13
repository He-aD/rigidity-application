pub mod user;

pub type ORMResult<R> = Result<R, diesel::result::Error>;