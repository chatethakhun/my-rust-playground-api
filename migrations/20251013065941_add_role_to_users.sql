-- PostgreSQL-safe migration: add `role` column to `users` table
-- - Idempotent (safe to run multiple times)
-- - Ensures default value and NOT NULL constraint

DO $$
BEGIN
    -- Add the column if it does not already exist
    ALTER TABLE users ADD COLUMN role TEXT;
EXCEPTION
    WHEN duplicate_column THEN
        -- Column already exists; proceed
        NULL;
END $$;

-- Backfill existing rows to ensure no NULL values
UPDATE users
SET role = 'user'
WHERE role IS NULL;

-- Ensure a default for future inserts
ALTER TABLE users
    ALTER COLUMN role SET DEFAULT 'user';

-- Enforce NOT NULL constraint
ALTER TABLE users
    ALTER COLUMN role SET NOT NULL;
