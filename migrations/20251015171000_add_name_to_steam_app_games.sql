-- Migration: Add `name` column to steam_app_games with safe default, then drop default

BEGIN;

-- 1) Add the column as nullable first
ALTER TABLE steam_app_games
ADD COLUMN name TEXT;

-- 2) Set a safe default for new inserts during migration
ALTER TABLE steam_app_games
ALTER COLUMN name SET DEFAULT '';

-- 3) Backfill existing rows
UPDATE steam_app_games
SET name = ''
WHERE name IS NULL;

-- 4) Enforce NOT NULL after backfill
ALTER TABLE steam_app_games
ALTER COLUMN name SET NOT NULL;

-- 5) Drop the default to avoid unintended future defaults
ALTER TABLE steam_app_games
ALTER COLUMN name DROP DEFAULT;

COMMIT;
