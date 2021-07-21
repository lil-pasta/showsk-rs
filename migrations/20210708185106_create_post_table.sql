-- Add migration script here
CREATE TABLE post(
	post_id uuid NOT NULL,
	PRIMARY KEY (post_id),
	body VARCHAR(10000) NOT NULL,
	img_path VARCHAR(100),
	user_id uuid NOT NULL REFERENCES users(user_id)
);
	
