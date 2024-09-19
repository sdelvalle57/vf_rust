-- Drop the `lot_codes` table
DROP TABLE IF EXISTS lot_codes;


CREATE TABLE IF NOT EXISTS counters (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    agent_id UUID NOT NULL REFERENCES agents(id),
    lot_code INTEGER NOT NULL DEFAULT 0,
    reference_number INTEGER NOT NULL DEFAULT 0
);