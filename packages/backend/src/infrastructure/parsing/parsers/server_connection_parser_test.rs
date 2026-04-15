#[cfg(test)]
mod tests {
    use crate::infrastructure::parsing::manager::ParserResult;
    use crate::infrastructure::parsing::parsers::server_connection_parser::ServerConnectionParser;
    use crate::infrastructure::parsing::LogParser;

    fn create_parser() -> ServerConnectionParser {
        ServerConnectionParser::new()
    }

    // ============= should_parse Tests =============

    #[test]
    fn test_should_parse_valid_server_connection_line() {
        let parser = create_parser();
        let line = "2026/01/11 10:30:45 12345678 abc [INFO Client 1234] Connecting to instance server at 192.168.1.1:6112";
        assert!(parser.should_parse(line));
    }

    #[test]
    fn test_should_parse_minimal_pattern() {
        let parser = create_parser();
        let line = "Connecting to instance server at 10.0.0.1:8080";
        assert!(parser.should_parse(line));
    }

    #[test]
    fn test_should_parse_returns_false_for_level_up() {
        let parser = create_parser();
        let line = "[INFO Client 1234] : TestCharacter (Warrior) is now level 50";
        assert!(!parser.should_parse(line));
    }

    #[test]
    fn test_should_parse_returns_false_for_scene_change() {
        let parser = create_parser();
        let line = "[INFO Client 1234] [SCENE] Set Source [The Coast]";
        assert!(!parser.should_parse(line));
    }

    #[test]
    fn test_should_parse_returns_false_for_similar_text() {
        let parser = create_parser();
        // Missing "instance" - should not match
        let line = "Connecting to server at 192.168.1.1:6112";
        assert!(!parser.should_parse(line));
    }

    #[test]
    fn test_should_parse_returns_false_for_empty_line() {
        let parser = create_parser();
        assert!(!parser.should_parse(""));
    }

    // ============= parse_line Tests =============

    #[test]
    fn test_parse_line_extracts_ip_and_port() {
        let parser = create_parser();
        let line = "Connecting to instance server at 192.168.1.1:6112";

        let result = parser.parse_line(line);
        assert!(result.is_ok());

        match result.unwrap() {
            ParserResult::ServerConnection(event) => {
                assert_eq!(event.ip_address, "192.168.1.1");
                assert_eq!(event.port, 6112);
            }
            _ => panic!("Expected ServerConnection result"),
        }
    }

    #[test]
    fn test_parse_line_with_full_log_format() {
        let parser = create_parser();
        let line = "2026/01/11 10:30:45 12345678 abc [INFO Client 1234] Connecting to instance server at 10.0.0.1:8080";

        let result = parser.parse_line(line);
        assert!(result.is_ok());

        match result.unwrap() {
            ParserResult::ServerConnection(event) => {
                assert_eq!(event.ip_address, "10.0.0.1");
                assert_eq!(event.port, 8080);
            }
            _ => panic!("Expected ServerConnection result"),
        }
    }

    #[test]
    fn test_parse_line_with_different_ports() {
        let parser = create_parser();

        let test_cases = vec![
            ("Connecting to instance server at 1.2.3.4:1", 1u16),
            ("Connecting to instance server at 1.2.3.4:80", 80u16),
            ("Connecting to instance server at 1.2.3.4:443", 443u16),
            ("Connecting to instance server at 1.2.3.4:6112", 6112u16),
            ("Connecting to instance server at 1.2.3.4:65535", 65535u16),
        ];

        for (line, expected_port) in test_cases {
            let result = parser.parse_line(line);
            assert!(result.is_ok(), "Should parse port: {expected_port}");

            match result.unwrap() {
                ParserResult::ServerConnection(event) => {
                    assert_eq!(event.port, expected_port, "Port should match");
                }
                _ => panic!("Expected ServerConnection result"),
            }
        }
    }

    #[test]
    fn test_parse_line_with_localhost() {
        let parser = create_parser();
        let line = "Connecting to instance server at 127.0.0.1:6112";

        let result = parser.parse_line(line);
        assert!(result.is_ok());

        match result.unwrap() {
            ParserResult::ServerConnection(event) => {
                assert_eq!(event.ip_address, "127.0.0.1");
                assert_eq!(event.port, 6112);
            }
            _ => panic!("Expected ServerConnection result"),
        }
    }

    #[test]
    fn test_parse_line_returns_error_for_non_matching_line() {
        let parser = create_parser();
        let line = "[INFO Client 1234] : TestChar has been slain.";

        let result = parser.parse_line(line);
        assert!(result.is_err());
    }

    #[test]
    fn test_parse_line_returns_error_for_missing_port() {
        let parser = create_parser();
        let line = "Connecting to instance server at 192.168.1.1";

        let result = parser.parse_line(line);
        assert!(result.is_err());
    }

    #[test]
    fn test_parse_line_returns_error_for_invalid_port() {
        let parser = create_parser();
        let line = "Connecting to instance server at 192.168.1.1:notaport";

        let result = parser.parse_line(line);
        assert!(result.is_err());
    }

    #[test]
    fn test_parse_line_returns_error_for_port_out_of_range() {
        let parser = create_parser();
        // Port 70000 is > 65535 (u16 max)
        let line = "Connecting to instance server at 192.168.1.1:70000";

        let result = parser.parse_line(line);
        assert!(result.is_err());
    }

    #[test]
    fn test_parse_line_returns_error_for_empty_ip() {
        let parser = create_parser();
        let line = "Connecting to instance server at :6112";

        let result = parser.parse_line(line);
        assert!(result.is_err());
    }

    #[test]
    fn test_parse_line_returns_error_for_empty_port() {
        let parser = create_parser();
        let line = "Connecting to instance server at 192.168.1.1:";

        let result = parser.parse_line(line);
        assert!(result.is_err());
    }

    #[test]
    fn test_parse_line_handles_trailing_whitespace() {
        let parser = create_parser();
        let line = "Connecting to instance server at 192.168.1.1:6112   ";

        let result = parser.parse_line(line);
        assert!(result.is_ok());

        match result.unwrap() {
            ParserResult::ServerConnection(event) => {
                assert_eq!(event.ip_address, "192.168.1.1");
                assert_eq!(event.port, 6112);
            }
            _ => panic!("Expected ServerConnection result"),
        }
    }

    // ============= parser_name Tests =============

    #[test]
    fn test_parser_name() {
        let parser = create_parser();
        assert_eq!(parser.parser_name(), "server_connection");
    }

    // ============= Default Implementation Tests =============

    #[test]
    fn test_default_implementation() {
        let parser: ServerConnectionParser = Default::default();
        assert_eq!(parser.parser_name(), "server_connection");
    }
}
