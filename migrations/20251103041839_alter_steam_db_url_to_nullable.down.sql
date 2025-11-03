-- Revert: make steam_db_url required again
ALTER TABLE steam_app_games
ALTER COLUMN steam_db_url SET NOT NULL;
