-- 20251015120000_alter_kit_part_requirements_gate_to_jsonb.sql
-- Purpose:
--   Convert kit_part_requirements.gate from TEXT to JSONB (array of strings) safely.
-- Notes:
--   - This migration intentionally does NOT add any CHECK constraints or triggers.
--     Validation will be handled in a subsequent migration.
--   - Existing TEXT values will be converted as follows:
--       * NULL -> '[]' (empty array)
--       * String that looks like a JSON array (e.g. '["A","B"]') -> parsed as JSONB array
--       * Any other string (e.g. 'A') -> wrapped as a single-element array '["A"]'
--   - The script is idempotent and will skip type change if the column is already JSONB.

BEGIN;

DO $$
BEGIN
  -- Only convert if the column exists and is not JSONB yet
  IF EXISTS (
    SELECT 1
    FROM information_schema.columns
    WHERE table_schema = 'public'
      AND table_name = 'kit_part_requirements'
      AND column_name = 'gate'
      AND data_type <> 'jsonb'
  ) THEN
    ALTER TABLE public.kit_part_requirements
      ALTER COLUMN gate TYPE jsonb
      USING (
        CASE
          WHEN gate IS NULL THEN '[]'::jsonb
          -- If it looks like a JSON array, cast directly
          WHEN trim(both FROM gate) ~ '^\[' THEN gate::jsonb
          -- Otherwise, wrap the text as a single string element
          ELSE jsonb_build_array(gate)
        END
      );
  END IF;
END
$$;

COMMIT;
