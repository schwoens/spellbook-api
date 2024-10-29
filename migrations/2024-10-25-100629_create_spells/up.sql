-- Your SQL goes here
CREATE TABLE spells (
	id SERIAL PRIMARY KEY,
	name VARCHAR NOT NULL,
	level VARCHAR NOT NULL,
	casting_time VARCHAR NOT NULL,
	magic_school VARCHAR NOT NULL,
	concentration BOOL NOT NULL,
	range VARCHAR NOT NULL,
	duration VARCHAR NOT NULL,
	UNIQUE(name)
);
