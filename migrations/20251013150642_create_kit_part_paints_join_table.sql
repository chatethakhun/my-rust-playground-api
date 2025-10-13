-- Add migration script here
-- migrations/{timestamp}_create_kit_part_paints_join_table.up.sql

CREATE TABLE kit_part_paints (
    kit_part_id INTEGER NOT NULL,
    paint_id INTEGER NOT NULL,

    PRIMARY KEY (kit_part_id, paint_id),
    FOREIGN KEY (kit_part_id) REFERENCES kit_parts(id) ON DELETE CASCADE,
    FOREIGN KEY (paint_id) REFERENCES paints(id) ON DELETE CASCADE -- สมมติว่ามีตาราง 'paints'
);
