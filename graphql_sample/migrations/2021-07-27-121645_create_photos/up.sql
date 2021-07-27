-- Your SQL goes here
CREATE TABLE photos (
  id SERIAL NOT NULL PRIMARY KEY,
  name VARCHAR(255) NOT NULL,
  description TEXT,
  created_at timestamp with time zone NOT NULL default CURRENT_TIMESTAMP,
  updated_at timestamp with time zone NOT NULL default CURRENT_TIMESTAMP
);
