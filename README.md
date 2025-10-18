# DevERP

**DevERP** - IT Development Project Management ERP System

A CLI-based project management system designed for individual developers to manage multiple development projects, tasks, resources, and timelines.

## Features

- **Project Management**: Create, track, and manage development projects
- **Task Management**: Organize tasks with dependencies and priorities
- **Resource Tracking**: Monitor development resources (libraries, APIs, tools)
- **Timeline Management**: Schedule projects with milestones
- **Reporting**: Generate analytics and progress reports
- **CLI Interface**: Fast and efficient command-line interface

## Technology Stack

- **Language**: Rust
- **Database**: PostgreSQL 14+
- **Runtime**: Tokio (async)
- **CLI Framework**: clap v4

## Prerequisites

- Rust 1.70+ (https://rustup.rs/)
- PostgreSQL 14+ (https://www.postgresql.org/download/)
- cargo (comes with Rust)

## Quick Start

### 1. Database Setup

Create the database and user:

```sql
CREATE DATABASE deverp;
CREATE USER deverp_user WITH PASSWORD '2147483647';
GRANT ALL PRIVILEGES ON DATABASE deverp TO deverp_user;
```

### 2. Environment Configuration

Copy the example environment file and configure it:

```bash
cp .env.example .env
# Edit .env with your database credentials if different
```

### 3. Install sqlx-cli

```bash
cargo install sqlx-cli --features postgres
```

### 4. Run Database Migrations

```bash
sqlx migrate run
```

### 5. Build and Run

```bash
# Development build
cargo build

# Run the application
cargo run -- --help

# Release build (optimized)
cargo build --release
```

## Usage

### Basic Commands

```bash
# Project management
deverp project create --name "My Project" --description "Project description"
deverp project list
deverp project show <id>

# Task management
deverp task create --project-id 1 --title "Task title"
deverp task list --project-id 1
deverp task update <id> --status done

# Resource management
deverp resource create --name "PostgreSQL" --type database
deverp resource link --project-id 1 --resource-id 1

# Timeline management
deverp timeline create --project-id 1 --name "Sprint 1"
deverp timeline add-milestone --timeline-id 1 --name "MVP"

# Reports
deverp report status
deverp report project-summary
```

## Development

### Project Structure

```
deverp/
├── src/
│   ├── cli/              # CLI interface layer
│   ├── domain/           # Business logic layer
│   │   ├── project/
│   │   ├── task/
│   │   ├── resource/
│   │   └── timeline/
│   ├── infrastructure/   # Data access layer
│   ├── config/           # Configuration management
│   └── utils/            # Utilities (error, logger, formatter)
├── migrations/           # Database migrations
├── tests/                # Integration tests
├── docs/                 # Documentation
└── config/               # Configuration files
```

### Running Tests

```bash
# Run all tests
cargo test

# Run with logging
RUST_LOG=debug cargo test

# Run specific test
cargo test test_name
```

### Code Quality

```bash
# Format code
cargo fmt

# Run linter
cargo clippy

# Check for issues
cargo check
```

## Documentation

- [Architecture Design](docs/architecture.md)
- [Database Schema](docs/database.md)
- [Implementation Plan](plan.md)

## Roadmap

- [x] Phase 1: Project setup and infrastructure
- [ ] Phase 2: Core infrastructure (error handling, logging)
- [ ] Phase 3: Database schema implementation
- [ ] Phase 4: Project domain implementation
- [ ] Phase 5: Task domain implementation
- [ ] Phase 6: Resource domain implementation
- [ ] Phase 7: Timeline domain implementation
- [ ] Phase 8-11: CLI implementation
- [ ] Phase 12: Reporting
- [ ] Phase 13: Configuration management
- [ ] Phase 14-16: Testing, documentation, and deployment

## License

MIT License

## Author

DevERP Team

---

**Status**: Active Development
**Version**: 0.1.0
