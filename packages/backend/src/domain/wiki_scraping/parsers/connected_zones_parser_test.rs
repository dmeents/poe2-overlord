#[cfg(test)]
mod tests {
    use crate::domain::wiki_scraping::parsers::connected_zones_parser::*;
    use scraper::Html;

    #[test]
    fn test_parse_from_infobox() {
        let infobox = Html::parse_fragment(
            r#"
            <table>
                <tr>
                    <td>Connections</td>
                    <td>
                        <a href="/wiki/Zone_1">Zone 1</a>,
                        <a href="/wiki/Zone_2">Zone 2</a>,
                        <a href="/wiki/Zone_3">Zone 3</a>
                    </td>
                </tr>
            </table>
        "#,
        );
        let doc = Html::parse_document("<html><body></body></html>");

        let connections = ConnectedZonesParser::parse(Some(&infobox), &doc);
        assert_eq!(connections.len(), 3);
        assert!(connections.contains(&"Zone 1".to_string()));
        assert!(connections.contains(&"Zone 2".to_string()));
        assert!(connections.contains(&"Zone 3".to_string()));
    }

    #[test]
    fn test_parse_from_page_text_fallback() {
        let infobox = Html::parse_fragment(
            r#"
            <table>
                <tr><td>Act</td><td>1</td></tr>
            </table>
        "#,
        );
        let doc = Html::parse_document(
            r#"
            <html>
                <body>
                    <p>This area is connected to The Grim Tangle, Mausoleum, and Hunting Grounds.</p>
                </body>
            </html>
        "#,
        );

        let connections = ConnectedZonesParser::parse(Some(&infobox), &doc);
        assert_eq!(connections.len(), 3);
        assert!(connections.contains(&"The Grim Tangle".to_string()));
        assert!(connections.contains(&"Mausoleum".to_string()));
        assert!(connections.contains(&"Hunting Grounds".to_string()));
    }

    #[test]
    fn test_parse_no_connections() {
        let infobox = Html::parse_fragment(
            r#"
            <table>
                <tr><td>Act</td><td>1</td></tr>
            </table>
        "#,
        );
        let doc = Html::parse_document("<html><body><p>No connections mentioned</p></body></html>");

        let connections = ConnectedZonesParser::parse(Some(&infobox), &doc);
        assert!(connections.is_empty());
    }

    #[test]
    fn test_parse_no_infobox() {
        let doc = Html::parse_document(
            r#"
            <html>
                <body>
                    <p>This zone is connected to Zone A, Zone B, and Zone C.</p>
                </body>
            </html>
        "#,
        );

        let connections = ConnectedZonesParser::parse(None, &doc);
        assert_eq!(connections.len(), 3);
        assert!(connections.contains(&"Zone A".to_string()));
        assert!(connections.contains(&"Zone B".to_string()));
        assert!(connections.contains(&"Zone C".to_string()));
    }

    #[test]
    fn test_parse_real_example() {
        let infobox = Html::parse_fragment(
            r#"
            <table>
                <tr>
                    <td>Connections</td>
                    <td>
                        <a href="/wiki/The_Grim_Tangle">The Grim Tangle</a>,
                        <a href="/wiki/Mausoleum_of_the_Praetor">Mausoleum of the Praetor</a>,
                        <a href="/wiki/Tomb_of_the_Consort">Tomb of the Consort</a>,
                        <a href="/wiki/Hunting_Grounds">Hunting Grounds</a>
                    </td>
                </tr>
            </table>
        "#,
        );
        let doc = Html::parse_document("<html><body></body></html>");

        let connections = ConnectedZonesParser::parse(Some(&infobox), &doc);
        assert_eq!(connections.len(), 4);
        assert!(connections.contains(&"The Grim Tangle".to_string()));
        assert!(connections.contains(&"Mausoleum of the Praetor".to_string()));
        assert!(connections.contains(&"Tomb of the Consort".to_string()));
        assert!(connections.contains(&"Hunting Grounds".to_string()));
    }
}
