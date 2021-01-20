-- Your SQL goes here
CREATE TYPE enum_archetypes AS ENUM ('leader', 'healer', 'spiker');
CREATE TYPE enum_game_modes AS ENUM ('deathmatch');
CREATE TYPE enum_maps AS ENUM ('heaven');

CREATE DOMAIN uint2 AS int4
   CHECK(VALUE >= 0 AND VALUE < 65536);

CREATE TABLE custom_rooms (
  id SERIAL PRIMARY KEY,
  label VARCHAR(100) NOT NULL,
  user_id INT UNIQUE NOT NULL,
  nb_teams uint2 NOT NULL,
  max_player_per_team uint2 NOT NULL,
  current_game_mode enum_game_modes NOT NULL,
  current_map enum_maps NOT NULL,

  CONSTRAINT fk_owner
    FOREIGN KEY(user_id) 
      REFERENCES users(id)
);

CREATE TABLE custom_room_slots (
  id SERIAL PRIMARY KEY,
  custom_room_id INT NOT NULL,
  team uint2 NOT NULL,
  team_position uint2 NOT NULL,
  user_id INT UNIQUE NOT NULL,
  current_archetype enum_archetypes NOT NULL,

  CONSTRAINT fk_user
    FOREIGN KEY(user_id) 
      REFERENCES users(id),
  CONSTRAINT fk_custom_room
    FOREIGN KEY(custom_room_id) 
      REFERENCES custom_rooms(id)
);