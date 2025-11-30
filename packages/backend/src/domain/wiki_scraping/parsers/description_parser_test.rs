#[cfg(test)]
mod tests {
    use crate::domain::wiki_scraping::parsers::description_parser::*;
    use scraper::Html;

    #[test]
    fn test_parse_description() {
        let html = Html::parse_document(
            r#"
            <html>
                <body>
                    <p><i>This is a description of the zone that is longer than ten characters.</i></p>
                </body>
            </html>
        "#,
        );

        let description = DescriptionParser::parse(&html);
        assert!(description.is_some());
        assert!(description.unwrap().contains("description of the zone"));
    }

    #[test]
    fn test_parse_description_with_em_tag() {
        let html = Html::parse_document(
            r#"
            <html>
                <body>
                    <p><em>The bootprint of the Eternals upon the face of Ogham.</em></p>
                </body>
            </html>
        "#,
        );

        let description = DescriptionParser::parse(&html);
        assert!(description.is_some());
        assert_eq!(
            description.unwrap(),
            "The bootprint of the Eternals upon the face of Ogham."
        );
    }

    #[test]
    fn test_parse_description_too_short() {
        let html = Html::parse_document(
            r#"
            <html>
                <body>
                    <p><i>Short</i></p>
                </body>
            </html>
        "#,
        );

        let description = DescriptionParser::parse(&html);
        assert!(description.is_none());
    }

    #[test]
    fn test_parse_no_description() {
        let html = Html::parse_document(
            r#"
            <html>
                <body>
                    <p>Regular text without italic formatting.</p>
                </body>
            </html>
        "#,
        );

        let description = DescriptionParser::parse(&html);
        assert!(description.is_none());
    }

    #[test]
    fn test_parse_multiple_italics_returns_first_long_enough() {
        let html = Html::parse_document(
            r#"
            <html>
                <body>
                    <p><i>Short</i></p>
                    <p><i>This is the description we want to extract here.</i></p>
                    <p><i>Another description text.</i></p>
                </body>
            </html>
        "#,
        );

        let description = DescriptionParser::parse(&html);
        assert!(description.is_some());
        assert_eq!(
            description.unwrap(),
            "This is the description we want to extract here."
        );
    }
}
