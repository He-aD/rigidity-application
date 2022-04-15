-- This file should undo anything in `up.sql`
ALTER TABLE custom_rooms ALTER current_map drop default;
ALTER TABLE custom_rooms ALTER current_map TYPE TEXT;
DROP TYPE enum_maps;
CREATE TYPE enum_maps AS ENUM ('ascent', 'inferno', 'colosseum', 'heaven');
ALTER TABLE custom_rooms ALTER current_map TYPE enum_maps USING current_map::enum_maps;
ALTER TABLE custom_rooms ALTER current_map SET DEFAULT 'inferno';