-- Add migration script here
-- Update existing status values to snake_case
UPDATE kits SET status = 'pending' WHERE status = 'Pending';
UPDATE kits SET status = 'in_progress' WHERE status = 'InProgress' OR status = 'in_progress';
UPDATE kits SET status = 'done' WHERE status = 'Done';

-- Optional: Add check constraint to ensure only valid values
-- (ถ้าต้องการบังคับให้ใส่ค่าถูกต้องเท่านั้น)
-- CREATE TABLE kits_new (
--     id INTEGER PRIMARY KEY,
--     name TEXT NOT NULL,
--     grade TEXT NOT NULL,
--     status TEXT NOT NULL CHECK(status IN ('pending', 'in_progress', 'done')),
--     user_id INTEGER NOT NULL,
--     created_at TEXT NOT NULL,
--     updated_at TEXT NOT NULL
-- );
--
-- INSERT INTO kits_new SELECT * FROM kits;
-- DROP TABLE kits;
-- ALTER TABLE kits_new RENAME TO kits;
