-- Your SQL goes here
ALTER TABLE users 
    ALTER COLUMN hash TYPE varchar(159),
    ALTER COLUMN reset_password_hash TYPE varchar(159);