// use argon2::{self, Config};



// const SALT: &'static [u8] = b"supersecuresalt";

// WARNING THIS IS ONLY FOR DEMO PLEASE DO MORE RESEARCH FOR PRODUCTION USE
// pub fn hash_password(password: &str) -> Result<String, ServiceError> {
//     let config = Config {
//         secret: SECRET_KEY.as_bytes(),
//         ..Default::default()
//     };
//     argon2::hash_encoded(password.as_bytes(), &SALT, &config).map_err(|err| {
//         dbg!(err);
//         ServiceError::InternalServerError
//     })
// }