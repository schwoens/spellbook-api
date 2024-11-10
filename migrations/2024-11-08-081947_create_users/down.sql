-- This file should undo anything in `up.sql`
ALTER TABLE spells
DROP COLUMN user_id;

DROP TABLE IF EXISTS "users";

