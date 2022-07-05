-- This file should undo anything in `up.sql`
ALTER TABLE users 
    DROP COLUMN steam_id,
    DROP COLUMN first_name,
    DROP COLUMN last_name,
    DROP COLUMN birth_date;