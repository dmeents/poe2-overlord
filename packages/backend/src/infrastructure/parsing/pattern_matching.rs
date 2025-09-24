/// Check if a line matches any of the given patterns
pub fn matches_patterns(line: &str, patterns: &[String]) -> bool {
    patterns.iter().any(|pattern| line.contains(pattern))
}
