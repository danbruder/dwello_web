-- Your SQL goes here
CREATE TABLE profiles (
  id SERIAL PRIMARY KEY,
  uid INTEGER NOT NULL REFERENCES users(id) ON DELETE CASCADE UNIQUE,
  title VARCHAR NOT NULL,
  intro TEXT NOT NULL,
  body TEXT NOT NULL
);