-- Add the `extends` column to `packs` to record the base persona pack name
-- (from the pack manifest's `extends` field). NULL for packs with no base.
ALTER TABLE packs ADD COLUMN extends TEXT;
