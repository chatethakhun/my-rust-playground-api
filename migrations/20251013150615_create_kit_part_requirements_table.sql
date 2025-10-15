-- PostgreSQL migration: create kit_part_requirements table using BIGSERIAL PK, BOOLEAN flags,
-- proper FKs to runners/kit_parts/users, and helpful indexes

CREATE TABLE IF NOT EXISTS kit_part_requirements (
    id BIGSERIAL PRIMARY KEY,
    gate TEXT NOT NULL,
    qty INTEGER NOT NULL DEFAULT 1,
    is_cut BOOLEAN NOT NULL DEFAULT false,
    runner_id BIGINT NOT NULL,
    kit_part_id BIGINT NOT NULL,
    user_id BIGINT NOT NULL,

    CONSTRAINT kit_part_requirements_runner_id_fkey
        FOREIGN KEY (runner_id) REFERENCES runners(id) ON DELETE CASCADE,
    CONSTRAINT kit_part_requirements_kit_part_id_fkey
        FOREIGN KEY (kit_part_id) REFERENCES kit_parts(id) ON DELETE CASCADE,
    CONSTRAINT kit_part_requirements_user_id_fkey
        FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE
);

-- Indexes to speed up lookups and joins
CREATE INDEX IF NOT EXISTS idx_kit_part_requirements_runner_id ON kit_part_requirements(runner_id);
CREATE INDEX IF NOT EXISTS idx_kit_part_requirements_kit_part_id ON kit_part_requirements(kit_part_id);
CREATE INDEX IF NOT EXISTS idx_kit_part_requirements_user_id ON kit_part_requirements(user_id);
