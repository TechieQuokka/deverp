// Report Service - Business logic for generating analytics and reports

use std::sync::Arc;
use serde::{Serialize, Deserialize};
use chrono::{DateTime, Utc};

use crate::utils::error::DevErpError;
use crate::domain::project::repository::ProjectRepository;
use crate::domain::project::entity::{ProjectFilter, ProjectStatus, Priority};
use crate::domain::task::repository::TaskRepository;
use crate::domain::task::entity::{TaskFilter, TaskStatus, TaskPriority};
use crate::domain::resource::repository::ResourceRepository;
use crate::domain::resource::entity::{ResourceFilter, ResourceType, ResourceStatus};
use crate::domain::timeline::repository::{TimelineRepository, MilestoneRepository};
use crate::domain::timeline::entity::{TimelineFilter, TimelineStatus, MilestoneStatus};

/// Project Status Report - Overall project statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectStatusReport {
    /// Total number of projects
    pub total_projects: i64,
    /// Number of active projects
    pub active_projects: i64,
    /// Number of completed projects
    pub completed_projects: i64,
    /// Number of projects on hold
    pub on_hold_projects: i64,
    /// Number of cancelled projects
    pub cancelled_projects: i64,
    /// Number of archived projects
    pub archived_projects: i64,
    /// Projects by priority
    pub projects_by_priority: PriorityDistribution,
    /// Average project progress
    pub average_progress: f64,
    /// Projects with delays (actual > planned)
    pub delayed_projects: i64,
    /// Report generation timestamp
    pub generated_at: DateTime<Utc>,
}

/// Priority Distribution
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PriorityDistribution {
    pub critical: i64,
    pub high: i64,
    pub medium: i64,
    pub low: i64,
}

/// Task Analytics Report - Task completion and progress statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaskAnalyticsReport {
    /// Total number of tasks
    pub total_tasks: i64,
    /// Tasks by status
    pub tasks_by_status: TaskStatusDistribution,
    /// Tasks by priority
    pub tasks_by_priority: PriorityDistribution,
    /// Completion rate (percentage)
    pub completion_rate: f64,
    /// Average estimated hours
    pub avg_estimated_hours: f64,
    /// Average actual hours
    pub avg_actual_hours: f64,
    /// Total estimated hours
    pub total_estimated_hours: f64,
    /// Total actual hours
    pub total_actual_hours: f64,
    /// Variance (actual - estimated) percentage
    pub time_variance_percentage: f64,
    /// Overdue tasks count
    pub overdue_tasks: i64,
    /// Tasks completed on time
    pub on_time_completion_count: i64,
    /// Report generation timestamp
    pub generated_at: DateTime<Utc>,
}

/// Task Status Distribution
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaskStatusDistribution {
    pub todo: i64,
    pub in_progress: i64,
    pub blocked: i64,
    pub review: i64,
    pub testing: i64,
    pub done: i64,
    pub cancelled: i64,
}

/// Resource Usage Report - Resource utilization statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResourceUsageReport {
    /// Total number of resources
    pub total_resources: i64,
    /// Active resources
    pub active_resources: i64,
    /// Deprecated resources
    pub deprecated_resources: i64,
    /// Resources by type
    pub resources_by_type: ResourceTypeDistribution,
    /// Most used resources (top 10)
    pub most_used_resources: Vec<ResourceUsageItem>,
    /// Unused resources
    pub unused_resources: i64,
    /// Report generation timestamp
    pub generated_at: DateTime<Utc>,
}

/// Resource Type Distribution
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResourceTypeDistribution {
    pub library: i64,
    pub api: i64,
    pub tool: i64,
    pub service: i64,
    pub documentation: i64,
    pub other: i64,
}

/// Resource Usage Item
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResourceUsageItem {
    pub resource_id: i64,
    pub resource_name: String,
    pub resource_type: String,
    pub project_count: i64,
    pub critical_project_count: i64,
}

/// Timeline Progress Report - Timeline and milestone tracking
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TimelineProgressReport {
    /// Total number of timelines
    pub total_timelines: i64,
    /// Active timelines
    pub active_timelines: i64,
    /// Completed timelines
    pub completed_timelines: i64,
    /// Total milestones
    pub total_milestones: i64,
    /// Completed milestones
    pub completed_milestones: i64,
    /// Missed milestones
    pub missed_milestones: i64,
    /// Milestone completion rate (percentage)
    pub milestone_completion_rate: f64,
    /// On-time milestone completion rate
    pub on_time_milestone_rate: f64,
    /// Upcoming milestones (next 30 days)
    pub upcoming_milestones_count: i64,
    /// Report generation timestamp
    pub generated_at: DateTime<Utc>,
}

/// Project Summary Item
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectSummaryItem {
    pub project_id: i64,
    pub project_name: String,
    pub project_code: Option<String>,
    pub status: String,
    pub priority: String,
    pub progress_percentage: i32,
    pub total_tasks: i64,
    pub completed_tasks: i64,
    pub start_date: Option<String>,
    pub end_date: Option<String>,
}

/// Report Service
pub struct ReportService {
    project_repo: Arc<dyn ProjectRepository>,
    task_repo: Arc<dyn TaskRepository>,
    resource_repo: Arc<dyn ResourceRepository>,
    timeline_repo: Arc<dyn TimelineRepository>,
    milestone_repo: Arc<dyn MilestoneRepository>,
}

impl ReportService {
    /// Create a new ReportService
    pub fn new(
        project_repo: Arc<dyn ProjectRepository>,
        task_repo: Arc<dyn TaskRepository>,
        resource_repo: Arc<dyn ResourceRepository>,
        timeline_repo: Arc<dyn TimelineRepository>,
        milestone_repo: Arc<dyn MilestoneRepository>,
    ) -> Self {
        Self {
            project_repo,
            task_repo,
            resource_repo,
            timeline_repo,
            milestone_repo,
        }
    }

    /// Generate overall project status report
    pub async fn generate_project_status_report(&self) -> Result<ProjectStatusReport, DevErpError> {
        // Count total projects
        let total_filter = ProjectFilter::default();
        let total_projects = self.project_repo.count(total_filter).await?;

        // Count projects by status
        let active_projects = self.project_repo.count(ProjectFilter {
            status: Some(ProjectStatus::Active),
            ..Default::default()
        }).await?;

        let completed_projects = self.project_repo.count(ProjectFilter {
            status: Some(ProjectStatus::Completed),
            ..Default::default()
        }).await?;

        let on_hold_projects = self.project_repo.count(ProjectFilter {
            status: Some(ProjectStatus::OnHold),
            ..Default::default()
        }).await?;

        let cancelled_projects = self.project_repo.count(ProjectFilter {
            status: Some(ProjectStatus::Cancelled),
            ..Default::default()
        }).await?;

        let archived_projects = self.project_repo.count(ProjectFilter {
            status: Some(ProjectStatus::Archived),
            ..Default::default()
        }).await?;

        // Count projects by priority
        let critical_priority = self.project_repo.count(ProjectFilter {
            priority: Some(Priority::Critical),
            ..Default::default()
        }).await?;

        let high_priority = self.project_repo.count(ProjectFilter {
            priority: Some(Priority::High),
            ..Default::default()
        }).await?;

        let medium_priority = self.project_repo.count(ProjectFilter {
            priority: Some(Priority::Medium),
            ..Default::default()
        }).await?;

        let low_priority = self.project_repo.count(ProjectFilter {
            priority: Some(Priority::Low),
            ..Default::default()
        }).await?;

        // Calculate average progress
        let all_projects = self.project_repo.find_all(ProjectFilter::default()).await?;
        let average_progress = if !all_projects.is_empty() {
            let total_progress: i32 = all_projects.iter()
                .map(|p| p.progress_percentage.unwrap_or(0))
                .sum();
            total_progress as f64 / all_projects.len() as f64
        } else {
            0.0
        };

        // Count delayed projects (where actual_end_date > end_date or current date > end_date for active projects)
        let now = Utc::now().naive_utc().date();
        let delayed_projects = all_projects.iter().filter(|p| {
            if let Some(end_date) = p.end_date {
                if let Some(actual_end_date) = p.actual_end_date {
                    actual_end_date > end_date
                } else if p.status == ProjectStatus::Active {
                    now > end_date
                } else {
                    false
                }
            } else {
                false
            }
        }).count() as i64;

        Ok(ProjectStatusReport {
            total_projects,
            active_projects,
            completed_projects,
            on_hold_projects,
            cancelled_projects,
            archived_projects,
            projects_by_priority: PriorityDistribution {
                critical: critical_priority,
                high: high_priority,
                medium: medium_priority,
                low: low_priority,
            },
            average_progress,
            delayed_projects,
            generated_at: Utc::now(),
        })
    }

    /// Generate task analytics report
    pub async fn generate_task_analytics(&self) -> Result<TaskAnalyticsReport, DevErpError> {
        // Count total tasks
        let total_filter = TaskFilter::default();
        let total_tasks = self.task_repo.count(total_filter).await?;

        // Count tasks by status
        let todo_tasks = self.task_repo.count(TaskFilter {
            status: Some(TaskStatus::Todo),
            ..Default::default()
        }).await?;

        let in_progress_tasks = self.task_repo.count(TaskFilter {
            status: Some(TaskStatus::InProgress),
            ..Default::default()
        }).await?;

        let blocked_tasks = self.task_repo.count(TaskFilter {
            status: Some(TaskStatus::Blocked),
            ..Default::default()
        }).await?;

        let review_tasks = self.task_repo.count(TaskFilter {
            status: Some(TaskStatus::Review),
            ..Default::default()
        }).await?;

        let testing_tasks = self.task_repo.count(TaskFilter {
            status: Some(TaskStatus::Testing),
            ..Default::default()
        }).await?;

        let done_tasks = self.task_repo.count(TaskFilter {
            status: Some(TaskStatus::Done),
            ..Default::default()
        }).await?;

        let cancelled_tasks = self.task_repo.count(TaskFilter {
            status: Some(TaskStatus::Cancelled),
            ..Default::default()
        }).await?;

        // Count tasks by priority
        let critical_priority = self.task_repo.count(TaskFilter {
            priority: Some(TaskPriority::Critical),
            ..Default::default()
        }).await?;

        let high_priority = self.task_repo.count(TaskFilter {
            priority: Some(TaskPriority::High),
            ..Default::default()
        }).await?;

        let medium_priority = self.task_repo.count(TaskFilter {
            priority: Some(TaskPriority::Medium),
            ..Default::default()
        }).await?;

        let low_priority = self.task_repo.count(TaskFilter {
            priority: Some(TaskPriority::Low),
            ..Default::default()
        }).await?;

        // Calculate completion rate
        let completion_rate = if total_tasks > 0 {
            (done_tasks as f64 / total_tasks as f64) * 100.0
        } else {
            0.0
        };

        // Get all tasks for time calculations
        let all_tasks = self.task_repo.find_all(TaskFilter::default()).await?;

        // Calculate time statistics
        let tasks_with_estimated: Vec<_> = all_tasks.iter()
            .filter(|t| t.estimated_hours.is_some())
            .collect();

        let tasks_with_actual: Vec<_> = all_tasks.iter()
            .filter(|t| t.actual_hours.is_some())
            .collect();

        let total_estimated_hours: f64 = tasks_with_estimated.iter()
            .filter_map(|t| t.estimated_hours)
            .sum();

        let total_actual_hours: f64 = tasks_with_actual.iter()
            .filter_map(|t| t.actual_hours)
            .sum();

        let avg_estimated_hours = if !tasks_with_estimated.is_empty() {
            total_estimated_hours / tasks_with_estimated.len() as f64
        } else {
            0.0
        };

        let avg_actual_hours = if !tasks_with_actual.is_empty() {
            total_actual_hours / tasks_with_actual.len() as f64
        } else {
            0.0
        };

        // Calculate time variance
        let time_variance_percentage = if total_estimated_hours > 0.0 {
            ((total_actual_hours - total_estimated_hours) / total_estimated_hours) * 100.0
        } else {
            0.0
        };

        // Count overdue tasks
        let now = Utc::now();
        let overdue_tasks = all_tasks.iter()
            .filter(|t| {
                if let Some(due_date) = t.due_date {
                    t.status != TaskStatus::Done && t.status != TaskStatus::Cancelled && due_date < now
                } else {
                    false
                }
            })
            .count() as i64;

        // Count tasks completed on time
        let on_time_completion_count = all_tasks.iter()
            .filter(|t| {
                if t.status == TaskStatus::Done {
                    if let (Some(due_date), Some(completed_at)) = (t.due_date, t.completed_at) {
                        completed_at <= due_date
                    } else {
                        false
                    }
                } else {
                    false
                }
            })
            .count() as i64;

        Ok(TaskAnalyticsReport {
            total_tasks,
            tasks_by_status: TaskStatusDistribution {
                todo: todo_tasks,
                in_progress: in_progress_tasks,
                blocked: blocked_tasks,
                review: review_tasks,
                testing: testing_tasks,
                done: done_tasks,
                cancelled: cancelled_tasks,
            },
            tasks_by_priority: PriorityDistribution {
                critical: critical_priority,
                high: high_priority,
                medium: medium_priority,
                low: low_priority,
            },
            completion_rate,
            avg_estimated_hours,
            avg_actual_hours,
            total_estimated_hours,
            total_actual_hours,
            time_variance_percentage,
            overdue_tasks,
            on_time_completion_count,
            generated_at: Utc::now(),
        })
    }

    /// Generate resource usage report
    pub async fn generate_resource_usage_report(&self) -> Result<ResourceUsageReport, DevErpError> {
        // Get all resources
        let all_resources = self.resource_repo.find_all(ResourceFilter::default()).await?;
        let total_resources = all_resources.len() as i64;

        // Count active and deprecated resources
        let active_resources = all_resources.iter()
            .filter(|r| r.status.as_ref().map_or(false, |s| matches!(s, ResourceStatus::Active)))
            .count() as i64;

        let deprecated_resources = all_resources.iter()
            .filter(|r| r.status.as_ref().map_or(false, |s| matches!(s, ResourceStatus::Deprecated)))
            .count() as i64;

        // Count resources by type
        let library_count = all_resources.iter()
            .filter(|r| r.resource_type == ResourceType::Library)
            .count() as i64;

        let api_count = all_resources.iter()
            .filter(|r| r.resource_type == ResourceType::Api)
            .count() as i64;

        let tool_count = all_resources.iter()
            .filter(|r| r.resource_type == ResourceType::Tool)
            .count() as i64;

        let service_count = all_resources.iter()
            .filter(|r| r.resource_type == ResourceType::Service)
            .count() as i64;

        let documentation_count = all_resources.iter()
            .filter(|r| r.resource_type == ResourceType::Documentation)
            .count() as i64;

        let other_count = all_resources.iter()
            .filter(|r| r.resource_type == ResourceType::Other)
            .count() as i64;

        // Get resource usage statistics
        let mut resource_usage_items = Vec::new();
        for resource in &all_resources {
            let usage = self.resource_repo.get_usage_stats(resource.id).await?;
            resource_usage_items.push(ResourceUsageItem {
                resource_id: resource.id,
                resource_name: resource.name.clone(),
                resource_type: resource.resource_type.to_string(),
                project_count: usage.total_projects,
                critical_project_count: usage.critical_projects,
            });
        }

        // Sort by project count and take top 10
        resource_usage_items.sort_by(|a, b| b.project_count.cmp(&a.project_count));
        let most_used_resources = resource_usage_items.iter().take(10).cloned().collect();

        // Count unused resources
        let unused_resources = resource_usage_items.iter()
            .filter(|r| r.project_count == 0)
            .count() as i64;

        Ok(ResourceUsageReport {
            total_resources,
            active_resources,
            deprecated_resources,
            resources_by_type: ResourceTypeDistribution {
                library: library_count,
                api: api_count,
                tool: tool_count,
                service: service_count,
                documentation: documentation_count,
                other: other_count,
            },
            most_used_resources,
            unused_resources,
            generated_at: Utc::now(),
        })
    }

    /// Generate timeline progress report
    pub async fn generate_timeline_progress_report(&self) -> Result<TimelineProgressReport, DevErpError> {
        // Get all timelines
        let all_timelines = self.timeline_repo.find_all(TimelineFilter::default()).await?;
        let total_timelines = all_timelines.len() as i64;

        // Count timelines by status
        let active_timelines = all_timelines.iter()
            .filter(|t| matches!(t.status, TimelineStatus::Active))
            .count() as i64;

        let completed_timelines = all_timelines.iter()
            .filter(|t| matches!(t.status, TimelineStatus::Completed))
            .count() as i64;

        // Get all milestones
        let mut all_milestones = Vec::new();
        for timeline in &all_timelines {
            let milestones = self.milestone_repo.find_by_timeline(timeline.id).await?;
            all_milestones.extend(milestones);
        }

        let total_milestones = all_milestones.len() as i64;

        // Count milestones by status
        let completed_milestones = all_milestones.iter()
            .filter(|m| matches!(m.status, MilestoneStatus::Completed))
            .count() as i64;

        let missed_milestones = all_milestones.iter()
            .filter(|m| matches!(m.status, MilestoneStatus::Missed))
            .count() as i64;

        // Calculate milestone completion rate
        let milestone_completion_rate = if total_milestones > 0 {
            (completed_milestones as f64 / total_milestones as f64) * 100.0
        } else {
            0.0
        };

        // Calculate on-time milestone completion rate
        let on_time_completions = all_milestones.iter()
            .filter(|m| {
                if matches!(m.status, MilestoneStatus::Completed) {
                    if let Some(actual_date) = m.actual_date {
                        actual_date <= m.target_date
                    } else {
                        false
                    }
                } else {
                    false
                }
            })
            .count() as i64;

        let on_time_milestone_rate = if completed_milestones > 0 {
            (on_time_completions as f64 / completed_milestones as f64) * 100.0
        } else {
            0.0
        };

        // Count upcoming milestones (next 30 days)
        let now = Utc::now().naive_utc().date();
        let thirty_days_later = now + chrono::Duration::days(30);
        let upcoming_milestones_count = all_milestones.iter()
            .filter(|m| {
                matches!(m.status, MilestoneStatus::Pending | MilestoneStatus::InProgress) &&
                m.target_date >= now &&
                m.target_date <= thirty_days_later
            })
            .count() as i64;

        Ok(TimelineProgressReport {
            total_timelines,
            active_timelines,
            completed_timelines,
            total_milestones,
            completed_milestones,
            missed_milestones,
            milestone_completion_rate,
            on_time_milestone_rate,
            upcoming_milestones_count,
            generated_at: Utc::now(),
        })
    }

    /// Generate project summary report
    pub async fn generate_project_summary(&self) -> Result<Vec<ProjectSummaryItem>, DevErpError> {
        let projects = self.project_repo.find_all(ProjectFilter::default()).await?;

        let mut summary_items = Vec::new();
        for project in projects {
            // Count tasks for this project
            let total_tasks = self.task_repo.count(TaskFilter {
                project_id: Some(project.id),
                ..Default::default()
            }).await?;

            let completed_tasks = self.task_repo.count(TaskFilter {
                project_id: Some(project.id),
                status: Some(TaskStatus::Done),
                ..Default::default()
            }).await?;

            summary_items.push(ProjectSummaryItem {
                project_id: project.id,
                project_name: project.name,
                project_code: project.code,
                status: project.status.to_string(),
                priority: project.priority.to_string(),
                progress_percentage: project.progress_percentage.unwrap_or(0),
                total_tasks,
                completed_tasks,
                start_date: project.start_date.map(|d| d.to_string()),
                end_date: project.end_date.map(|d| d.to_string()),
            });
        }

        Ok(summary_items)
    }
}
