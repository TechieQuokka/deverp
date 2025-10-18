-- Database Views for Reporting and Analytics
-- Phase 3: Database Schema Implementation

-- ====================================================================
-- ACTIVE PROJECTS VIEW
-- ====================================================================
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

-- ====================================================================
-- TASK SUMMARY VIEW
-- ====================================================================
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

-- ====================================================================
-- RESOURCE USAGE VIEW
-- ====================================================================
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

-- ====================================================================
-- PROJECT TIMELINE VIEW
-- ====================================================================
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
GROUP BY p.id, p.name, t.id, t.name, t.timeline_type, t.start_date, t.end_date, t.status
