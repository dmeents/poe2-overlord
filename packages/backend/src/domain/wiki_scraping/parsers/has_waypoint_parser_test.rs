#[cfg(test)]
mod tests {
    use crate::domain::wiki_scraping::parsers::has_waypoint_parser::*;
    use scraper::Html;

    #[test]
    fn test_parse_has_waypoint() {
        let html = Html::parse_fragment(
            r#"
            <table>
                <tr><td>Waypoint</td><td>Yes</td></tr>
            </table>
        "#,
        );

        assert!(HasWaypointParser::parse(Some(&html)));
    }

    #[test]
    fn test_parse_has_waypoint_lowercase() {
        let html = Html::parse_fragment(
            r#"
            <table>
                <tr><td>Waypoint</td><td>yes</td></tr>
            </table>
        "#,
        );

        assert!(HasWaypointParser::parse(Some(&html)));
    }

    #[test]
    fn test_parse_no_waypoint() {
        let html = Html::parse_fragment(
            r#"
            <table>
                <tr><td>Waypoint</td><td>No</td></tr>
            </table>
        "#,
        );

        assert!(!HasWaypointParser::parse(Some(&html)));
    }

    #[test]
    fn test_parse_waypoint_not_found() {
        let html = Html::parse_fragment(
            r#"
            <table>
                <tr><td>Act</td><td>1</td></tr>
            </table>
        "#,
        );

        assert!(!HasWaypointParser::parse(Some(&html)));
    }

    #[test]
    fn test_parse_no_infobox() {
        assert!(!HasWaypointParser::parse(None));
    }
}
