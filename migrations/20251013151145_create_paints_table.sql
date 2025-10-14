-- Add migration script here
-- migrations/{timestamp}_create_paints_table.up.sql

CREATE TABLE paints (
    id INTEGER PRIMARY KEY NOT NULL,
    name TEXT NOT NULL,
    brand TEXT,
    code TEXT,
    user_id INTEGER NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,

    FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE
);
