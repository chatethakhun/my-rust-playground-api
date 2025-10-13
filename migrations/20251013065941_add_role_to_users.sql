-- Add migration script here

-- Up
ALTER TABLE users ADD COLUMN role TEXT NOT NULL DEFAULT 'user';
