-- Alter the `process_executions` table to remove the NOT NULL constraint and default value
ALTER TABLE process_executions
ALTER COLUMN resource_reference_number DROP NOT NULL,
ALTER COLUMN resource_reference_number DROP DEFAULT,
ALTER COLUMN resource_quantity DROP NOT NULL,
ALTER COLUMN resource_quantity DROP DEFAULT,
ALTER COLUMN to_resource_reference_number DROP NOT NULL,
ALTER COLUMN to_resource_reference_number DROP DEFAULT,
ALTER COLUMN to_resource_lot_number DROP NOT NULL,
ALTER COLUMN to_resource_lot_number DROP DEFAULT,
ALTER COLUMN resource_lot_number DROP NOT NULL,
ALTER COLUMN resource_lot_number DROP DEFAULT;