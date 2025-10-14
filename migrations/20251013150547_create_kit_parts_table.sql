-- Add migration script here
-- migrations/{timestamp}_create_kit_parts_table.up.sql

CREATE TABLE kit_parts (
    id INTEGER PRIMARY KEY NOT NULL,
    code TEXT,
    is_cut BOOLEAN NOT NULL DEFAULT FALSE,
    kit_id INTEGER NOT NULL,
    sub_assembly_id INTEGER NOT NULL,
    user_id INTEGER NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,

    FOREIGN KEY (kit_id) REFERENCES kits(id) ON DELETE CASCADE,
    FOREIGN KEY (sub_assembly_id) REFERENCES sub_assemblies(id) ON DELETE CASCADE,
    FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE,

    -- สร้าง unique constraint ตาม schema เดิม
    UNIQUE(user_id, kit_id, sub_assembly_id, code)
);

CREATE INDEX idx_kit_parts_kit_id ON kit_parts(kit_id);
CREATE INDEX idx_kit_parts_sub_assembly_id ON kit_parts(sub_assembly_id);
