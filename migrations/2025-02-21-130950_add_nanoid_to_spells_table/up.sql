-- Your SQL goes here
ALTER TABLE spells
ADD nanoid VARCHAR NOT NULL;

ALTER TABLE spells
ADD UNIQUE(nanoid);

