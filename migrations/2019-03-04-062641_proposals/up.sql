CREATE TABLE proposals (
  id SERIAL PRIMARY KEY,
  user_id SERIAL references users(id),
  name TEXT NOT NULL,
  description TEXT NOT NULL,
  created_at TIMESTAMP NOT NULL DEFAULT NOW()
)
