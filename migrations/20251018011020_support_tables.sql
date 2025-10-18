-- Support Tables Migration (Tags, Configuration, Audit)
-- Phase 3: Database Schema Implementation

-- ====================================================================
-- TAGS TABLE
-- ====================================================================
CREATE TABLE tags (
    -- Primary Key
    id BIGSERIAL PRIMARY KEY,

    -- Tag Information
    name VARCHAR(100) NOT NULL UNIQUE,
    description TEXT,
    color VARCHAR(7), -- Hex color code (e.g., #FF5733)

    -- Tag Type
    tag_type VARCHAR(50) DEFAULT 'general',
        -- Values: general, technology, category, status, priority

    -- Audit Fields
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP
);

-- Trigger for tags updated_at
CREATE TRIGGER update_tags_updated_at
    BEFORE UPDATE ON tags
    FOR EACH ROW
    EXECUTE FUNCTION update_updated_at_column();

-- ====================================================================
-- PROJECT_TAGS TABLE
-- ====================================================================
CREATE TABLE project_tags (
    project_id BIGINT NOT NULL REFERENCES projects(id) ON DELETE CASCADE,
    tag_id BIGINT NOT NULL REFERENCES tags(id) ON DELETE CASCADE,

    -- Audit
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,

    -- Constraints
    PRIMARY KEY (project_id, tag_id)
);

-- ====================================================================
-- CONFIGURATIONS TABLE
-- ====================================================================
CREATE TABLE configurations (
    -- Primary Key
    id BIGSERIAL PRIMARY KEY,

    -- Configuration Key-Value
    config_key VARCHAR(255) NOT NULL UNIQUE,
    config_value TEXT NOT NULL,

    -- Configuration Metadata
    description TEXT,
    data_type VARCHAR(50) DEFAULT 'string',
        -- Values: string, integer, boolean, json

    -- Validation
    is_encrypted BOOLEAN DEFAULT FALSE,
    is_required BOOLEAN DEFAULT FALSE,

    -- Audit Fields
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP
);

-- Trigger for configurations updated_at
CREATE TRIGGER update_configurations_updated_at
    BEFORE UPDATE ON configurations
    FOR EACH ROW
    EXECUTE FUNCTION update_updated_at_column();

-- ====================================================================
-- AUDIT_LOGS TABLE
-- ====================================================================
CREATE TABLE audit_logs (
    -- Primary Key
    id BIGSERIAL PRIMARY KEY,

    -- Event Information
    entity_type VARCHAR(100) NOT NULL,
        -- Values: project, task, resource, etc.
    entity_id BIGINT NOT NULL,
    action VARCHAR(50) NOT NULL,
        -- Values: create, update, delete, status_change

    -- User Information
    user_identifier VARCHAR(100),

    -- Change Details
    old_values JSONB,
    new_values JSONB,

    -- Additional Context
    description TEXT,
    metadata JSONB,

    -- Timestamp
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP
)
