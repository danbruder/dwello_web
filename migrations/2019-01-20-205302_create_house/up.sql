-- Your SQL goes here
CREATE TABLE houses (
  id SERIAL PRIMARY KEY,

  address VARCHAR(255) NOT NULL UNIQUE,

  created TIMESTAMP NOT NULL,
  updated TIMESTAMP NOT NULL
);
