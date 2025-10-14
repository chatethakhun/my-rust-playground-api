-- Add migration script here
-- migrations/{timestamp}_create_kit_part_requirements_table.up.sql

CREATE TABLE kit_part_requirements (
    id INTEGER PRIMARY KEY NOT NULL,
    gate TEXT NOT NULL,
    qty INTEGER NOT NULL DEFAULT 1,
    is_cut BOOLEAN NOT NULL DEFAULT FALSE,
    runner_id INTEGER NOT NULL,
    kit_part_id INTEGER NOT NULL,
    user_id INTEGER NOT NULL,

    FOREIGN KEY (runner_id) REFERENCES runners(id) ON DELETE CASCADE,
    FOREIGN KEY (kit_part_id) REFERENCES kit_parts(id) ON DELETE CASCADE,
    FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE
);

CREATE INDEX idx_kit_part_requirements_kit_part_id ON kit_part_requirements(kit_part_id);
