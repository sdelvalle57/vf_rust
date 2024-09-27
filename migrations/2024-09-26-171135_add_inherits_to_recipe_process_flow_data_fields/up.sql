ALTER TABLE recipe_process_flow_data_fields
ADD COLUMN inherits UUID REFERENCES recipe_process_flow_data_fields(id);
