ALTER TABLE recipe_processes
DROP COLUMN IF EXISTS commitment,
DROP COLUMN IF EXISTS action_type_enum,
DROP COLUMN IF EXISTS fulfills,
DROP COLUMN IF EXISTS identifier,
DROP COLUMN IF EXISTS trigger;