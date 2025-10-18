-- Initial Seed Data (Configurations and Tags)
-- Phase 3: Database Schema Implementation

-- ====================================================================
-- DEFAULT CONFIGURATIONS
-- ====================================================================
INSERT INTO configurations (config_key, config_value, description, data_type) VALUES
('default_project_status', 'planning', 'Default status for new projects', 'string'),
('default_task_status', 'todo', 'Default status for new tasks', 'string'),
('date_format', '%Y-%m-%d', 'Default date format', 'string'),
('enable_audit_log', 'true', 'Enable audit logging', 'boolean'),
('max_task_dependencies', '50', 'Maximum number of dependencies per task', 'integer'),
('default_priority', 'medium', 'Default priority level', 'string');

-- ====================================================================
-- DEFAULT TAGS
-- ====================================================================
INSERT INTO tags (name, description, tag_type, color) VALUES
-- Technology Tags
('rust', 'Rust programming language', 'technology', '#CE412B'),
('postgresql', 'PostgreSQL database', 'technology', '#336791'),
('python', 'Python programming language', 'technology', '#3776AB'),
('javascript', 'JavaScript programming language', 'technology', '#F7DF1E'),
('typescript', 'TypeScript programming language', 'technology', '#3178C6'),
('react', 'React framework', 'technology', '#61DAFB'),
('vue', 'Vue.js framework', 'technology', '#4FC08D'),
('docker', 'Docker containerization', 'technology', '#2496ED'),
('kubernetes', 'Kubernetes orchestration', 'technology', '#326CE5'),

-- Category Tags
('cli', 'Command-line interface', 'category', '#000000'),
('backend', 'Backend development', 'category', '#3178C6'),
('frontend', 'Frontend development', 'category', '#61DAFB'),
('api', 'API development', 'category', '#FF6C37'),
('database', 'Database work', 'category', '#336791'),
('devops', 'DevOps and infrastructure', 'category', '#2496ED'),
('testing', 'Testing and QA', 'category', '#A9225C'),
('documentation', 'Documentation', 'category', '#0075CA'),
('security', 'Security related', 'category', '#D73A4A'),

-- Status Tags
('bug', 'Bug fix', 'status', '#D73A4A'),
('feature', 'New feature', 'status', '#0E8A16'),
('enhancement', 'Enhancement to existing feature', 'status', '#0052CC'),
('refactor', 'Code refactoring', 'status', '#FBCA04'),
('hotfix', 'Critical hotfix', 'status', '#B60205'),

-- Priority Tags
('critical', 'Critical priority', 'priority', '#B60205'),
('high', 'High priority', 'priority', '#D73A4A'),
('medium', 'Medium priority', 'priority', '#FBCA04'),
('low', 'Low priority', 'priority', '#0E8A16'),

-- General Tags
('poc', 'Proof of concept', 'general', '#BFD4F2'),
('mvp', 'Minimum viable product', 'general', '#C5DEF5'),
('production', 'Production ready', 'general', '#0E8A16'),
('experimental', 'Experimental feature', 'general', '#FBCA04'),
('deprecated', 'Deprecated feature', 'general', '#6A737D')
