-- This file should undo anything in `up.sql`
ALTER TABLE custom_rooms ALTER current_map TYPE TEXT;
DROP TYPE enum_maps;
CREATE TYPE enum_maps AS ENUM ('heaven');
ALTER TABLE custom_rooms ALTER current_map TYPE enum_maps USING current_map::enum_maps;