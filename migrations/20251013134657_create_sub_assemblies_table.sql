-- Add migration script here
-- migrations/{timestamp}_create_sub_assemblies.up.sql

CREATE TABLE sub_assemblies (
    id INTEGER PRIMARY KEY NOT NULL,
    name TEXT NOT NULL,
    kit_id INTEGER NOT NULL,
    user_id INTEGER NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,

    FOREIGN KEY (kit_id) REFERENCES kits(id) ON DELETE CASCADE,
    FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE
);

CREATE INDEX idx_sub_assemblies_kit_id ON sub_assemblies(kit_id);
