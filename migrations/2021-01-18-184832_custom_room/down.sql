-- This file should undo anything in `up.sql`
DROP TABLE custom_room_slots;
DROP TABLE custom_rooms;

DROP TYPE enum_archetypes;
DROP TYPE enum_game_modes;
DROP TYPE enum_maps;

DROP DOMAIN uint2;