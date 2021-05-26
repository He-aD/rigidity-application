-- Your SQL goes here

ALTER TABLE custom_rooms ALTER current_game_mode TYPE TEXT;
DROP TYPE enum_game_modes;
CREATE TYPE enum_game_modes AS ENUM ('deathmatch', 'king_of_the_hill');
ALTER TABLE custom_rooms ALTER current_game_mode TYPE enum_game_modes USING current_game_mode::enum_game_modes;