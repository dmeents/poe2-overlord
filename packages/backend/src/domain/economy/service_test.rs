#[cfg(test)]
mod tests {
    use crate::domain::economy::service::EconomyService;

    #[test]
    fn test_service_creation() {
        let service = EconomyService::new();
        assert!(service.client.get("https://example.com").build().is_ok());
    }
}
