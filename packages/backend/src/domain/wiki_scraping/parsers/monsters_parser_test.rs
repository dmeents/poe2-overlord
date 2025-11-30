#[cfg(test)]
mod tests {
    use crate::domain::wiki_scraping::parsers::monsters_parser::*;
    use scraper::Html;

    #[test]
    fn test_parse_monsters() {
        let html = Html::parse_document(
            r#"
            <html>
                <body>
                    <h2>Monsters</h2>
                    <ul>
                        <li>Monster 1</li>
                        <li>Monster 2</li>
                        <li>Monster 3</li>
                    </ul>
                </body>
            </html>
        "#,
        );

        let monsters = MonstersParser::parse(&html);
        assert_eq!(monsters.len(), 3);
        assert!(monsters.contains(&"Monster 1".to_string()));
        assert!(monsters.contains(&"Monster 2".to_string()));
        assert!(monsters.contains(&"Monster 3".to_string()));
    }

    #[test]
    fn test_parse_monsters_with_bosses() {
        let html = Html::parse_document(
            r#"
            <html>
                <body>
                    <h2>Monsters</h2>
                    <ul>
                        <li>Lachlann of Endless Lament</li>
                        <li>Burdened Wretch</li>
                        <li>Death Knight</li>
                        <li>Boss the Destroyer</li>
                    </ul>
                </body>
            </html>
        "#,
        );

        let monsters = MonstersParser::parse(&html);
        assert_eq!(monsters.len(), 4);
        assert!(monsters.contains(&"Lachlann of Endless Lament".to_string()));
        assert!(monsters.contains(&"Burdened Wretch".to_string()));
        assert!(monsters.contains(&"Death Knight".to_string()));
        assert!(monsters.contains(&"Boss the Destroyer".to_string()));
    }

    #[test]
    fn test_parse_no_monsters() {
        let html = Html::parse_document(
            r#"
            <html>
                <body>
                    <h2>NPCs</h2>
                    <ul>
                        <li>NPC 1</li>
                    </ul>
                </body>
            </html>
        "#,
        );

        let monsters = MonstersParser::parse(&html);
        assert!(monsters.is_empty());
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

        let monsters = MonstersParser::parse(&html);
        assert!(monsters.is_empty());
    }
}
