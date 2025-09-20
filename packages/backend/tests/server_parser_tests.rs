use app_lib::parsers::core::LogParser;
use app_lib::parsers::specific_parsers::ServerConnectionParser;

#[test]
fn test_server_connection_parser_with_ip_port() {
    let parser = ServerConnectionParser::new();

    // Test the problematic log line from the error
    let log_line = "2025/09/03 22:43:49 246857285 91c6ccb [INFO Client 320] Connecting to instance server at 64.87.41.217:21360";

    // Should match the pattern
    assert!(parser.should_parse(log_line));

    // Should successfully parse
    let result = parser.parse_line(log_line);
    assert!(result.is_ok(), "Failed to parse log line: {:?}", result);

    let event = result.unwrap();
    assert_eq!(event.ip_address, "64.87.41.217");
    assert_eq!(event.port, 21360);
}

#[test]
fn test_server_connection_parser_with_different_ip_port() {
    let parser = ServerConnectionParser::new();

    // Test with a different IP:PORT combination
    let log_line = "2025/09/03 22:43:49 246857285 91c6ccb [INFO Client 320] Connecting to instance server at 192.168.1.100:8080";

    // Should match the pattern
    assert!(parser.should_parse(log_line));

    // Should successfully parse
    let result = parser.parse_line(log_line);
    assert!(result.is_ok(), "Failed to parse log line: {:?}", result);

    let event = result.unwrap();
    assert_eq!(event.ip_address, "192.168.1.100");
    assert_eq!(event.port, 8080);
}

#[test]
fn test_server_connection_parser_invalid_line() {
    let parser = ServerConnectionParser::new();

    // Test with a line that doesn't match the pattern
    let log_line = "2025/09/03 22:43:49 246857285 91c6ccb [INFO Client 320] Some other log message";

    // Should not match the pattern
    assert!(!parser.should_parse(log_line));
}

#[test]
fn test_server_connection_parser_malformed_ip_port() {
    let parser = ServerConnectionParser::new();

    // Test with malformed IP:PORT
    let log_line = "2025/09/03 22:43:49 246857285 91c6ccb [INFO Client 320] Connecting to instance server at invalid-ip";

    // Should match the pattern but fail to parse
    assert!(parser.should_parse(log_line));

    let result = parser.parse_line(log_line);
    assert!(
        result.is_err(),
        "Should have failed to parse malformed IP:PORT"
    );
}
