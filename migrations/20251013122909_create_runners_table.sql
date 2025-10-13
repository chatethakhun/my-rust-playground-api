-- Add migration script here
-- migrations/{timestamp}_create_runners_table.up.sql

CREATE TABLE runners (
    id INTEGER PRIMARY KEY NOT NULL,
    name TEXT NOT NULL,
    kit_id INTEGER NOT NULL,
    color_id INTEGER NOT NULL,
    amount INTEGER NOT NULL,
    user_id INTEGER NOT NULL,
    is_used BOOLEAN NOT NULL DEFAULT FALSE,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,

    -- 🛡️ Foreign Keys for data integrity
    FOREIGN KEY (kit_id) REFERENCES kits(id) ON DELETE CASCADE,
    FOREIGN KEY (color_id) REFERENCES colors(id) ON DELETE CASCADE,
    FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE
);

-- ✨ Indexes for faster queries
CREATE INDEX idx_runners_kit_id ON runners(kit_id);
CREATE INDEX idx_runners_user_id ON runners(user_id);
