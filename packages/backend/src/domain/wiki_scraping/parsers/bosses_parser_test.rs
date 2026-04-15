#[cfg(test)]
mod tests {
    use crate::domain::wiki_scraping::parsers::bosses_parser::*;
    use scraper::Html;

    fn make_infobox(bosses_cell: &str) -> Html {
        Html::parse_fragment(&format!(
            r"<table><tbody>
              <tr><th>Id</th><td>test_zone</td></tr>
              <tr><th>Act</th><td>1</td></tr>
              <tr><th>Bosses</th><td>{bosses_cell}</td></tr>
            </tbody></table>",
        ))
    }

    #[test]
    fn test_parse_bosses_from_infobox_links() {
        let infobox = make_infobox(
            r#"<a href="/wiki/Lachlann">Lachlann of Endless Lament</a>, <a href="/wiki/Vargir">Vargir the Feral Mutt</a>"#,
        );
        let doc = Html::parse_document("<html><body></body></html>");
        let bosses = BossesParser::parse(Some(&infobox), &doc);
        assert_eq!(bosses.len(), 2);
        assert!(bosses.contains(&"Lachlann of Endless Lament".to_string()));
        assert!(bosses.contains(&"Vargir the Feral Mutt".to_string()));
    }

    #[test]
    fn test_parse_bosses_from_infobox_text_fallback() {
        // Cell has no links — should comma-split the raw text
        let infobox = make_infobox("Doryani, Voll");
        let doc = Html::parse_document("<html><body></body></html>");
        let bosses = BossesParser::parse(Some(&infobox), &doc);
        assert_eq!(bosses.len(), 2);
        assert!(bosses.contains(&"Doryani".to_string()));
        assert!(bosses.contains(&"Voll".to_string()));
    }

    #[test]
    fn test_parse_bosses_from_section_when_no_infobox() {
        let doc = Html::parse_document(
            r"<html><body>
                <h2>Bosses</h2>
                <ul>
                    <li>Lachlann of Endless Lament</li>
                    <li>Vargir the Feral Mutt</li>
                </ul>
            </body></html>",
        );
        let bosses = BossesParser::parse(None, &doc);
        assert_eq!(bosses.len(), 2);
        assert!(bosses.contains(&"Lachlann of Endless Lament".to_string()));
    }

    #[test]
    fn test_parse_bosses_from_monsters_heuristic() {
        // No infobox, no dedicated boss section — falls back to heuristic filter on Monsters
        let doc = Html::parse_document(
            r"<html><body>
                <h2>Monsters</h2>
                <ul>
                    <li>Regular Monster</li>
                    <li>Vargir the Feral Mutt</li>
                    <li>Another Monster</li>
                    <li>Boss the Destroyer</li>
                </ul>
            </body></html>",
        );
        let bosses = BossesParser::parse(None, &doc);
        assert_eq!(bosses.len(), 2);
        assert!(bosses.contains(&"Vargir the Feral Mutt".to_string()));
        assert!(bosses.contains(&"Boss the Destroyer".to_string()));
    }

    #[test]
    fn test_parse_no_bosses() {
        let doc = Html::parse_document(
            r"<html><body>
                <h2>Monsters</h2>
                <ul>
                    <li>Regular Monster 1</li>
                    <li>Regular Monster 2</li>
                </ul>
            </body></html>",
        );
        let bosses = BossesParser::parse(None, &doc);
        assert!(bosses.is_empty());
    }

    #[test]
    fn test_parse_no_monsters_section() {
        let doc = Html::parse_document(
            r"<html><body>
                <h2>Other Section</h2>
                <p>Some content</p>
            </body></html>",
        );
        let bosses = BossesParser::parse(None, &doc);
        assert!(bosses.is_empty());
    }
}
