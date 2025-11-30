#[cfg(test)]
mod tests {
    use crate::domain::wiki_scraping::parsers::base::*;

    #[test]
    fn test_parse_number() {
        assert_eq!(BaseParser::parse_number("42"), Some(42));
        assert_eq!(BaseParser::parse_number("42 extra text"), Some(42));
        assert_eq!(BaseParser::parse_number("  42  "), Some(42));
        assert_eq!(BaseParser::parse_number("not a number"), None);
    }

    #[test]
    fn test_contains_case_insensitive() {
        assert!(BaseParser::contains_case_insensitive(
            "Hello World",
            "hello"
        ));
        assert!(BaseParser::contains_case_insensitive(
            "Hello World",
            "WORLD"
        ));
        assert!(!BaseParser::contains_case_insensitive(
            "Hello World",
            "goodbye"
        ));
    }

    #[test]
    fn test_split_before_delimiter() {
        assert_eq!(
            BaseParser::split_before_delimiter("Name - Description", &['-']),
            "Name"
        );
        assert_eq!(
            BaseParser::split_before_delimiter("Name (Description)", &['(', '-']),
            "Name"
        );
        assert_eq!(
            BaseParser::split_before_delimiter("Just Name", &['-', '(']),
            "Just Name"
        );
    }
}
