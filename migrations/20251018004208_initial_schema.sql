-- Initial schema migration - Core tables
-- Phase 3: Database Schema Implementation

-- Enable UUID extension
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";
CREATE EXTENSION IF NOT EXISTS "pgcrypto";

-- Create update_updated_at trigger function
CREATE OR REPLACE FUNCTION update_updated_at_column()
RETURNS TRIGGER AS $$
BEGIN
    NEW.updated_at = CURRENT_TIMESTAMP;
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

-- ====================================================================
-- PROJECTS TABLE
-- ====================================================================
CREATE TABLE projects (
    -- Primary Key
    id BIGSERIAL PRIMARY KEY,

    -- Unique identifier (UUID for external references)
    uuid UUID NOT NULL UNIQUE DEFAULT gen_random_uuid(),

    -- Project Information
    name VARCHAR(255) NOT NULL,
    description TEXT,
    code VARCHAR(50) UNIQUE,

    -- Status and Priority
    status VARCHAR(50) NOT NULL DEFAULT 'planning',
        -- Values: planning, active, on_hold, completed, archived, cancelled
    priority VARCHAR(20) NOT NULL DEFAULT 'medium',
        -- Values: low, medium, high, critical

    -- Dates
    start_date DATE,
    end_date DATE,
    actual_start_date DATE,
    actual_end_date DATE,

    -- Progress Tracking
    progress_percentage INTEGER DEFAULT 0 CHECK (progress_percentage BETWEEN 0 AND 100),

    -- Repository Information
    repository_url TEXT,
    repository_branch VARCHAR(100) DEFAULT 'main',

    -- Metadata
    tags TEXT[],
    metadata JSONB,

    -- Audit Fields
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
    deleted_at TIMESTAMP WITH TIME ZONE,

    -- Constraints
    CONSTRAINT valid_dates CHECK (
        (start_date IS NULL OR end_date IS NULL) OR (start_date <= end_date)
    ),
    CONSTRAINT valid_actual_dates CHECK (
        (actual_start_date IS NULL OR actual_end_date IS NULL) OR
        (actual_start_date <= actual_end_date)
    )
);

-- Trigger for projects updated_at
CREATE TRIGGER update_projects_updated_at
    BEFORE UPDATE ON projects
    FOR EACH ROW
    EXECUTE FUNCTION update_updated_at_column();

-- ====================================================================
-- TASKS TABLE
-- ====================================================================
CREATE TABLE tasks (
    -- Primary Key
    id BIGSERIAL PRIMARY KEY,

    -- Unique identifier
    uuid UUID NOT NULL UNIQUE DEFAULT gen_random_uuid(),

    -- Foreign Key
    project_id BIGINT NOT NULL REFERENCES projects(id) ON DELETE CASCADE,
    parent_task_id BIGINT REFERENCES tasks(id) ON DELETE SET NULL,

    -- Task Information
    title VARCHAR(500) NOT NULL,
    description TEXT,
    task_number VARCHAR(50),

    -- Status and Priority
    status VARCHAR(50) NOT NULL DEFAULT 'todo',
        -- Values: todo, in_progress, blocked, review, testing, done, cancelled
    priority VARCHAR(20) NOT NULL DEFAULT 'medium',
        -- Values: low, medium, high, critical

    -- Assignment
    assigned_to VARCHAR(100),

    -- Estimation and Tracking
    estimated_hours DECIMAL(10, 2),
    actual_hours DECIMAL(10, 2),

    -- Dates
    due_date TIMESTAMP WITH TIME ZONE,
    started_at TIMESTAMP WITH TIME ZONE,
    completed_at TIMESTAMP WITH TIME ZONE,

    -- Classification
    task_type VARCHAR(50) DEFAULT 'feature',
        -- Values: feature, bug, enhancement, refactor, docs, test, chore

    -- Tags and Metadata
    tags TEXT[],
    metadata JSONB,

    -- Audit Fields
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
    deleted_at TIMESTAMP WITH TIME ZONE,

    -- Constraints
    CONSTRAINT valid_hours CHECK (
        (estimated_hours IS NULL OR estimated_hours >= 0) AND
        (actual_hours IS NULL OR actual_hours >= 0)
    )
);

-- Trigger for tasks updated_at
CREATE TRIGGER update_tasks_updated_at
    BEFORE UPDATE ON tasks
    FOR EACH ROW
    EXECUTE FUNCTION update_updated_at_column();

-- ====================================================================
-- TASK_DEPENDENCIES TABLE
-- ====================================================================
CREATE TABLE task_dependencies (
    -- Composite Primary Key
    task_id BIGINT NOT NULL REFERENCES tasks(id) ON DELETE CASCADE,
    depends_on_task_id BIGINT NOT NULL REFERENCES tasks(id) ON DELETE CASCADE,

    -- Dependency Type
    dependency_type VARCHAR(50) NOT NULL DEFAULT 'finish_to_start',
        -- Values: finish_to_start, start_to_start, finish_to_finish, start_to_finish

    -- Metadata
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,

    -- Constraints
    PRIMARY KEY (task_id, depends_on_task_id),
    CONSTRAINT no_self_dependency CHECK (task_id != depends_on_task_id)
);

-- ====================================================================
-- TASK_COMMENTS TABLE
-- ====================================================================
CREATE TABLE task_comments (
    -- Primary Key
    id BIGSERIAL PRIMARY KEY,

    -- Foreign Key
    task_id BIGINT NOT NULL REFERENCES tasks(id) ON DELETE CASCADE,

    -- Comment Information
    comment_text TEXT NOT NULL,
    author VARCHAR(100),

    -- Audit Fields
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
    deleted_at TIMESTAMP WITH TIME ZONE
);

-- Trigger for task_comments updated_at
CREATE TRIGGER update_task_comments_updated_at
    BEFORE UPDATE ON task_comments
    FOR EACH ROW
    EXECUTE FUNCTION update_updated_at_column();
