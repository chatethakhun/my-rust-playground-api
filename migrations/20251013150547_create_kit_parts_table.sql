-- PostgreSQL migration: create kit_parts table using BIGSERIAL PK, BOOLEAN flags,
-- TIMESTAMPTZ timestamps, proper FKs, and a composite UNIQUE constraint

CREATE TABLE IF NOT EXISTS kit_parts (
    id BIGSERIAL PRIMARY KEY,
    code TEXT,
    is_cut BOOLEAN NOT NULL DEFAULT false,
    kit_id BIGINT NOT NULL,
    sub_assembly_id BIGINT NOT NULL,
    user_id BIGINT NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),

    CONSTRAINT kit_parts_kit_id_fkey
        FOREIGN KEY (kit_id) REFERENCES kits(id) ON DELETE CASCADE,
    CONSTRAINT kit_parts_sub_assembly_id_fkey
        FOREIGN KEY (sub_assembly_id) REFERENCES sub_assemblies(id) ON DELETE CASCADE,
    CONSTRAINT kit_parts_user_id_fkey
        FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE,

    -- Keep the original uniqueness rule across these columns
    CONSTRAINT kit_parts_user_kit_subassembly_code_key
        UNIQUE (user_id, kit_id, sub_assembly_id, code)
);

-- Helpful indexes
CREATE INDEX IF NOT EXISTS idx_kit_parts_kit_id ON kit_parts(kit_id);
CREATE INDEX IF NOT EXISTS idx_kit_parts_sub_assembly_id ON kit_parts(sub_assembly_id);
CREATE INDEX IF NOT EXISTS idx_kit_parts_user_id ON kit_parts(user_id);
