pub fn validate_content(content: &str) -> bool {
    !content.is_empty()
        && content != "(null)"
        && content != "(undefined)"
        && content != "(unknown)"
        && content != "undefined"
        && content.to_lowercase() != "null"
        && content.to_lowercase() != "undefined"
}
