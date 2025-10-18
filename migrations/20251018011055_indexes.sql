-- Indexes for Performance Optimization
-- Phase 3: Database Schema Implementation

-- ====================================================================
-- PROJECTS TABLE INDEXES
-- ====================================================================
CREATE INDEX idx_projects_status ON projects(status) WHERE deleted_at IS NULL;
CREATE INDEX idx_projects_priority ON projects(priority) WHERE deleted_at IS NULL;
CREATE INDEX idx_projects_dates ON projects(start_date, end_date) WHERE deleted_at IS NULL;
CREATE INDEX idx_projects_code ON projects(code) WHERE deleted_at IS NULL;
CREATE INDEX idx_projects_tags ON projects USING gin(tags);
CREATE INDEX idx_projects_metadata ON projects USING gin(metadata);

-- ====================================================================
-- TASKS TABLE INDEXES
-- ====================================================================
CREATE INDEX idx_tasks_project_id ON tasks(project_id) WHERE deleted_at IS NULL;
CREATE INDEX idx_tasks_status ON tasks(status) WHERE deleted_at IS NULL;
CREATE INDEX idx_tasks_priority ON tasks(priority) WHERE deleted_at IS NULL;
CREATE INDEX idx_tasks_assigned_to ON tasks(assigned_to) WHERE deleted_at IS NULL;
CREATE INDEX idx_tasks_due_date ON tasks(due_date) WHERE deleted_at IS NULL;
CREATE INDEX idx_tasks_parent_task_id ON tasks(parent_task_id) WHERE deleted_at IS NULL;
CREATE INDEX idx_tasks_tags ON tasks USING gin(tags);

-- ====================================================================
-- TASK_DEPENDENCIES TABLE INDEXES
-- ====================================================================
CREATE INDEX idx_task_deps_task_id ON task_dependencies(task_id);
CREATE INDEX idx_task_deps_depends_on ON task_dependencies(depends_on_task_id);

-- ====================================================================
-- TASK_COMMENTS TABLE INDEXES
-- ====================================================================
CREATE INDEX idx_task_comments_task_id ON task_comments(task_id) WHERE deleted_at IS NULL;
CREATE INDEX idx_task_comments_created_at ON task_comments(created_at DESC);

-- ====================================================================
-- RESOURCES TABLE INDEXES
-- ====================================================================
CREATE INDEX idx_resources_type ON resources(resource_type) WHERE deleted_at IS NULL;
CREATE INDEX idx_resources_status ON resources(status) WHERE deleted_at IS NULL;
CREATE INDEX idx_resources_name ON resources(name) WHERE deleted_at IS NULL;

-- ====================================================================
-- PROJECT_RESOURCES TABLE INDEXES
-- ====================================================================
CREATE INDEX idx_project_resources_project_id ON project_resources(project_id);
CREATE INDEX idx_project_resources_resource_id ON project_resources(resource_id);

-- ====================================================================
-- TIMELINES TABLE INDEXES
-- ====================================================================
CREATE INDEX idx_timelines_project_id ON timelines(project_id) WHERE deleted_at IS NULL;
CREATE INDEX idx_timelines_dates ON timelines(start_date, end_date);

-- ====================================================================
-- MILESTONES TABLE INDEXES
-- ====================================================================
CREATE INDEX idx_milestones_timeline_id ON milestones(timeline_id) WHERE deleted_at IS NULL;
CREATE INDEX idx_milestones_project_id ON milestones(project_id) WHERE deleted_at IS NULL;
CREATE INDEX idx_milestones_target_date ON milestones(target_date);
CREATE INDEX idx_milestones_status ON milestones(status);

-- ====================================================================
-- TAGS TABLE INDEXES
-- ====================================================================
CREATE INDEX idx_tags_name ON tags(name);
CREATE INDEX idx_tags_type ON tags(tag_type);

-- ====================================================================
-- PROJECT_TAGS TABLE INDEXES
-- ====================================================================
CREATE INDEX idx_project_tags_project_id ON project_tags(project_id);
CREATE INDEX idx_project_tags_tag_id ON project_tags(tag_id);

-- ====================================================================
-- CONFIGURATIONS TABLE INDEXES
-- ====================================================================
CREATE INDEX idx_configurations_key ON configurations(config_key);

-- ====================================================================
-- AUDIT_LOGS TABLE INDEXES
-- ====================================================================
CREATE INDEX idx_audit_logs_entity ON audit_logs(entity_type, entity_id);
CREATE INDEX idx_audit_logs_action ON audit_logs(action);
CREATE INDEX idx_audit_logs_created_at ON audit_logs(created_at DESC);
CREATE INDEX idx_audit_logs_user ON audit_logs(user_identifier)
