#[cfg(test)]
mod tests {
    use crate::domain::wiki_scraping::parsers::bosses_parser::*;
    use scraper::Html;

    #[test]
    fn test_parse_bosses() {
        let html = Html::parse_document(
            r#"
            <html>
                <body>
                    <h2>Monsters</h2>
                    <ul>
                        <li>Regular Monster</li>
                        <li>Vargir the Feral Mutt</li>
                        <li>Another Monster</li>
                        <li>Boss the Destroyer</li>
                    </ul>
                </body>
            </html>
        "#,
        );

        let bosses = BossesParser::parse(&html);
        assert_eq!(bosses.len(), 2);
        assert!(bosses.contains(&"Vargir the Feral Mutt".to_string()));
        assert!(bosses.contains(&"Boss the Destroyer".to_string()));
    }

    #[test]
    fn test_parse_no_bosses() {
        let html = Html::parse_document(
            r#"
            <html>
                <body>
                    <h2>Monsters</h2>
                    <ul>
                        <li>Regular Monster 1</li>
                        <li>Regular Monster 2</li>
                    </ul>
                </body>
            </html>
        "#,
        );

        let bosses = BossesParser::parse(&html);
        assert!(bosses.is_empty());
    }

    #[test]
    fn test_parse_no_monsters_section() {
        let html = Html::parse_document(
            r#"
            <html>
                <body>
                    <h2>Other Section</h2>
                    <p>Some content</p>
                </body>
            </html>
        "#,
        );

        let bosses = BossesParser::parse(&html);
        assert!(bosses.is_empty());
    }

    #[test]
    fn test_parse_case_sensitive_the() {
        let html = Html::parse_document(
            r#"
            <html>
                <body>
                    <h2>Monsters</h2>
                    <ul>
                        <li>Lachlann of Endless Lament</li>
                        <li>Boss the Great</li>
                    </ul>
                </body>
            </html>
        "#,
        );

        let bosses = BossesParser::parse(&html);
        // Only "Boss the Great" should be extracted (contains " the ")
        assert_eq!(bosses.len(), 1);
        assert!(bosses.contains(&"Boss the Great".to_string()));
    }
}
