-- PostgreSQL migration: create colors table using BIGSERIAL PK, BOOLEAN flags, TIMESTAMPTZ timestamps, and FK to users

CREATE TABLE colors (
    id BIGSERIAL PRIMARY KEY,
    name TEXT NOT NULL,
    code TEXT NOT NULL,
    hex TEXT NOT NULL,
    is_clear BOOLEAN NOT NULL DEFAULT false,
    is_multi BOOLEAN NOT NULL DEFAULT false,

    -- Foreign Key to users table
    user_id BIGINT NOT NULL,

    -- Timestamps
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),

    -- Constraint: prevent duplicate color per user
    UNIQUE(user_id, name, code),

    -- Foreign Key Constraint
    CONSTRAINT colors_user_id_fkey
        FOREIGN KEY (user_id) REFERENCES users(id)
        ON DELETE CASCADE
);

-- Optional: index to speed up lookups by user
CREATE INDEX IF NOT EXISTS idx_colors_user_id ON colors(user_id);
