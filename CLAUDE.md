# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

DevERP is a CLI-based development project management ERP system built with Rust and PostgreSQL. It's designed for individual developers to manage multiple projects, tasks, resources, and timelines from the command line.

**Technology Stack**: Rust + PostgreSQL (local deployment only)

## Architecture

### Layered Architecture

The codebase follows a strict layered architecture:

```
CLI Layer (src/cli/)
    ↓
Business Logic Layer (src/domain/)
    ↓
Data Access Layer (src/infrastructure/)
    ↓
PostgreSQL Database
```

**Critical Rule**: Dependencies flow downward only. CLI depends on domain, domain depends on infrastructure, but never the reverse.

### Repository Pattern

All database access must go through repository traits defined in `src/domain/*/repository.rs` and implemented in `src/infrastructure/repositories/*_repo.rs`. Never write direct SQL queries in service or CLI layers.

Example:
```rust
// ✅ Correct: Service uses repository trait
pub struct ProjectService {
    repository: Arc<dyn ProjectRepository>,
}

// ❌ Wrong: Service has direct database access
pub struct ProjectService {
    pool: PgPool,
}
```

### Domain-Driven Structure

Each domain module (project, task, resource, timeline) follows this pattern:
- `entity.rs`: Data structures, enums, validation
- `service.rs`: Business logic, orchestration
- `repository.rs`: Data access trait definition

## Building and Running

### Prerequisites
```bash
# Ensure PostgreSQL is running locally
# Database should be named 'deverp'

# Set DATABASE_URL environment variable
export DATABASE_URL="postgres://deverp_user:password@localhost/deverp"
```

### Common Commands
```bash
# Build (development)
cargo build

# Build (release)
cargo build --release

# Run tests
cargo test

# Run with debug logging
RUST_LOG=debug cargo run -- <command>

# Run specific command
cargo run -- project list
cargo run -- task create --project-id 1 --title "My Task"
```

### Database Migrations
```bash
# Install sqlx-cli if not already installed
cargo install sqlx-cli --features postgres

# Run migrations
sqlx migrate run

# Create new migration
sqlx migrate add <migration_name>

# Revert last migration
sqlx migrate revert
```

## Code Conventions

### Error Handling

All functions that can fail return `Result<T, DevErpError>`. Use the custom error types defined in `src/utils/error.rs`:

```rust
// ✅ Correct
pub async fn create_project(&self, input: CreateProject) -> Result<Project, DevErpError> {
    // Use ? operator for error propagation
    let project = self.repository.create(input).await?;
    Ok(project)
}

// ❌ Wrong: Don't use unwrap() or expect() except in tests
let project = self.repository.create(input).await.unwrap();
```

### Async/Await

Use `async/await` for all I/O operations. The runtime is tokio.

```rust
// ✅ Correct: Repository methods are async
#[async_trait]
pub trait ProjectRepository {
    async fn create(&self, project: CreateProject) -> Result<Project, DevErpError>;
}

// Service methods calling repository are also async
pub async fn create_project(&self, input: CreateProject) -> Result<Project, DevErpError> {
    self.repository.create(input).await
}
```

### CLI Command Structure

All CLI commands follow this pattern:
```
deverp <entity> <action> [options] [arguments]

Examples:
  deverp project create --name "My Project"
  deverp task list --project-id 1 --status active
  deverp resource link --project-id 1 --resource-id 5
```

Each entity has its own CLI module in `src/cli/<entity>.rs` with subcommands defined using clap's derive API.

### Database Queries

Use sqlx's compile-time checked queries:

```rust
// ✅ Correct: Type-safe query with sqlx::query_as!
let project = sqlx::query_as!(
    Project,
    r#"
    SELECT id, uuid, name, description, status as "status: ProjectStatus"
    FROM projects
    WHERE id = $1 AND deleted_at IS NULL
    "#,
    id
)
.fetch_optional(&self.pool)
.await?;

// ❌ Wrong: String queries without type checking
let query = format!("SELECT * FROM projects WHERE id = {}", id);
```

### Soft Deletes

All main entities use soft deletes. Never hard delete records unless explicitly required:

```rust
// ✅ Correct: Soft delete by setting deleted_at
sqlx::query!(
    "UPDATE projects SET deleted_at = NOW() WHERE id = $1",
    id
)
.execute(&self.pool)
.await?;

// ❌ Wrong: Hard delete
sqlx::query!("DELETE FROM projects WHERE id = $1", id)
.execute(&self.pool)
.await?;
```

Always include `WHERE deleted_at IS NULL` in SELECT queries unless you explicitly want deleted records.

## Testing

### Unit Tests

Test business logic in service layer by mocking repositories using `mockall`:

```rust
#[cfg(test)]
mod tests {
    use super::*;
    use mockall::predicate::*;

    #[tokio::test]
    async fn test_create_project() {
        let mut mock_repo = MockProjectRepository::new();
        mock_repo
            .expect_create()
            .returning(|input| Ok(Project { /* ... */ }));

        let service = ProjectService::new(Arc::new(mock_repo));
        let result = service.create_project(input).await;
        assert!(result.is_ok());
    }
}
```

### Integration Tests

Test repository implementations using `testcontainers` for PostgreSQL:

```rust
#[tokio::test]
async fn test_project_repository_create() {
    // Testcontainer setup
    let container = PostgresContainer::default().start().await;
    let pool = create_test_pool(&container).await;

    // Run migrations
    sqlx::migrate!().run(&pool).await.unwrap();

    // Test repository
    let repo = PostgresProjectRepository::new(pool);
    let project = repo.create(CreateProject { /* ... */ }).await.unwrap();

    assert_eq!(project.name, "Test Project");
}
```

### Test Organization

- Unit tests: In same file as implementation using `#[cfg(test)]`
- Integration tests: In `tests/` directory
- Test helpers: In `tests/helpers/`

## Database Schema

### Key Tables

- **projects**: Main project information
- **tasks**: Tasks within projects (1:N with projects)
- **task_dependencies**: Task dependency graph (M:N self-referencing)
- **resources**: Development resources (libraries, APIs, tools)
- **project_resources**: Links projects to resources (M:N)
- **timelines**: Project timelines (1:N with projects)
- **milestones**: Timeline milestones (1:N with timelines)
- **tags**: Tag master table
- **project_tags**: Links projects to tags (M:N)

### UUID vs ID

Each entity has both:
- `id`: BIGSERIAL primary key for internal use
- `uuid`: UUID for external references and API exposure

Always use `id` for foreign keys and internal queries, `uuid` for CLI output and external references.

### Status Enums

Status values are stored as VARCHAR but mapped to Rust enums:

```rust
#[derive(Debug, sqlx::Type)]
#[sqlx(type_name = "VARCHAR", rename_all = "lowercase")]
pub enum ProjectStatus {
    Planning,
    Active,
    OnHold,
    Completed,
    Archived,
    Cancelled,
}
```

This allows type-safe status handling in Rust while keeping database schema simple.

## Common Patterns

### Dependency Injection in Services

```rust
pub struct ProjectService {
    repository: Arc<dyn ProjectRepository>,
}

impl ProjectService {
    pub fn new(repository: Arc<dyn ProjectRepository>) -> Self {
        Self { repository }
    }
}
```

### CLI to Service Flow

```rust
// In src/cli/project.rs
pub async fn handle_create(args: CreateArgs, pool: PgPool) -> Result<(), DevErpError> {
    // Create repository
    let repo = PostgresProjectRepository::new(pool);

    // Create service with repository
    let service = ProjectService::new(Arc::new(repo));

    // Call service method
    let project = service.create_project(args.into()).await?;

    // Format and display output
    println!("Created project: {}", project.name);
    Ok(())
}
```

### Transaction Handling

Use sqlx transactions for operations that modify multiple tables:

```rust
let mut tx = self.pool.begin().await?;

// Multiple operations
sqlx::query!("INSERT INTO projects ...").execute(&mut *tx).await?;
sqlx::query!("INSERT INTO project_tags ...").execute(&mut *tx).await?;

tx.commit().await?;
```

## Documentation

- **Architecture**: `docs/architecture.md` - Overall system design
- **Database**: `docs/database.md` - Complete schema and query examples
- **Implementation Plan**: `docs/implementation-plan.md` - Detailed development phases

When adding new features, update relevant documentation files.

## Performance Considerations

### Connection Pooling

Use a connection pool with 5-10 connections (defined in `src/infrastructure/pool.rs`):

```rust
PgPoolOptions::new()
    .max_connections(5)
    .connect(&database_url)
    .await?
```

### Pagination

For list operations, implement pagination to handle large datasets:

```rust
pub struct ListOptions {
    pub offset: Option<i64>,
    pub limit: Option<i64>,
}

// Default limit should be reasonable (e.g., 50)
```

### Indexing

Critical indexes are defined in migration files. When adding queries on new columns, consider adding indexes:

```sql
CREATE INDEX idx_tasks_project_id ON tasks(project_id) WHERE deleted_at IS NULL;
CREATE INDEX idx_tasks_status ON tasks(status) WHERE deleted_at IS NULL;
```

## Logging

Use `tracing` for structured logging:

```rust
use tracing::{info, warn, error, debug};

// ✅ Correct: Structured logging
info!(project_id = %project.id, "Created new project");
warn!(task_id = %task.id, "Task dependency may cause cycle");
error!(error = %e, "Failed to connect to database");

// ❌ Wrong: String-based logging
println!("Created project {}", project.id);
```

Log levels:
- `error`: Unrecoverable errors
- `warn`: Recoverable issues, deprecated features
- `info`: Important state changes (create, update, delete)
- `debug`: Detailed flow information
- `trace`: Very detailed information (usually DB queries)

## Security

### SQL Injection Prevention

Always use parameterized queries (sqlx does this automatically):

```rust
// ✅ Correct: Parameterized query
sqlx::query!("SELECT * FROM projects WHERE name = $1", name)

// ❌ Wrong: String interpolation
sqlx::query(&format!("SELECT * FROM projects WHERE name = '{}'", name))
```

### Input Validation

Validate inputs in service layer before repository calls:

```rust
pub async fn create_project(&self, input: CreateProject) -> Result<Project, DevErpError> {
    // Validate
    if input.name.is_empty() {
        return Err(DevErpError::Validation("Project name cannot be empty".into()));
    }

    if let Some(ref end_date) = input.end_date {
        if let Some(ref start_date) = input.start_date {
            if end_date < start_date {
                return Err(DevErpError::Validation("End date must be after start date".into()));
            }
        }
    }

    // Proceed with creation
    self.repository.create(input).await
}
```

## Future Extensions

When implementing new features, follow these guidelines:

1. **Add database migration** first (in `migrations/`)
2. **Create entity** in appropriate domain module
3. **Define repository trait** with required methods
4. **Implement repository** in infrastructure layer
5. **Create service** with business logic
6. **Add CLI commands** with clap
7. **Write tests** (unit + integration)
8. **Update documentation**

This order ensures dependencies flow correctly and each layer is properly tested before moving to the next.
