-- PostgreSQL migration: create steam_app_games table with FKs and indexes

CREATE TABLE IF NOT EXISTS steam_app_games (
    id BIGSERIAL PRIMARY KEY,
    app_id BIGINT NOT NULL,
    steam_db_url TEXT NOT NULL,
    is_buy BOOLEAN NOT NULL DEFAULT false,
    user_id BIGINT NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),

    CONSTRAINT steam_app_games_user_id_fkey
        FOREIGN KEY (user_id) REFERENCES users(id)
        ON DELETE CASCADE,

    -- Prevent duplicate app records per user
    CONSTRAINT steam_app_games_user_app_unique UNIQUE (user_id, app_id)
);

-- Useful indexes for common lookups
CREATE INDEX IF NOT EXISTS idx_steam_app_games_user_id ON steam_app_games(user_id);
CREATE INDEX IF NOT EXISTS idx_steam_app_games_app_id ON steam_app_games(app_id);
