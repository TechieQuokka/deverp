# DevERP Integration Tests

This directory contains integration tests for the DevERP system. These tests verify the complete functionality of the application from CLI to database.

## Test Structure

### Test Helpers (`helpers/`)

- `database.rs`: Database setup, cleanup, and migration utilities
- `fixtures.rs`: Test data factories for creating test entities

### Test Scenarios

1. **`integration_test.rs`**: Basic integration tests
   - Project CRUD operations
   - Task creation and management
   - Database connectivity

2. **`scenario_1_project_lifecycle.rs`**: Complete project lifecycle (NEEDS UPDATES)
   - Project creation to completion workflow
   - Task management
   - Timeline and milestones
   - Resource linking
   - Report generation

3. **`scenario_2_task_dependencies.rs`**: Task dependency management (NEEDS UPDATES)
   - Dependency creation
   - Circular dependency detection
   - Dependency removal
   - Complex dependency chains

4. **`scenario_3_resource_management.rs`**: Resource management (NEEDS UPDATES)
   - Resource creation
   - Project-resource linking
   - Resource utilization analysis
   - Resource lifecycle

5. **`performance_tests.rs`**: Performance benchmarks (NEEDS UPDATES)
   - Bulk operations (100 projects, 1000 tasks)
   - Query performance
   - Memory usage
   - Concurrent operations
   - Connection pool efficiency

6. **`error_handling_tests.rs`**: Error handling (NEEDS UPDATES)
   - Validation errors
   - Not found errors
   - Duplicate data handling
   - Referential integrity
   - Transaction rollback
   - Connection errors

## Running Tests

### Prerequisites

1. **PostgreSQL Database**: Ensure PostgreSQL is running
2. **Test Database**: Create a test database (recommended to be separate from development DB)
   ```sql
   CREATE DATABASE deverp_test;
   CREATE USER deverp_user WITH PASSWORD '2147483647';
   GRANT ALL PRIVILEGES ON DATABASE deverp_test TO deverp_user;
   ```

3. **Environment Variable**: Set the DATABASE_URL
   ```bash
   export DATABASE_URL="postgres://deverp_user:2147483647@localhost:5432/deverp_test"
   ```

### Running Tests

```bash
# Run all tests
cargo test

# Run only integration tests
cargo test --test integration_test

# Run specific test
cargo test test_create_and_get_project

# Run tests with output
cargo test -- --nocapture

# Run tests in sequence (not parallel)
cargo test -- --test-threads=1
```

### Important Notes

- Tests use soft deletes, so data accumulates between test runs
- Each test should call `setup_test_database()` which runs migrations and cleans up data
- Use `-- --test-threads=1` if you encounter database connection issues
- Some tests are resource-intensive and may take time to complete

## Test Coverage

### Phase 14 Integration Tests Status

- ✅ Test helpers created (database setup, fixtures)
- ✅ Basic integration test working
- ⚠️  Scenario 1 (Project Lifecycle) - Created but needs service method updates
- ⚠️  Scenario 2 (Task Dependencies) - Created but needs service method updates
- ⚠️  Scenario 3 (Resource Management) - Created but needs service method updates
- ⚠️  Performance tests - Created but needs service method updates
- ⚠️  Error handling tests - Created but needs service method updates

### Known Issues

The advanced test scenarios (2-6) were created based on the architectural design but need updates to match the actual service implementations. Specifically:

1. Some service methods have different signatures than initially designed
2. Entity structures (CreateTask, UpdateProject, etc.) have fields that differ from the design docs
3. Some methods may not exist yet (e.g., `add_dependency` on TaskService)

### Next Steps to Complete Phase 14

1. **Update Service Methods**: Review and update test files to match actual service signatures
2. **Verify Entity Structures**: Ensure all CreateX and UpdateX structures match the actual implementations
3. **Add Missing Methods**: If tests require methods that don't exist, either implement them or modify tests
4. **Run and Debug**: Execute tests and fix any remaining compilation or runtime issues
5. **Measure Performance**: Run performance tests and document baseline metrics
6. **Coverage Report**: Generate test coverage report using `cargo tarpaulin` or similar

## Test Database Cleanup

To completely reset the test database:

```sql
-- Connect to postgres database
\c postgres

-- Drop and recreate test database
DROP DATABASE IF EXISTS deverp_test;
CREATE DATABASE deverp_test;
GRANT ALL PRIVILEGES ON DATABASE deverp_test TO deverp_user;
```

## Continuous Integration

For CI/CD pipelines, consider:

1. Using Docker Compose to spin up PostgreSQL for tests
2. Running tests with `--test-threads=1` to avoid database connection conflicts
3. Setting appropriate timeouts for performance tests
4. Collecting and archiving test reports

## Contributing

When adding new integration tests:

1. Place test helpers in `helpers/` directory
2. Use `setup_test_database()` at the start of each test
3. Follow naming convention: `test_<feature>_<scenario>`
4. Add documentation in this README
5. Ensure tests are idempotent and can run in any order
