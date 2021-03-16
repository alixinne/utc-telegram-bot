CREATE TABLE users(
	id BIGINT PRIMARY KEY,
	first_name TEXT,
	last_name TEXT,
	username TEXT,
	language_code TEXT,
	request_count INT NOT NULL,
	first_seen DATETIME NOT NULL,
	last_seen DATETIME NOT NULL
);
