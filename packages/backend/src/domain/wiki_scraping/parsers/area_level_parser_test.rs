#[cfg(test)]
mod tests {
    use crate::domain::wiki_scraping::parsers::area_level_parser::AreaLevelParser;
    use scraper::Html;

    #[test]
    fn test_parse_area_level() {
        let html = Html::parse_fragment(
            r"
            <table>
                <tr><td>Area level</td><td>42</td></tr>
            </table>
        ",
        );

        assert_eq!(AreaLevelParser::parse(Some(&html)), Some(42));
    }

    #[test]
    fn test_parse_area_level_with_extra_text() {
        let html = Html::parse_fragment(
            r"
            <table>
                <tr><td>Area level</td><td>65 (normal)</td></tr>
            </table>
        ",
        );

        assert_eq!(AreaLevelParser::parse(Some(&html)), Some(65));
    }

    #[test]
    fn test_parse_area_level_not_found() {
        let html = Html::parse_fragment(
            r"
            <table>
                <tr><td>Act</td><td>1</td></tr>
            </table>
        ",
        );

        assert_eq!(AreaLevelParser::parse(Some(&html)), None);
    }

    #[test]
    fn test_parse_no_infobox() {
        assert_eq!(AreaLevelParser::parse(None), None);
    }
}
