-- Add migration script here
-- Update existing status values to snake_case
UPDATE kits SET status = 'pending' WHERE status = 'Pending';
UPDATE kits SET status = 'in_progress' WHERE status = 'InProgress' OR status = 'in_progress';
UPDATE kits SET status = 'done' WHERE status = 'Done';

-- Optional: Add check constraint to ensure only valid values (PostgreSQL-safe)
-- Option A: Add a CHECK constraint without recreating the table
-- DO $$
-- BEGIN
--     ALTER TABLE kits
--         ADD CONSTRAINT kits_status_check
--         CHECK (status IN ('pending', 'in_progress', 'done'));
-- EXCEPTION
--     WHEN duplicate_object THEN NULL;
-- END $$;
--
-- Option B: Recreate table with proper PostgreSQL types and constraint
-- CREATE TABLE kits_new (
--     id BIGSERIAL PRIMARY KEY,
--     name TEXT NOT NULL,
--     grade TEXT NOT NULL,
--     status TEXT NOT NULL CHECK (status IN ('pending', 'in_progress', 'done')),
--     user_id BIGINT NOT NULL,
--     created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
--     updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
--     CONSTRAINT kits_user_id_fkey FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE
-- );
-- INSERT INTO kits_new (id, name, grade, status, user_id, created_at, updated_at)
--     SELECT id, name, grade, status, user_id, created_at, updated_at FROM kits;
-- DROP TABLE kits;
-- ALTER TABLE kits_new RENAME TO kits;
