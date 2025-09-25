use crate::infrastructure::parsing::ParseError;
use crate::infrastructure::parsing::validation::validate_content;
use std::borrow::Cow;

pub fn extract_content_between_delimiters<'a>(
    line: &'a str,
    pattern: &str,
    start_delimiter: char,
    end_delimiter: char,
) -> Result<Cow<'a, str>, ParseError> {
    if let Some(start) = line.find(pattern) {
        let content_start = start + pattern.len();
        let remaining = &line[content_start..];

        let content_start_pos = if pattern.ends_with(start_delimiter) {
            content_start
        } else {
            if let Some(start_bracket) = remaining.find(start_delimiter) {
                content_start + start_bracket + 1
            } else {
                return Err(ParseError::content_extraction_failed(line));
            }
        };

        let content_remaining = &line[content_start_pos..];

        if let Some(end_bracket) = content_remaining.rfind(end_delimiter) {
            let content = &line[content_start_pos..content_start_pos + end_bracket];
            let trimmed_content = content.trim();

            if validate_content(trimmed_content) {
                return Ok(Cow::Borrowed(trimmed_content));
            } else {
                return Err(ParseError::invalid_content(trimmed_content));
            }
        }
    }

    Err(ParseError::content_extraction_failed(line))
}

pub fn extract_content_by_patterns<'a>(
    line: &'a str,
    patterns: &[String],
    start_delimiter: char,
    end_delimiter: char,
) -> Result<Cow<'a, str>, ParseError> {
    for pattern in patterns {
        if let Ok(content) =
            extract_content_between_delimiters(line, pattern, start_delimiter, end_delimiter)
        {
            return Ok(content);
        }
    }

    Err(ParseError::content_extraction_failed(line))
}
