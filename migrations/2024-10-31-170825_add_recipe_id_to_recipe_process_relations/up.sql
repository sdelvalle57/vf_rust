ALTER TABLE recipe_process_relations
ADD COLUMN recipe_id UUID NOT NULL,
ADD CONSTRAINT fk_recipe_id FOREIGN KEY (recipe_id) REFERENCES recipes(id);
