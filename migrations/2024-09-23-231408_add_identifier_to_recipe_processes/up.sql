ALTER TABLE recipe_processes
ALTER COLUMN identifier SET NOT NULL;

ALTER TABLE recipe_processes
ADD CONSTRAINT unique_identifier UNIQUE (identifier);
