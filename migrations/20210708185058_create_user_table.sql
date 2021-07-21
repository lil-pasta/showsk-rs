-- Add migration script here
CREATE TABLE post(
	id uuid NOT NULL,
	PRIMARY KEY (id),
	email TEXT NOT NULL UNIQUE,
	username TEXT NOT NULL UNIQUE,
	password_hash TEXT NOT NULL UNIQUE,
	about_me TEXT,
	role ENUM(admin, guest) DEFAULT 'guest',
)


