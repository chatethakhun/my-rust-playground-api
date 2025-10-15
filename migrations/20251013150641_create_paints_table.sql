-- PostgreSQL migration: create paints table using BIGSERIAL PK, TIMESTAMPTZ timestamps, and FK to users

CREATE TABLE IF NOT EXISTS paints (
    id BIGSERIAL PRIMARY KEY,
    name TEXT NOT NULL,
    brand TEXT,
    code TEXT,
    user_id BIGINT NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    CONSTRAINT paints_user_id_fkey
        FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE
);

-- Helpful index to speed up lookups by user
CREATE INDEX IF NOT EXISTS idx_paints_user_id ON paints(user_id);
