-- Add new columns to the recipe_processes table
ALTER TABLE recipe_processes
ADD COLUMN commitment action_type_enum,
ADD COLUMN fulfills UUID REFERENCES recipe_processes(id);
