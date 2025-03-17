-- Add migration script here

CREATE TABLE users (
  id UUID NOT NULL PRIMARY KEY,
  email CITEXT UNIQUE NOT NULL,
  sha256_hashed_password CHAR(64) NOT NULL
)
