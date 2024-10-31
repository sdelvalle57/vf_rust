ALTER TABLE recipe_templates
ADD COLUMN first_version UUID REFERENCES recipe_templates(id);
