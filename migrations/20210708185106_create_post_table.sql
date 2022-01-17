-- Add migration script here
CREATE TABLE post(
	post_id uuid NOT NULL,
	PRIMARY KEY (post_id),
	body VARCHAR(10000) NOT NULL,
	image VARCHAR(100),
	timestmp timestamptz NOT NULL,
	user_id uuid NOT NULL REFERENCES users(user_id)
);
	
