ALTER TABLE recipe_processes
    ALTER COLUMN recipe_template_id SET NOT NULL;

ALTER TABLE recipe_processes
    ADD CONSTRAINT recipe_template_id_fk
    FOREIGN KEY (recipe_template_id) REFERENCES recipe_templates(id);
