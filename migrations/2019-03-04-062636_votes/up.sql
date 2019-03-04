CREATE TABLE votes (
  id SERIAL PRIMARY KEY,
  talk_id SERIAL references talks(id),
  user_id SERIAL references users(id),
  created_at TIMESTAMP DEFAULT NOW()
)
