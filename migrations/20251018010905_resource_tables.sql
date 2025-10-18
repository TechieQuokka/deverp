-- Resource Management Tables Migration
-- Phase 3: Database Schema Implementation

-- ====================================================================
-- RESOURCES TABLE
-- ====================================================================
CREATE TABLE resources (
    -- Primary Key
    id BIGSERIAL PRIMARY KEY,

    -- Unique identifier
    uuid UUID NOT NULL UNIQUE DEFAULT gen_random_uuid(),

    -- Resource Information
    name VARCHAR(255) NOT NULL,
    description TEXT,
    resource_type VARCHAR(50) NOT NULL,
        -- Values: library, api, tool, service, documentation, other

    -- Resource Details
    version VARCHAR(100),
    url TEXT,
    documentation_url TEXT,
    license VARCHAR(100),

    -- Status
    status VARCHAR(50) DEFAULT 'active',
        -- Values: active, deprecated, archived

    -- Metadata
    metadata JSONB,
    tags TEXT[],

    -- Audit Fields
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
    deleted_at TIMESTAMP WITH TIME ZONE
);

-- Trigger for resources updated_at
CREATE TRIGGER update_resources_updated_at
    BEFORE UPDATE ON resources
    FOR EACH ROW
    EXECUTE FUNCTION update_updated_at_column();

-- ====================================================================
-- PROJECT_RESOURCES TABLE
-- ====================================================================
CREATE TABLE project_resources (
    -- Composite Primary Key
    project_id BIGINT NOT NULL REFERENCES projects(id) ON DELETE CASCADE,
    resource_id BIGINT NOT NULL REFERENCES resources(id) ON DELETE CASCADE,

    -- Usage Information
    usage_notes TEXT,
    version_used VARCHAR(100),
    is_critical BOOLEAN DEFAULT FALSE,

    -- Audit Fields
    added_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
    removed_at TIMESTAMP WITH TIME ZONE,

    -- Constraints
    PRIMARY KEY (project_id, resource_id)
)
