// Task domain module

pub mod entity;
pub mod repository;
pub mod service;

// Re-export commonly used types
pub use entity::{
    CreateTask, CreateTaskComment, CreateTaskDependency, DependencyType, Task, TaskComment,
    TaskDependency, TaskFilter, TaskPriority, TaskStatus, TaskType, UpdateTask,
};
pub use repository::{TaskCommentRepository, TaskDependencyRepository, TaskRepository};
pub use service::TaskService;
