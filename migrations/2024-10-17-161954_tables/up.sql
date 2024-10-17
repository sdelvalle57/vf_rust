CREATE EXTENSION IF NOT EXISTS "uuid-ossp";
CREATE TYPE resource_type_enum AS ENUM ('Product', 'Resource', 'Asset');
CREATE TYPE template_type_enum AS ENUM ('FDA', 'Custom');
CREATE TYPE event_type_enum AS ENUM ('EconomicEvent');
CREATE TYPE action_type_enum AS ENUM ('Cite', 'Modify', 'Produce', 'Consume', 'Transfer', 'Use', 'Load', 'Unload', 'Accept', 'Dispatch');
CREATE TYPE role_type_enum AS ENUM ('Input', 'Output');
CREATE TYPE field_class_enum AS ENUM (
    'resourceSpecification', 
    'economicResource',
    'quantity', 
    'hasPointInTime', 
    'agent', 
    'location', 
    'note', 
    'trackingIdentifier', 
    'custom',
    'referenceDocumentNumber',
    'referenceDocumentType'

);
CREATE TYPE field_group_class_enum AS ENUM ('ResourceSpecification', 'EconomicResource', 'Location', 'Custom', 'ReferenceDocument');
CREATE TYPE field_type_enum AS ENUM ('Text', 'Date', 'Number', 'Select');
CREATE TYPE flow_through_enum AS ENUM ('Internal', 'External');

-- Company Table
CREATE TABLE IF NOT EXISTS agents (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    name TEXT NOT NULL,
    note TEXT,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);

-- Products Specification Table
CREATE TABLE IF NOT EXISTS resource_specifications (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    agent_id UUID NOT NULL REFERENCES agents(id),
    name TEXT NOT NULL,
    note TEXT,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    resource_type resource_type_enum NOT NULL,
    unit_of_measure TEXT NOT NULL
);

-- Products Table, Inventory
CREATE TABLE IF NOT EXISTS economic_resources (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    resource_specification_id UUID NOT NULL REFERENCES resource_specifications(id),
    name TEXT NOT NULL,
    note TEXT,
    accounting_quantity INTEGER NOT NULL,
    on_hand_quantity INTEGER NOT NULL,
    tracking_identifier TEXT,
    current_location TEXT NOT NULL,
    lot TEXT,
    contained_in UUID REFERENCES economic_resources(id),
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    reference_number SERIAL
);

-- ProcessTemplates
CREATE TABLE IF NOT EXISTS map_templates (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    name TEXT NOT NULL,
    type template_type_enum NOT NULL
);

-- ProcessTemplates
CREATE TABLE IF NOT EXISTS recipe_templates (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    map_template_id UUID NOT NULL REFERENCES map_templates(id),
    identifier TEXT NOT NULL, 
    name TEXT NOT NULL,
    commitment action_type_enum,
    fulfills UUID REFERENCES recipe_templates(id),
    trigger action_type_enum
);

-- ProcessWhitelistRules, 
CREATE TABLE IF NOT EXISTS recipe_template_blacklists (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    map_template_id UUID NOT NULL REFERENCES map_templates(id),
    recipe_template_id UUID NOT NULL REFERENCES recipe_templates(id),
    recipe_template_predecesor_id UUID NOT NULL REFERENCES recipe_templates(id),
    CONSTRAINT unique_process_restriction UNIQUE (recipe_template_id, recipe_template_predecesor_id)
);

-- ProcessTemplateAccess
CREATE TABLE IF NOT EXISTS recipe_templates_access (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    agent_id UUID NOT NULL REFERENCES agents(id),
    recipe_template_id UUID NOT NULL REFERENCES recipe_templates(id)
);

-- ProcessFlowTemplates
CREATE TABLE IF NOT EXISTS recipe_flow_templates (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    recipe_template_id UUID NOT NULL REFERENCES recipe_templates(id),
    event_type event_type_enum NOT NULL,
    role_type role_type_enum NOT NULL,
    action action_type_enum NOT NULL,
    identifier TEXT NOT NULL,
    interactions INTEGER
);

-- ProcessFlowTemplateGroupDataFields
CREATE TABLE IF NOT EXISTS recipe_flow_template_group_data_fields (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    name TEXT NOT NULL,
    group_class field_group_class_enum NOT NULL
);

-- ProcessFlowTemplateDataFields
CREATE TABLE IF NOT EXISTS recipe_flow_template_data_fields (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    recipe_flow_template_id UUID NOT NULL REFERENCES recipe_flow_templates(id),
    group_id UUID REFERENCES recipe_flow_template_group_data_fields(id),
    field_identifier TEXT NOT NULL,
    field_class field_class_enum NOT NULL,
    field TEXT NOT NULL,
    field_type field_type_enum NOT NULL,
    note TEXT,
    required BOOLEAN NOT NULL,
    flow_through flow_through_enum,
    inherits UUID REFERENCES recipe_flow_template_data_fields(id),
    accept_default BOOLEAN NOT NULL
);

-- Locations
CREATE TABLE IF NOT EXISTS locations (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    agent_id UUID NOT NULL REFERENCES agents(id),
    name TEXT NOT NULL,
    value TEXT NOT NULL
);


-- Counters, product lot_codes and reference_numbers
CREATE TABLE IF NOT EXISTS counters (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    agent_id UUID NOT NULL REFERENCES agents(id),
    lot_code INTEGER NOT NULL DEFAULT 0,
    reference_number INTEGER NOT NULL DEFAULT 0,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);

-- Process Map
CREATE TABLE IF NOT EXISTS recipes (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    agent_id UUID NOT NULL REFERENCES agents(id),
    name TEXT NOT NULL,
    note TEXT,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);

-- Process Map Products
CREATE TABLE IF NOT EXISTS recipe_resources (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    recipe_id UUID NOT NULL REFERENCES recipes(id),
    resource_specification_id UUID NOT NULL REFERENCES resource_specifications(id),
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);

-- Process Map process, Process
CREATE TABLE IF NOT EXISTS recipe_processes (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    recipe_id UUID NOT NULL REFERENCES recipes(id),
    recipe_template_id UUID REFERENCES recipe_templates(id),
    name TEXT NOT NULL,
    commitment action_type_enum,
    fulfills UUID REFERENCES recipe_processes(id),
    identifier TEXT NOT NULL UNIQUE,
    trigger action_type_enum
);

-- Process connection relations
CREATE TABLE IF NOT EXISTS recipe_process_relations (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    recipe_process_id UUID NOT NULL REFERENCES recipe_processes(id),
    output_of UUID NOT NULL REFERENCES recipe_processes(id)  -- Corrected the syntax here
);

-- Process Flow, FORM
CREATE TABLE IF NOT EXISTS recipe_process_flows (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    recipe_process_id UUID NOT NULL REFERENCES recipe_processes(id),
    recipe_flow_template_id UUID NOT NULL REFERENCES recipe_flow_templates(id),
    event_type event_type_enum NOT NULL,
    role_type role_type_enum NOT NULL,
    action action_type_enum NOT NULL,
    identifier TEXT NOT NULL UNIQUE
);

-- Process Flow Group Data Fields
CREATE TABLE IF NOT EXISTS recipe_process_flow_group_data_fields (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    name TEXT NOT NULL,
    group_class field_group_class_enum NOT NULL
);

-- Process Flow Data Fields
CREATE TABLE IF NOT EXISTS recipe_process_flow_data_fields (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    recipe_process_flow_id UUID NOT NULL REFERENCES recipe_process_flows(id),
    recipe_flow_template_data_field_id UUID REFERENCES recipe_flow_template_data_fields(id),
    group_id UUID REFERENCES recipe_process_flow_group_data_fields(id),
    field_identifier TEXT NOT NULL,
    field_class field_class_enum NOT NULL,
    field TEXT NOT NULL,
    field_type field_type_enum NOT NULL,
    note TEXT,
    required BOOLEAN NOT NULL,
    default_value TEXT,
    flow_through flow_through_enum,
    inherits UUID REFERENCES recipe_process_flow_data_fields(id)
);

-- Process Executions
CREATE TABLE IF NOT EXISTS process_executions (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    process_flow_id UUID NOT NULL REFERENCES recipe_process_flows(id),
    action action_type_enum NOT NULL,
    role_type role_type_enum NOT NULL,
    resource_specification UUID REFERENCES resource_specifications(id),
    resource_reference_number INTEGER,
    resource_lot_number INTEGER,
    resource_quantity INTEGER,
    to_resource_specification UUID REFERENCES resource_specifications(id),
    to_resource_reference_number INTEGER,
    to_resource_lot_number INTEGER,
    provider_agent UUID NOT NULL REFERENCES agents(id),
    receiver_agent UUID NOT NULL REFERENCES agents(id),
    at_location UUID REFERENCES locations(id),
    to_location UUID REFERENCES locations(id),
    has_point_in_time TIMESTAMP,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    corrects UUID REFERENCES process_executions(id),
    note TEXT
);

-- Process Execution Custom Values
CREATE TABLE IF NOT EXISTS process_execution_custom_values (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    process_execution_id UUID NOT NULL REFERENCES process_executions(id),
    field_id UUID NOT NULL REFERENCES recipe_process_flow_data_fields(id),
    field_value TEXT NOT NULL,
    corrects UUID REFERENCES process_execution_custom_values(id),
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);