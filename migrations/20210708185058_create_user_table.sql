-- Add migration script here
CREATE TYPE role_enum as ENUM('admin', 'guest');

CREATE TABLE users (
	user_id uuid NOT NULL,
	PRIMARY KEY (user_id),
	email TEXT NOT NULL UNIQUE,
	username TEXT NOT NULL UNIQUE,
	password_hash TEXT NOT NULL UNIQUE,
	about_me TEXT,
	role role_enum DEFAULT 'guest'
);


