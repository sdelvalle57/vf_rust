-- Alter the `process_executions` table to remove the NOT NULL constraint and default value
ALTER TABLE process_executions
ALTER COLUMN at_location DROP NOT NULL;