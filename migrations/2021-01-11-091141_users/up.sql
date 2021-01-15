-- Your SQL goes here
CREATE TABLE users (
  id SERIAL PRIMARY KEY,
  email VARCHAR(100) NOT NULL UNIQUE,
  nickname VARCHAR(100) NOT NULL UNIQUE,
  hash VARCHAR(122) NOT NULL, --argon hash
  reset_password_hash VARCHAR(122) NULL UNIQUE,
  password_hash_expire_at TIMESTAMP NULL,
  created_at TIMESTAMP NOT NULL
);