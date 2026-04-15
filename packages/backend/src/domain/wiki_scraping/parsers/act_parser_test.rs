#[cfg(test)]
mod tests {
    use crate::domain::wiki_scraping::parsers::act_parser::ActParser;
    use scraper::Html;

    #[test]
    fn test_parse_from_infobox() {
        let html = Html::parse_fragment(
            r"
            <table>
                <tr><td>Act</td><td>1</td></tr>
            </table>
        ",
        );
        let doc = Html::parse_document("<html><body></body></html>");

        assert_eq!(ActParser::parse(Some(&html), &doc), 1);
    }

    #[test]
    fn test_parse_from_infobox_with_extra_text() {
        let html = Html::parse_fragment(
            r"
            <table>
                <tr><td>Act</td><td>2 extra text</td></tr>
            </table>
        ",
        );
        let doc = Html::parse_document("<html><body></body></html>");

        assert_eq!(ActParser::parse(Some(&html), &doc), 2);
    }

    #[test]
    fn test_parse_from_page_text_fallback() {
        let doc = Html::parse_document(
            r"
            <html>
                <body>
                    <p>This is an area in Act 3 of the game.</p>
                </body>
            </html>
        ",
        );

        assert_eq!(ActParser::parse(None, &doc), 3);
    }

    #[test]
    fn test_parse_not_found_returns_zero() {
        let html = Html::parse_fragment(
            r"
            <table>
                <tr><td>Level</td><td>42</td></tr>
            </table>
        ",
        );
        let doc = Html::parse_document("<html><body><p>No act information</p></body></html>");

        assert_eq!(ActParser::parse(Some(&html), &doc), 0);
    }

    #[test]
    fn test_parse_no_infobox() {
        let doc = Html::parse_document("<html><body><p>No act information</p></body></html>");

        assert_eq!(ActParser::parse(None, &doc), 0);
    }
}
