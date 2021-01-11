-- Your SQL goes here
CREATE TABLE users (
  email VARCHAR(100) NOT NULL UNIQUE PRIMARY KEY,
  nickname VARCHAR(100) NOT NULL UNIQUE,
  hash VARCHAR(122) NOT NULL, --argon hash
  reset_password_hash VARCHAR(122) NULL UNIQUE,
  created_at TIMESTAMP NOT NULL
);