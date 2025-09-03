// Utility tests for POE2 Overlord backend
// Tests for string operations, collections, option/result handling, and error patterns

#[test]
fn test_string_operations() {
    let test_string = "Test Zone Name";

    assert_eq!(test_string.len(), 14); // "Test Zone Name" is 14 characters
    assert!(test_string.contains("Zone"));
    assert!(!test_string.contains("Invalid"));
    assert_eq!(test_string.to_lowercase(), "test zone name");
    assert_eq!(test_string.to_uppercase(), "TEST ZONE NAME");
}

#[test]
fn test_vector_operations() {
    let mut vec = Vec::new();

    vec.push(1);
    vec.push(2);
    vec.push(3);

    assert_eq!(vec.len(), 3);
    assert_eq!(vec[0], 1);
    assert_eq!(vec[1], 2);
    assert_eq!(vec[2], 3);

    vec.pop();
    assert_eq!(vec.len(), 2);
    assert_eq!(vec[1], 2);
}

#[test]
fn test_option_handling() {
    let some_value: Option<i32> = Some(42);
    let none_value: Option<i32> = None;

    assert!(some_value.is_some());
    assert!(none_value.is_none());
    assert_eq!(some_value.unwrap(), 42);
    assert_eq!(some_value.unwrap_or(0), 42);
    assert_eq!(none_value.unwrap_or(0), 0);
}

#[test]
fn test_result_handling() {
    let ok_result: Result<i32, &str> = Ok(42);
    let err_result: Result<i32, &str> = Err("error message");

    assert!(ok_result.is_ok());
    assert!(err_result.is_err());
    assert_eq!(ok_result.unwrap(), 42);
    assert_eq!(err_result.unwrap_err(), "error message");
}

#[test]
fn test_error_handling_patterns() {
    // Test common error handling patterns
    let result: Result<i32, &str> = Ok(42);

    match result {
        Ok(value) => assert_eq!(value, 42),
        Err(_) => panic!("Expected Ok value"),
    }

    let result: Result<i32, &str> = Err("error");

    match result {
        Ok(_) => panic!("Expected Err value"),
        Err(e) => assert_eq!(e, "error"),
    }
}
