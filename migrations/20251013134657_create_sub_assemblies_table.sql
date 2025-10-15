-- PostgreSQL migration: create sub_assemblies table using BIGSERIAL PK, TIMESTAMPTZ timestamps, and FKs to kits and users

CREATE TABLE IF NOT EXISTS sub_assemblies (
    id BIGSERIAL PRIMARY KEY,
    name TEXT NOT NULL,
    kit_id BIGINT NOT NULL,
    user_id BIGINT NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),

    CONSTRAINT sub_assemblies_kit_id_fkey
        FOREIGN KEY (kit_id) REFERENCES kits(id) ON DELETE CASCADE,
    CONSTRAINT sub_assemblies_user_id_fkey
        FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE
);

-- Indexes for faster lookups
CREATE INDEX IF NOT EXISTS idx_sub_assemblies_kit_id ON sub_assemblies(kit_id);
CREATE INDEX IF NOT EXISTS idx_sub_assemblies_user_id ON sub_assemblies(user_id);
