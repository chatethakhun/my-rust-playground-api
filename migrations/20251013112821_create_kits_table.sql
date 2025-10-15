-- PostgreSQL migration: create kits table using BIGSERIAL PK, TIMESTAMPTZ timestamps, and FK to users

CREATE TABLE IF NOT EXISTS kits (
    id BIGSERIAL PRIMARY KEY,
    name VARCHAR(255) NOT NULL,
    grade TEXT NOT NULL,
    status TEXT NOT NULL DEFAULT 'pending',
    user_id BIGINT NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    CONSTRAINT kits_user_id_fkey
        FOREIGN KEY (user_id) REFERENCES users(id)
        ON DELETE CASCADE
);

CREATE INDEX IF NOT EXISTS idx_kits_user_id ON kits(user_id);
CREATE INDEX IF NOT EXISTS idx_kits_status ON kits(status);
