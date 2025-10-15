-- PostgreSQL migration: create kit_part_paints join table using BIGINT FKs and composite primary key

CREATE TABLE IF NOT EXISTS kit_part_paints (
    kit_part_id BIGINT NOT NULL,
    paint_id BIGINT NOT NULL,

    CONSTRAINT kit_part_paints_pkey
        PRIMARY KEY (kit_part_id, paint_id),

    CONSTRAINT kit_part_paints_kit_part_id_fkey
        FOREIGN KEY (kit_part_id) REFERENCES kit_parts(id) ON DELETE CASCADE,

    CONSTRAINT kit_part_paints_paint_id_fkey
        FOREIGN KEY (paint_id) REFERENCES paints(id) ON DELETE CASCADE
);
