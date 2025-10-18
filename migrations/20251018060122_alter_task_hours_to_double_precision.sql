-- Convert NUMERIC columns to DOUBLE PRECISION for task hours
-- This allows better Rust type compatibility with f64

-- Drop dependent view
DROP VIEW IF EXISTS v_task_summary;

-- Alter column types
ALTER TABLE tasks
ALTER COLUMN estimated_hours TYPE DOUBLE PRECISION,
ALTER COLUMN actual_hours TYPE DOUBLE PRECISION;

-- Recreate view
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
