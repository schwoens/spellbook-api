-- Your SQL goes here
CREATE TABLE spells (
	id SERIAL PRIMARY KEY,
	name VARCHAR NOT NULL,
	level VARCHAR NOT NULL,
	time VARCHAR NOT NULL,
	school VARCHAR NOT NULL,
	concentration BOOL NOT NULL,
	range VARCHAR NOT NULL,
	duration VARCHAR NOT NULL,
	UNIQUE(name)
);
