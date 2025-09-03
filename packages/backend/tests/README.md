# POE2 Overlord Backend Tests

This directory contains a simplified test suite focused on unit tests that can be easily run and maintained.

## Test Structure

- **`mod.rs`** - Main test module declarations
- **`model_tests.rs`** - Tests for scene change events, location sessions, and location types
- **`serialization_tests.rs`** - Tests for chrono timestamps, serde JSON, and serialization roundtrip
- **`utility_tests.rs`** - Tests for string operations, collections, option/result handling, and error patterns
- **`concurrency_tests.rs`** - Tests for Arc reference counting and tokio channels
- **`system_tests.rs`** - Tests for file system operations and logging functionality

## Running Tests

```bash
# Run all tests
cargo test --tests

# Run tests from a specific module
cargo test --tests model_tests
cargo test --tests serialization_tests
cargo test --tests utility_tests
cargo test --tests concurrency_tests
cargo test --tests system_tests

# Run specific test
cargo test --tests test_scene_change_event_zone

# Run tests with output
cargo test --tests -- --nocapture
```

## Test Categories

### Model Tests (`model_tests.rs`)
- Scene change events (Zone, Act, Hideout)
- Location sessions and types
- Data structure validation
- **6 tests**

### Serialization Tests (`serialization_tests.rs`)
- JSON serialization/deserialization
- Chrono timestamp handling
- Serde roundtrip validation
- **3 tests**

### Utility Tests (`utility_tests.rs`)
- String operations
- Vector operations
- Option and Result handling
- Error handling patterns
- **5 tests**

### Concurrency Tests (`concurrency_tests.rs`)
- Arc cloning and reference counting
- Channel operations (mpsc)
- **2 tests**

### System Tests (`system_tests.rs`)
- Path operations
- Logging macros
- **2 tests**

**Total: 18 tests**

## Test Philosophy

These tests focus on:
1. **Simplicity** - Easy to understand and maintain
2. **Isolation** - Test individual functions without complex mocking
3. **Reliability** - Tests that don't depend on external systems
4. **Coverage** - Test core functionality and edge cases
5. **Organization** - Logically grouped by functionality

## Adding New Tests

When adding new tests:
1. **Choose the right file** - Add tests to the appropriate module file
2. Keep them simple and focused
3. Test one concept per test function
4. Use descriptive test names
5. Avoid complex setup or teardown
6. Focus on testing logic, not integration

### Example: Adding a new model test
```rust
// In model_tests.rs
#[test]
fn test_new_model_feature() {
    // Your test here
}
```

## Notes

- The complex handler tests were removed due to architectural constraints
- Focus is on testing models, utilities, and simple functions
- Integration tests would require significant refactoring of the handler code
- All tests should run quickly and reliably
- Tests are now organized in logical, maintainable modules
