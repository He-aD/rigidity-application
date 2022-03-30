-- This file should undo anything in `up.sql`
ALTER TABLE custom_room_slots ALTER current_archetype TYPE TEXT;
DROP TYPE enum_archetypes;
CREATE TYPE enum_archetypes AS ENUM ('leader', 'spiker', 'healer');
ALTER TABLE custom_room_slots ALTER current_archetype TYPE enum_archetypes USING current_archetype::enum_archetypes;