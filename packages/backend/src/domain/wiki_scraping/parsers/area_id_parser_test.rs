#[cfg(test)]
mod tests {
    use crate::domain::wiki_scraping::parsers::area_id_parser::AreaIdParser;
    use scraper::Html;

    #[test]
    fn test_parse_area_id() {
        let html = Html::parse_fragment(
            r#"
            <table>
                <tr><td>Id</td><td>zone_1_2</td></tr>
            </table>
        "#,
        );

        assert_eq!(
            AreaIdParser::parse(Some(&html)),
            Some("zone_1_2".to_string())
        );
    }

    #[test]
    fn test_parse_area_id_not_found() {
        let html = Html::parse_fragment(
            r#"
            <table>
                <tr><td>Act</td><td>1</td></tr>
            </table>
        "#,
        );

        assert_eq!(AreaIdParser::parse(Some(&html)), None);
    }

    #[test]
    fn test_parse_area_id_complex() {
        let html = Html::parse_fragment(
            r#"
            <table>
                <tr><td>Id</td><td>HideoutFelled</td></tr>
            </table>
        "#,
        );

        assert_eq!(
            AreaIdParser::parse(Some(&html)),
            Some("HideoutFelled".to_string())
        );
    }

    #[test]
    fn test_parse_no_infobox() {
        assert_eq!(AreaIdParser::parse(None), None);
    }
}
