-- Remove the `extends` column added by the corresponding up migration.
ALTER TABLE packs DROP COLUMN extends;
