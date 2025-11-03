-- Add migration script here
-- Make steam_db_url nullable
ALTER TABLE steam_app_games
ALTER COLUMN steam_db_url DROP NOT NULL;
