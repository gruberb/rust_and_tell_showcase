CREATE TABLE meetups (
  id SERIAL PRIMARY KEY,
  title TEXT NOT NULL,
  talks TEXT [],
  date TIMESTAMP NOT NULL 
)
