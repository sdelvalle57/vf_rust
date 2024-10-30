ALTER TABLE recipe_templates
ADD COLUMN version INTEGER NOT NULL DEFAULT 1,
ADD COLUMN overriden_by UUID REFERENCES recipe_templates(id),
ADD COLUMN created_by UUID REFERENCES agents(id);
