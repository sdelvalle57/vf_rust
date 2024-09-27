ALTER TABLE recipe_process_flows
ADD COLUMN identifier TEXT NOT NULL UNIQUE;
