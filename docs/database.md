# DevERP - Database Design Document

## 1. Overview

### 1.1 Database Information
- **Database System**: PostgreSQL 14+
- **Purpose**: Persistent storage for DevERP project management system
- **Deployment**: Local PostgreSQL instance
- **Character Encoding**: UTF-8
- **Timezone**: UTC

### 1.2 Design Principles
- **Normalization**: Minimum 3NF (Third Normal Form)
- **Referential Integrity**: Foreign key constraints enforced
- **Data Integrity**: Check constraints and triggers where appropriate
- **Audit Trail**: Created/updated timestamps on all entities
- **Soft Deletes**: Logical deletion with `deleted_at` timestamp

---

## 2. Entity Relationship Diagram

```
┌─────────────────┐
│    projects     │
└────────┬────────┘
         │ 1
         │
         │ *
┌────────▼────────┐       ┌──────────────────┐
│     tasks       │───────│ task_dependencies│
└────────┬────────┘  *  * └──────────────────┘
         │ 1
         │
         │ *
┌────────▼────────┐
│  task_comments  │
└─────────────────┘

┌─────────────────┐
│    resources    │
└────────┬────────┘
         │ 1
         │
         │ *
┌────────▼────────┐
│project_resources│──────┐
└─────────────────┘      │
                         │
                    ┌────▼────┐
                    │projects │
                    └─────────┘

┌─────────────────┐
│   timelines     │
└────────┬────────┘
         │ 1
         │
         │ *
┌────────▼────────┐
│   milestones    │
└─────────────────┘

┌─────────────────┐
│   tags          │
└────────┬────────┘
         │ *
         │
         │ *
┌────────▼────────┐
│  project_tags   │──────┐
└─────────────────┘      │
                    ┌────▼────┐
                    │projects │
                    └─────────┘
```

---

## 3. Database Schema

### 3.1 Core Tables

#### 3.1.1 projects

**Purpose**: Store project information and metadata

```sql
CREATE TABLE projects (
    -- Primary Key
    id BIGSERIAL PRIMARY KEY,

    -- Unique identifier (UUID for external references)
    uuid UUID NOT NULL UNIQUE DEFAULT gen_random_uuid(),

    -- Project Information
    name VARCHAR(255) NOT NULL,
    description TEXT,
    code VARCHAR(50) UNIQUE, -- Short code like "PROJ-001"

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
    tags TEXT[], -- Array of tag names
    metadata JSONB, -- Additional flexible metadata

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

-- Indexes
CREATE INDEX idx_projects_status ON projects(status) WHERE deleted_at IS NULL;
CREATE INDEX idx_projects_priority ON projects(priority) WHERE deleted_at IS NULL;
CREATE INDEX idx_projects_dates ON projects(start_date, end_date) WHERE deleted_at IS NULL;
CREATE INDEX idx_projects_code ON projects(code) WHERE deleted_at IS NULL;
CREATE INDEX idx_projects_tags ON projects USING gin(tags);
CREATE INDEX idx_projects_metadata ON projects USING gin(metadata);

-- Trigger for updated_at
CREATE TRIGGER update_projects_updated_at
    BEFORE UPDATE ON projects
    FOR EACH ROW
    EXECUTE FUNCTION update_updated_at_column();
```

#### 3.1.2 tasks

**Purpose**: Store task information for projects

```sql
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
    task_number VARCHAR(50), -- e.g., "TASK-001"

    -- Status and Priority
    status VARCHAR(50) NOT NULL DEFAULT 'todo',
        -- Values: todo, in_progress, blocked, review, testing, done, cancelled
    priority VARCHAR(20) NOT NULL DEFAULT 'medium',
        -- Values: low, medium, high, critical

    -- Assignment
    assigned_to VARCHAR(100), -- Developer name or identifier

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

-- Indexes
CREATE INDEX idx_tasks_project_id ON tasks(project_id) WHERE deleted_at IS NULL;
CREATE INDEX idx_tasks_status ON tasks(status) WHERE deleted_at IS NULL;
CREATE INDEX idx_tasks_priority ON tasks(priority) WHERE deleted_at IS NULL;
CREATE INDEX idx_tasks_assigned_to ON tasks(assigned_to) WHERE deleted_at IS NULL;
CREATE INDEX idx_tasks_due_date ON tasks(due_date) WHERE deleted_at IS NULL;
CREATE INDEX idx_tasks_parent_task_id ON tasks(parent_task_id) WHERE deleted_at IS NULL;
CREATE INDEX idx_tasks_tags ON tasks USING gin(tags);

-- Trigger for updated_at
CREATE TRIGGER update_tasks_updated_at
    BEFORE UPDATE ON tasks
    FOR EACH ROW
    EXECUTE FUNCTION update_updated_at_column();
```

#### 3.1.3 task_dependencies

**Purpose**: Define dependencies between tasks

```sql
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

-- Indexes
CREATE INDEX idx_task_deps_task_id ON task_dependencies(task_id);
CREATE INDEX idx_task_deps_depends_on ON task_dependencies(depends_on_task_id);
```

#### 3.1.4 task_comments

**Purpose**: Store comments and notes on tasks

```sql
CREATE TABLE task_comments (
    -- Primary Key
    id BIGSERIAL PRIMARY KEY,

    -- Foreign Key
    task_id BIGINT NOT NULL REFERENCES tasks(id) ON DELETE CASCADE,

    -- Comment Information
    comment_text TEXT NOT NULL,
    author VARCHAR(100), -- Comment author

    -- Audit Fields
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
    deleted_at TIMESTAMP WITH TIME ZONE
);

-- Indexes
CREATE INDEX idx_task_comments_task_id ON task_comments(task_id) WHERE deleted_at IS NULL;
CREATE INDEX idx_task_comments_created_at ON task_comments(created_at DESC);

-- Trigger for updated_at
CREATE TRIGGER update_task_comments_updated_at
    BEFORE UPDATE ON task_comments
    FOR EACH ROW
    EXECUTE FUNCTION update_updated_at_column();
```

### 3.2 Resource Management Tables

#### 3.2.1 resources

**Purpose**: Track development resources (libraries, tools, APIs, etc.)

```sql
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

-- Indexes
CREATE INDEX idx_resources_type ON resources(resource_type) WHERE deleted_at IS NULL;
CREATE INDEX idx_resources_status ON resources(status) WHERE deleted_at IS NULL;
CREATE INDEX idx_resources_name ON resources(name) WHERE deleted_at IS NULL;

-- Trigger for updated_at
CREATE TRIGGER update_resources_updated_at
    BEFORE UPDATE ON resources
    FOR EACH ROW
    EXECUTE FUNCTION update_updated_at_column();
```

#### 3.2.2 project_resources

**Purpose**: Link resources to projects

```sql
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
);

-- Indexes
CREATE INDEX idx_project_resources_project_id ON project_resources(project_id);
CREATE INDEX idx_project_resources_resource_id ON project_resources(resource_id);
```

### 3.3 Timeline Management Tables

#### 3.3.1 timelines

**Purpose**: Store project timelines and schedules

```sql
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

-- Indexes
CREATE INDEX idx_timelines_project_id ON timelines(project_id) WHERE deleted_at IS NULL;
CREATE INDEX idx_timelines_dates ON timelines(start_date, end_date);

-- Trigger for updated_at
CREATE TRIGGER update_timelines_updated_at
    BEFORE UPDATE ON timelines
    FOR EACH ROW
    EXECUTE FUNCTION update_updated_at_column();
```

#### 3.3.2 milestones

**Purpose**: Track timeline milestones and key deliverables

```sql
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

-- Indexes
CREATE INDEX idx_milestones_timeline_id ON milestones(timeline_id) WHERE deleted_at IS NULL;
CREATE INDEX idx_milestones_project_id ON milestones(project_id) WHERE deleted_at IS NULL;
CREATE INDEX idx_milestones_target_date ON milestones(target_date);
CREATE INDEX idx_milestones_status ON milestones(status);

-- Trigger for updated_at
CREATE TRIGGER update_milestones_updated_at
    BEFORE UPDATE ON milestones
    FOR EACH ROW
    EXECUTE FUNCTION update_updated_at_column();
```

### 3.4 Tag Management Tables

#### 3.4.1 tags

**Purpose**: Centralized tag management for better consistency

```sql
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

-- Indexes
CREATE INDEX idx_tags_name ON tags(name);
CREATE INDEX idx_tags_type ON tags(tag_type);

-- Trigger for updated_at
CREATE TRIGGER update_tags_updated_at
    BEFORE UPDATE ON tags
    FOR EACH ROW
    EXECUTE FUNCTION update_updated_at_column();
```

#### 3.4.2 project_tags

**Purpose**: Link tags to projects (many-to-many relationship)

```sql
CREATE TABLE project_tags (
    project_id BIGINT NOT NULL REFERENCES projects(id) ON DELETE CASCADE,
    tag_id BIGINT NOT NULL REFERENCES tags(id) ON DELETE CASCADE,

    -- Audit
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,

    -- Constraints
    PRIMARY KEY (project_id, tag_id)
);

-- Indexes
CREATE INDEX idx_project_tags_project_id ON project_tags(project_id);
CREATE INDEX idx_project_tags_tag_id ON project_tags(tag_id);
```

### 3.5 Configuration and System Tables

#### 3.5.1 configurations

**Purpose**: Store application configuration and preferences

```sql
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

-- Indexes
CREATE INDEX idx_configurations_key ON configurations(config_key);

-- Trigger for updated_at
CREATE TRIGGER update_configurations_updated_at
    BEFORE UPDATE ON configurations
    FOR EACH ROW
    EXECUTE FUNCTION update_updated_at_column();
```

#### 3.5.2 audit_logs

**Purpose**: Track important system events and changes

```sql
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
);

-- Indexes
CREATE INDEX idx_audit_logs_entity ON audit_logs(entity_type, entity_id);
CREATE INDEX idx_audit_logs_action ON audit_logs(action);
CREATE INDEX idx_audit_logs_created_at ON audit_logs(created_at DESC);
CREATE INDEX idx_audit_logs_user ON audit_logs(user_identifier);
```

---

## 4. Database Functions and Triggers

### 4.1 Update Timestamp Function

```sql
CREATE OR REPLACE FUNCTION update_updated_at_column()
RETURNS TRIGGER AS $$
BEGIN
    NEW.updated_at = CURRENT_TIMESTAMP;
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;
```

### 4.2 Soft Delete Function

```sql
CREATE OR REPLACE FUNCTION soft_delete()
RETURNS TRIGGER AS $$
BEGIN
    IF NEW.deleted_at IS NOT NULL AND OLD.deleted_at IS NULL THEN
        -- Log the deletion
        INSERT INTO audit_logs (entity_type, entity_id, action, old_values)
        VALUES (TG_TABLE_NAME, OLD.id, 'soft_delete', row_to_json(OLD));
    END IF;
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;
```

### 4.3 Audit Log Trigger Function

```sql
CREATE OR REPLACE FUNCTION log_audit_changes()
RETURNS TRIGGER AS $$
BEGIN
    IF TG_OP = 'INSERT' THEN
        INSERT INTO audit_logs (entity_type, entity_id, action, new_values)
        VALUES (TG_TABLE_NAME, NEW.id, 'create', row_to_json(NEW));
    ELSIF TG_OP = 'UPDATE' THEN
        INSERT INTO audit_logs (entity_type, entity_id, action, old_values, new_values)
        VALUES (TG_TABLE_NAME, NEW.id, 'update', row_to_json(OLD), row_to_json(NEW));
    ELSIF TG_OP = 'DELETE' THEN
        INSERT INTO audit_logs (entity_type, entity_id, action, old_values)
        VALUES (TG_TABLE_NAME, OLD.id, 'delete', row_to_json(OLD));
    END IF;
    RETURN NULL;
END;
$$ LANGUAGE plpgsql;
```

### 4.4 Project Progress Calculation Function

```sql
CREATE OR REPLACE FUNCTION calculate_project_progress(p_project_id BIGINT)
RETURNS INTEGER AS $$
DECLARE
    total_tasks INTEGER;
    completed_tasks INTEGER;
    progress INTEGER;
BEGIN
    SELECT COUNT(*) INTO total_tasks
    FROM tasks
    WHERE project_id = p_project_id AND deleted_at IS NULL;

    IF total_tasks = 0 THEN
        RETURN 0;
    END IF;

    SELECT COUNT(*) INTO completed_tasks
    FROM tasks
    WHERE project_id = p_project_id
      AND status = 'done'
      AND deleted_at IS NULL;

    progress := (completed_tasks * 100) / total_tasks;

    RETURN progress;
END;
$$ LANGUAGE plpgsql;
```

---

## 5. Views

### 5.1 Active Projects View

```sql
CREATE OR REPLACE VIEW v_active_projects AS
SELECT
    p.id,
    p.uuid,
    p.name,
    p.code,
    p.status,
    p.priority,
    p.start_date,
    p.end_date,
    p.progress_percentage,
    COUNT(DISTINCT t.id) as total_tasks,
    COUNT(DISTINCT CASE WHEN t.status = 'done' THEN t.id END) as completed_tasks,
    COUNT(DISTINCT CASE WHEN t.status IN ('todo', 'in_progress') THEN t.id END) as active_tasks,
    p.created_at,
    p.updated_at
FROM projects p
LEFT JOIN tasks t ON p.id = t.project_id AND t.deleted_at IS NULL
WHERE p.deleted_at IS NULL
  AND p.status IN ('planning', 'active', 'on_hold')
GROUP BY p.id;
```

### 5.2 Task Summary View

```sql
CREATE OR REPLACE VIEW v_task_summary AS
SELECT
    t.id,
    t.uuid,
    t.title,
    t.task_number,
    t.status,
    t.priority,
    t.task_type,
    p.name as project_name,
    p.code as project_code,
    t.assigned_to,
    t.due_date,
    t.estimated_hours,
    t.actual_hours,
    COUNT(DISTINCT td.depends_on_task_id) as dependency_count,
    COUNT(DISTINCT tc.id) as comment_count,
    t.created_at,
    t.updated_at
FROM tasks t
INNER JOIN projects p ON t.project_id = p.id
LEFT JOIN task_dependencies td ON t.id = td.task_id
LEFT JOIN task_comments tc ON t.id = tc.task_id AND tc.deleted_at IS NULL
WHERE t.deleted_at IS NULL
GROUP BY t.id, p.name, p.code;
```

### 5.3 Resource Usage View

```sql
CREATE OR REPLACE VIEW v_resource_usage AS
SELECT
    r.id,
    r.name,
    r.resource_type,
    r.version,
    COUNT(DISTINCT pr.project_id) as project_count,
    COUNT(DISTINCT CASE WHEN pr.is_critical = TRUE THEN pr.project_id END) as critical_project_count,
    r.status,
    r.created_at
FROM resources r
LEFT JOIN project_resources pr ON r.id = pr.resource_id AND pr.removed_at IS NULL
WHERE r.deleted_at IS NULL
GROUP BY r.id;
```

### 5.4 Project Timeline View

```sql
CREATE OR REPLACE VIEW v_project_timeline AS
SELECT
    p.id as project_id,
    p.name as project_name,
    t.id as timeline_id,
    t.name as timeline_name,
    t.timeline_type,
    t.start_date,
    t.end_date,
    t.status as timeline_status,
    COUNT(DISTINCT m.id) as milestone_count,
    COUNT(DISTINCT CASE WHEN m.status = 'completed' THEN m.id END) as completed_milestones,
    MIN(m.target_date) as next_milestone_date
FROM projects p
INNER JOIN timelines t ON p.id = t.project_id
LEFT JOIN milestones m ON t.id = m.timeline_id AND m.deleted_at IS NULL
WHERE p.deleted_at IS NULL
  AND t.deleted_at IS NULL
GROUP BY p.id, p.name, t.id, t.name, t.timeline_type, t.start_date, t.end_date, t.status;
```

---

## 6. Database Initialization Script

### 6.1 Initial Setup (001_initial_schema.sql)

```sql
-- Enable UUID extension
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";
CREATE EXTENSION IF NOT EXISTS "pgcrypto";

-- Create update_updated_at function
CREATE OR REPLACE FUNCTION update_updated_at_column()
RETURNS TRIGGER AS $$
BEGIN
    NEW.updated_at = CURRENT_TIMESTAMP;
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

-- Create tables (refer to section 3 for complete DDL)

-- Insert default configurations
INSERT INTO configurations (config_key, config_value, description, data_type) VALUES
('default_project_status', 'planning', 'Default status for new projects', 'string'),
('default_task_status', 'todo', 'Default status for new tasks', 'string'),
('date_format', '%Y-%m-%d', 'Default date format', 'string'),
('enable_audit_log', 'true', 'Enable audit logging', 'boolean');

-- Insert default tags
INSERT INTO tags (name, description, tag_type, color) VALUES
('rust', 'Rust programming language', 'technology', '#CE412B'),
('postgresql', 'PostgreSQL database', 'technology', '#336791'),
('cli', 'Command-line interface', 'category', '#000000'),
('backend', 'Backend development', 'category', '#3178C6'),
('frontend', 'Frontend development', 'category', '#61DAFB'),
('bug', 'Bug fix', 'status', '#D73A4A'),
('feature', 'New feature', 'status', '#0E8A16'),
('documentation', 'Documentation', 'category', '#0075CA');
```

---

## 7. Query Examples

### 7.1 Common Queries

#### Get all active projects with task statistics
```sql
SELECT * FROM v_active_projects
ORDER BY priority DESC, start_date ASC;
```

#### Get overdue tasks
```sql
SELECT
    t.id,
    t.title,
    t.due_date,
    p.name as project_name,
    t.status,
    t.priority
FROM tasks t
INNER JOIN projects p ON t.project_id = p.id
WHERE t.deleted_at IS NULL
  AND t.status NOT IN ('done', 'cancelled')
  AND t.due_date < CURRENT_TIMESTAMP
ORDER BY t.due_date ASC;
```

#### Get project progress
```sql
SELECT
    p.id,
    p.name,
    p.progress_percentage as manual_progress,
    calculate_project_progress(p.id) as calculated_progress,
    COUNT(t.id) as total_tasks,
    COUNT(CASE WHEN t.status = 'done' THEN 1 END) as completed_tasks
FROM projects p
LEFT JOIN tasks t ON p.id = t.project_id AND t.deleted_at IS NULL
WHERE p.deleted_at IS NULL
GROUP BY p.id, p.name, p.progress_percentage;
```

#### Get task dependencies
```sql
SELECT
    t1.id as task_id,
    t1.title as task_title,
    t2.id as depends_on_task_id,
    t2.title as depends_on_title,
    t2.status as dependency_status,
    td.dependency_type
FROM tasks t1
INNER JOIN task_dependencies td ON t1.id = td.task_id
INNER JOIN tasks t2 ON td.depends_on_task_id = t2.id
WHERE t1.deleted_at IS NULL AND t2.deleted_at IS NULL
ORDER BY t1.id;
```

### 7.2 Analytics Queries

#### Project workload analysis
```sql
SELECT
    p.name,
    COUNT(DISTINCT t.id) as total_tasks,
    SUM(t.estimated_hours) as total_estimated_hours,
    SUM(t.actual_hours) as total_actual_hours,
    AVG(t.actual_hours - t.estimated_hours) as avg_variance
FROM projects p
LEFT JOIN tasks t ON p.id = t.project_id AND t.deleted_at IS NULL
WHERE p.deleted_at IS NULL
GROUP BY p.id, p.name
ORDER BY total_estimated_hours DESC;
```

#### Resource utilization
```sql
SELECT
    r.name,
    r.resource_type,
    COUNT(DISTINCT pr.project_id) as used_in_projects,
    STRING_AGG(DISTINCT p.name, ', ') as project_names
FROM resources r
LEFT JOIN project_resources pr ON r.id = pr.resource_id AND pr.removed_at IS NULL
LEFT JOIN projects p ON pr.project_id = p.id AND p.deleted_at IS NULL
WHERE r.deleted_at IS NULL
GROUP BY r.id, r.name, r.resource_type
ORDER BY used_in_projects DESC;
```

---

## 8. Performance Optimization

### 8.1 Index Strategy
- Primary keys automatically indexed
- Foreign keys indexed for join performance
- Status fields indexed for filtering
- Date fields indexed for range queries
- JSONB fields use GIN indexes for flexible queries
- Array fields (tags) use GIN indexes

### 8.2 Query Optimization Tips
- Use prepared statements to prevent SQL injection and improve performance
- Utilize views for complex recurring queries
- Implement pagination for large result sets
- Use EXPLAIN ANALYZE to identify slow queries
- Consider materialized views for heavy analytics queries

### 8.3 Partitioning Considerations
For future scalability, consider partitioning:
- `audit_logs` by date (monthly or yearly)
- `task_comments` by project_id or date

---

## 9. Backup and Maintenance

### 9.1 Backup Strategy
```bash
# Full database backup
pg_dump -U deverp_user -d deverp > deverp_backup_$(date +%Y%m%d).sql

# Restore from backup
psql -U deverp_user -d deverp < deverp_backup_20250118.sql
```

### 9.2 Maintenance Tasks
```sql
-- Vacuum and analyze
VACUUM ANALYZE;

-- Reindex
REINDEX DATABASE deverp;

-- Check table sizes
SELECT
    schemaname,
    tablename,
    pg_size_pretty(pg_total_relation_size(schemaname||'.'||tablename)) AS size
FROM pg_tables
WHERE schemaname = 'public'
ORDER BY pg_total_relation_size(schemaname||'.'||tablename) DESC;
```

---

## 10. Security Considerations

### 10.1 User Roles and Permissions
```sql
-- Create application user
CREATE USER deverp_user WITH PASSWORD 'secure_password';

-- Grant necessary permissions
GRANT CONNECT ON DATABASE deverp TO deverp_user;
GRANT USAGE ON SCHEMA public TO deverp_user;
GRANT SELECT, INSERT, UPDATE, DELETE ON ALL TABLES IN SCHEMA public TO deverp_user;
GRANT USAGE, SELECT ON ALL SEQUENCES IN SCHEMA public TO deverp_user;

-- Read-only user for reports
CREATE USER deverp_readonly WITH PASSWORD 'readonly_password';
GRANT CONNECT ON DATABASE deverp TO deverp_readonly;
GRANT USAGE ON SCHEMA public TO deverp_readonly;
GRANT SELECT ON ALL TABLES IN SCHEMA public TO deverp_readonly;
```

### 10.2 Data Protection
- Use SSL/TLS for database connections
- Store sensitive configuration values encrypted
- Implement row-level security if multiple users
- Regular security audits of permissions

---

## 11. Migration Strategy

### 11.1 Migration Tools
- Use `sqlx-cli` or `refinery` for schema migrations
- Version control all migration scripts
- Test migrations on development database first

### 11.2 Migration Workflow
```bash
# Create new migration
sqlx migrate add migration_name

# Run migrations
sqlx migrate run

# Rollback last migration
sqlx migrate revert
```

---

## Appendix

### A. Data Dictionary

| Table | Purpose | Records (Estimated) |
|-------|---------|---------------------|
| projects | Store project information | 10-100 |
| tasks | Store task information | 100-1000 |
| task_dependencies | Task relationships | 50-500 |
| task_comments | Task notes and comments | 200-2000 |
| resources | Development resources | 50-200 |
| project_resources | Project-resource links | 100-500 |
| timelines | Project timelines | 10-100 |
| milestones | Timeline milestones | 50-500 |
| tags | Reusable tags | 20-100 |
| project_tags | Project-tag links | 50-500 |
| configurations | System config | 10-50 |
| audit_logs | System audit trail | 1000+ |

### B. Status Values Reference

#### Project Status
- `planning`: Project in planning phase
- `active`: Project actively being worked on
- `on_hold`: Project temporarily paused
- `completed`: Project successfully completed
- `archived`: Project archived for reference
- `cancelled`: Project cancelled

#### Task Status
- `todo`: Task not started
- `in_progress`: Task being worked on
- `blocked`: Task blocked by dependencies
- `review`: Task under review
- `testing`: Task being tested
- `done`: Task completed
- `cancelled`: Task cancelled

### C. Version History
- v1.0 (2025-10-18): Initial database design

---

**Document Status**: Draft
**Last Updated**: 2025-10-18
**Next Review**: TBD
