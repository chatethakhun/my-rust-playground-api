-- PostgreSQL migration: create runners table using BIGSERIAL PK, BOOLEAN flags, TIMESTAMPTZ timestamps, and proper FKs

CREATE TABLE IF NOT EXISTS runners (
    id BIGSERIAL PRIMARY KEY,
    name TEXT NOT NULL,
    kit_id BIGINT NOT NULL,
    color_id BIGINT NOT NULL,
    amount INTEGER NOT NULL,
    user_id BIGINT NOT NULL,
    is_used BOOLEAN NOT NULL DEFAULT false,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),

    CONSTRAINT runners_kit_id_fkey
        FOREIGN KEY (kit_id) REFERENCES kits(id)
        ON DELETE CASCADE,
    CONSTRAINT runners_color_id_fkey
        FOREIGN KEY (color_id) REFERENCES colors(id)
        ON DELETE CASCADE,
    CONSTRAINT runners_user_id_fkey
        FOREIGN KEY (user_id) REFERENCES users(id)
        ON DELETE CASCADE
);

-- Indexes for faster lookups
CREATE INDEX IF NOT EXISTS idx_runners_kit_id ON runners(kit_id);
CREATE INDEX IF NOT EXISTS idx_runners_user_id ON runners(user_id);
