-- Add up migration script here
CREATE TABLE IF NOT EXISTS blog (
  title VARCHAR(255) NOT NULL,
  email VARCHAR(255) REFERENCES users(email) ON DELETE CASCADE NOT NULL,
  content TEXT NOT NULL,
  publish_date VARCHAR(255) NOT NULL
);