-- Your SQL goes here
ALTER TABLE users 
    DROP CONSTRAINT users_nickname_key,
    ADD steam_id TEXT NOT NULL UNIQUE,
    ADD first_name VARCHAR(100) NOT NULL,
    ADD last_name VARCHAR(100) NOT NULL,
    ADD birth_date TIMESTAMP NOT NULL, 
    ALTER COLUMN created_at SET DEFAULT CURRENT_TIMESTAMP;