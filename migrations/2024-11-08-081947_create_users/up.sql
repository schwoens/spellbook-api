-- Your SQL goes here
CREATE TABLE users (
  id SERIAL PRIMARY KEY,
  username VARCHAR NOT NULL,
  key_hash VARCHAR NOT NULL, 
  UNIQUE(username)
);

ALTER TABLE spells
ADD user_id INT NOT NULL REFERENCES users(id);
