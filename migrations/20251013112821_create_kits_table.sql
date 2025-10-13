-- Add migration script here
-- Up
CREATE TABLE kits (
id INTEGER PRIMARY KEY,
  name VARCHAR(255) NOT NULL,
  grade TEXT NOT NULL,
  status TEXT NOT NULL DEFAULT 'pending',

  -- Foreign Key to users table
  user_id INTEGER NOT NULL,

  created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
  updated_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,

  -- Constraint: ห้ามมีสีซ้ำสำหรับ User คนเดียวกัน
  UNIQUE(user_id),

  -- กำหนด Foreign Key Constraint
  FOREIGN KEY (user_id) REFERENCES users(id)
      ON DELETE CASCADE
);
