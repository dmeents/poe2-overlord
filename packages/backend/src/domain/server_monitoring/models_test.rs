//! Unit tests for `ServerStatus` model

#[cfg(test)]
mod tests {
    use crate::domain::server_monitoring::models::{ServerStatus, DEFAULT_SERVER_PORT};

    #[test]
    fn test_default_creates_invalid_status() {
        let status = ServerStatus::default();

        assert_eq!(status.ip_address, "127.0.0.1");
        assert_eq!(status.port, DEFAULT_SERVER_PORT);
        assert!(!status.is_online);
        assert_eq!(status.latency_ms, None);
        assert!(!status.timestamp.is_empty());
    }

    #[test]
    fn test_new_creates_offline_status() {
        let ip = "192.168.1.100".to_string();
        let port = 8080;

        let status = ServerStatus::new(ip.clone(), port);

        assert_eq!(status.ip_address, ip);
        assert_eq!(status.port, port);
        assert!(!status.is_online);
        assert_eq!(status.latency_ms, None);
        assert!(!status.timestamp.is_empty());
    }

    #[test]
    fn test_is_valid_with_valid_ipv4() {
        let status = ServerStatus::new("192.168.1.1".to_string(), 6112);
        assert!(status.is_valid());
    }

    #[test]
    fn test_is_valid_with_valid_ipv6() {
        let status = ServerStatus::new("2001:0db8:85a3:0000:0000:8a2e:0370:7334".to_string(), 6112);
        assert!(status.is_valid());
    }

    #[test]
    fn test_is_valid_with_localhost() {
        let status = ServerStatus::new("127.0.0.1".to_string(), 6112);
        assert!(status.is_valid());
    }

    #[test]
    fn test_is_invalid_with_empty_ip() {
        let status = ServerStatus::new(String::new(), 6112);
        assert!(!status.is_valid());
    }

    #[test]
    fn test_is_invalid_with_zero_ip() {
        let status = ServerStatus::new("0.0.0.0".to_string(), 6112);
        assert!(!status.is_valid());
    }

    #[test]
    fn test_is_invalid_with_malformed_ip() {
        let status = ServerStatus::new("not.an.ip.address".to_string(), 6112);
        assert!(!status.is_valid());
    }

    #[test]
    fn test_is_invalid_with_partial_ip() {
        let status = ServerStatus::new("192.168.1".to_string(), 6112);
        assert!(!status.is_valid());
    }

    #[test]
    fn test_is_invalid_with_out_of_range_ip() {
        let status = ServerStatus::new("192.168.1.256".to_string(), 6112);
        assert!(!status.is_valid());
    }

    #[test]
    fn test_mark_as_online() {
        let mut status = ServerStatus::new("192.168.1.1".to_string(), 6112);
        let latency = 42;

        let timestamp_before = status.timestamp.clone();

        // Small delay to ensure timestamp changes
        std::thread::sleep(std::time::Duration::from_millis(10));

        status.mark_as_online(latency);

        assert!(status.is_online);
        assert_eq!(status.latency_ms, Some(latency));
        assert_ne!(status.timestamp, timestamp_before);
    }

    #[test]
    fn test_mark_as_offline() {
        let mut status = ServerStatus::new("192.168.1.1".to_string(), 6112);

        // First mark as online
        status.mark_as_online(50);
        assert!(status.is_online);
        assert_eq!(status.latency_ms, Some(50));

        let timestamp_before = status.timestamp.clone();

        // Small delay to ensure timestamp changes
        std::thread::sleep(std::time::Duration::from_millis(10));

        // Then mark as offline
        status.mark_as_offline();

        assert!(!status.is_online);
        assert_eq!(status.latency_ms, None);
        assert_ne!(status.timestamp, timestamp_before);
    }

    #[test]
    fn test_mark_as_online_updates_existing_latency() {
        let mut status = ServerStatus::new("192.168.1.1".to_string(), 6112);

        status.mark_as_online(100);
        assert_eq!(status.latency_ms, Some(100));

        status.mark_as_online(50);
        assert_eq!(status.latency_ms, Some(50));
    }

    #[test]
    fn test_timestamp_format_is_rfc3339() {
        let status = ServerStatus::new("192.168.1.1".to_string(), 6112);

        // Verify that the timestamp can be parsed as RFC3339
        let parsed = chrono::DateTime::parse_from_rfc3339(&status.timestamp);
        assert!(parsed.is_ok(), "Timestamp should be valid RFC3339 format");
    }

    #[test]
    fn test_serialization() {
        let status = ServerStatus::new("192.168.1.1".to_string(), 6112);

        let json = serde_json::to_string(&status);
        assert!(json.is_ok(), "ServerStatus should be serializable");

        let json_str = json.unwrap();
        assert!(json_str.contains("192.168.1.1"));
        assert!(json_str.contains("6112"));
    }

    #[test]
    fn test_deserialization() {
        let json = r#"{
            "ip_address": "192.168.1.1",
            "port": 6112,
            "is_online": true,
            "latency_ms": 42,
            "timestamp": "2024-01-01T12:00:00Z"
        }"#;

        let status: Result<ServerStatus, _> = serde_json::from_str(json);
        assert!(status.is_ok(), "ServerStatus should be deserializable");

        let status = status.unwrap();
        assert_eq!(status.ip_address, "192.168.1.1");
        assert_eq!(status.port, 6112);
        assert!(status.is_online);
        assert_eq!(status.latency_ms, Some(42));
    }

    #[test]
    fn test_clone() {
        let status = ServerStatus::new("192.168.1.1".to_string(), 6112);
        let cloned = status.clone();

        assert_eq!(status.ip_address, cloned.ip_address);
        assert_eq!(status.port, cloned.port);
        assert_eq!(status.is_online, cloned.is_online);
        assert_eq!(status.latency_ms, cloned.latency_ms);
        assert_eq!(status.timestamp, cloned.timestamp);
    }
}
