-- Timeline Management Tables Migration
-- Phase 3: Database Schema Implementation

-- ====================================================================
-- TIMELINES TABLE
-- ====================================================================
CREATE TABLE timelines (
    -- Primary Key
    id BIGSERIAL PRIMARY KEY,

    -- Foreign Key
    project_id BIGINT NOT NULL REFERENCES projects(id) ON DELETE CASCADE,

    -- Timeline Information
    name VARCHAR(255) NOT NULL,
    description TEXT,

    -- Timeline Type
    timeline_type VARCHAR(50) DEFAULT 'project',
        -- Values: project, sprint, release, phase

    -- Dates
    start_date DATE NOT NULL,
    end_date DATE NOT NULL,

    -- Status
    status VARCHAR(50) DEFAULT 'planned',
        -- Values: planned, active, completed, cancelled

    -- Audit Fields
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
    deleted_at TIMESTAMP WITH TIME ZONE,

    -- Constraints
    CONSTRAINT valid_timeline_dates CHECK (start_date <= end_date)
);

-- Trigger for timelines updated_at
CREATE TRIGGER update_timelines_updated_at
    BEFORE UPDATE ON timelines
    FOR EACH ROW
    EXECUTE FUNCTION update_updated_at_column();

-- ====================================================================
-- MILESTONES TABLE
-- ====================================================================
CREATE TABLE milestones (
    -- Primary Key
    id BIGSERIAL PRIMARY KEY,

    -- Foreign Keys
    timeline_id BIGINT NOT NULL REFERENCES timelines(id) ON DELETE CASCADE,
    project_id BIGINT NOT NULL REFERENCES projects(id) ON DELETE CASCADE,

    -- Milestone Information
    name VARCHAR(255) NOT NULL,
    description TEXT,

    -- Dates
    target_date DATE NOT NULL,
    actual_date DATE,

    -- Status
    status VARCHAR(50) DEFAULT 'pending',
        -- Values: pending, in_progress, completed, missed, cancelled

    -- Progress
    completion_percentage INTEGER DEFAULT 0 CHECK (completion_percentage BETWEEN 0 AND 100),

    -- Metadata
    metadata JSONB,

    -- Audit Fields
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
    deleted_at TIMESTAMP WITH TIME ZONE
);

-- Trigger for milestones updated_at
CREATE TRIGGER update_milestones_updated_at
    BEFORE UPDATE ON milestones
    FOR EACH ROW
    EXECUTE FUNCTION update_updated_at_column()
