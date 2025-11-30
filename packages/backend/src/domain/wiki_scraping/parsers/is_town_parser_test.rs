#[cfg(test)]
mod tests {
    use crate::domain::wiki_scraping::parsers::is_town_parser::*;
    use scraper::Html;

    #[test]
    fn test_parse_town_area_label() {
        let html = Html::parse_fragment(
            r#"
            <table>
                <tr><td>Type</td><td>Town area</td></tr>
            </table>
        "#,
        );

        assert!(IsTownParser::parse(Some(&html)));
    }

    #[test]
    fn test_parse_town_from_area_id() {
        let html = Html::parse_fragment(
            r#"
            <table>
                <tr><td>Id</td><td>clearfell_town</td></tr>
            </table>
        "#,
        );

        assert!(IsTownParser::parse(Some(&html)));
    }

    #[test]
    fn test_parse_not_a_town() {
        let html = Html::parse_fragment(
            r#"
            <table>
                <tr><td>Id</td><td>zone_1_2</td></tr>
                <tr><td>Type</td><td>Combat area</td></tr>
            </table>
        "#,
        );

        assert!(!IsTownParser::parse(Some(&html)));
    }

    #[test]
    fn test_parse_no_infobox() {
        assert!(!IsTownParser::parse(None));
    }
}
