-- Add migration script here
-- Add migration script here
-- Up
CREATE TABLE colors (
    id INTEGER PRIMARY KEY,
    name TEXT NOT NULL,
    code TEXT NOT NULL,
    hex TEXT NOT NULL,
    is_clear BOOLEAN NOT NULL DEFAULT FALSE,
    is_multi BOOLEAN NOT NULL DEFAULT FALSE,

    -- Foreign Key to users table
    user_id INTEGER NOT NULL,

    -- Timestamps
    created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,

    -- Constraint: ห้ามมีสีซ้ำสำหรับ User คนเดียวกัน
    UNIQUE(user_id, name, code),

    -- กำหนด Foreign Key Constraint
    FOREIGN KEY (user_id) REFERENCES users(id)
        ON DELETE CASCADE
);
