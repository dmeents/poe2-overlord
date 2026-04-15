#[cfg(test)]
mod tests {
    use crate::domain::wiki_scraping::parsers::npcs_parser::*;
    use scraper::Html;

    #[test]
    fn test_parse_npcs() {
        let html = Html::parse_document(
            r"
            <html>
                <body>
                    <h2>NPCs</h2>
                    <ul>
                        <li>Una - Quest Giver</li>
                        <li>Merchant (Sells items)</li>
                        <li>Simple Name</li>
                    </ul>
                </body>
            </html>
        ",
        );

        let npcs = NpcsParser::parse(&html);
        assert_eq!(npcs.len(), 3);
        assert!(npcs.contains(&"Una".to_string()));
        assert!(npcs.contains(&"Merchant".to_string()));
        assert!(npcs.contains(&"Simple Name".to_string()));
    }

    #[test]
    fn test_parse_npc_with_dash() {
        let html = Html::parse_document(
            r"
            <html>
                <body>
                    <h2>NPCs</h2>
                    <ul>
                        <li>Lachlann the Lost - Important NPC</li>
                    </ul>
                </body>
            </html>
        ",
        );

        let npcs = NpcsParser::parse(&html);
        assert_eq!(npcs.len(), 1);
        assert_eq!(npcs[0], "Lachlann the Lost");
    }

    #[test]
    fn test_parse_npc_with_parenthesis() {
        let html = Html::parse_document(
            r"
            <html>
                <body>
                    <h2>NPCs</h2>
                    <ul>
                        <li>Vendor (sells equipment)</li>
                    </ul>
                </body>
            </html>
        ",
        );

        let npcs = NpcsParser::parse(&html);
        assert_eq!(npcs.len(), 1);
        assert_eq!(npcs[0], "Vendor");
    }

    #[test]
    fn test_parse_no_npcs() {
        let html = Html::parse_document(
            r"
            <html>
                <body>
                    <h2>Monsters</h2>
                    <ul>
                        <li>Monster 1</li>
                    </ul>
                </body>
            </html>
        ",
        );

        let npcs = NpcsParser::parse(&html);
        assert!(npcs.is_empty());
    }

    #[test]
    fn test_parse_no_npcs_section() {
        let html = Html::parse_document(
            r"
            <html>
                <body>
                    <h2>Other Section</h2>
                    <p>Some content</p>
                </body>
            </html>
        ",
        );

        let npcs = NpcsParser::parse(&html);
        assert!(npcs.is_empty());
    }
}
