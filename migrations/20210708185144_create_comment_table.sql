-- Add migration script here
CREATE TABLE comment(
	comment_id uuid NOT NULL,
	PRIMARY KEY (comment_id),
	body VARCHAR(1000) NOT NULL,
	user_id uuid NOT NULL REFERENCES users(user_id),
	post_id uuid NOT NULL REFERENCES post(post_id)
);
