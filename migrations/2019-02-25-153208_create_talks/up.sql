CREATE TABLE talks (
  id SERIAL PRIMARY KEY,
  user_id SERIAL references users(id),
  title TEXT NOT NULL,
  description TEXT NOT NULL,
  published BOOLEAN NOT NULL DEFAULT 'f',
  video_link TEXT,
  slides_link TEXT,
  created_at TIMESTAMP DEFAULT NOW(),
  updated_at TIMESTAMP DEFAULT NOW()
)
