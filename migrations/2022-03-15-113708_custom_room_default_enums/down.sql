-- This file should undo anything in `up.sql`
ALTER TABLE custom_rooms ALTER current_map drop default;
ALTER TABLE custom_rooms ALTER current_game_mode drop default;