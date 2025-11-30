#[cfg(test)]
mod tests {
    use crate::domain::wiki_scraping::parsers::points_of_interest_parser::*;
    use scraper::Html;

    #[test]
    fn test_parse_points_of_interest() {
        let html = Html::parse_document(
            r#"
            <html>
                <body>
                    <h2>Points of Interest</h2>
                    <ul>
                        <li>[Miniboss arena] Crop Circle: Description here</li>
                        <li>[Notable landmark] Una's Home: More description</li>
                        <li>[Area] Simple POI</li>
                    </ul>
                </body>
            </html>
        "#,
        );

        let pois = PointsOfInterestParser::parse(&html);
        assert_eq!(pois.len(), 3);
        assert!(pois.contains(&"[Miniboss arena] Crop Circle".to_string()));
        assert!(pois.contains(&"[Notable landmark] Una's Home".to_string()));
        assert!(pois.contains(&"[Area] Simple POI".to_string()));
    }

    #[test]
    fn test_parse_real_example() {
        let html = Html::parse_document(
            r#"
            <html>
                <body>
                    <h2>Points of Interest</h2>
                    <ul>
                        <li>[Boss arena] Memorial of the Lost: Boss fight location</li>
                        <li>[Notable chest] Ancient Ruin: Contains loot</li>
                    </ul>
                </body>
            </html>
        "#,
        );

        let pois = PointsOfInterestParser::parse(&html);
        assert_eq!(pois.len(), 2);
        assert!(pois.contains(&"[Boss arena] Memorial of the Lost".to_string()));
        assert!(pois.contains(&"[Notable chest] Ancient Ruin".to_string()));
    }

    #[test]
    fn test_parse_no_points_of_interest() {
        let html = Html::parse_document(
            r#"
            <html>
                <body>
                    <h2>Other Section</h2>
                    <p>Some text</p>
                </body>
            </html>
        "#,
        );

        let pois = PointsOfInterestParser::parse(&html);
        assert!(pois.is_empty());
    }

    #[test]
    fn test_parse_poi_without_colon() {
        let html = Html::parse_document(
            r#"
            <html>
                <body>
                    <h2>Points of Interest</h2>
                    <ul>
                        <li>[Landmark] Tower</li>
                        <li>[Area] Clearing</li>
                    </ul>
                </body>
            </html>
        "#,
        );

        let pois = PointsOfInterestParser::parse(&html);
        assert_eq!(pois.len(), 2);
        assert!(pois.contains(&"[Landmark] Tower".to_string()));
        assert!(pois.contains(&"[Area] Clearing".to_string()));
    }

    #[test]
    fn test_parse_ignores_items_without_brackets() {
        let html = Html::parse_document(
            r#"
            <html>
                <body>
                    <h2>Points of Interest</h2>
                    <ul>
                        <li>[Valid] POI: description</li>
                        <li>Invalid POI without brackets</li>
                    </ul>
                </body>
            </html>
        "#,
        );

        let pois = PointsOfInterestParser::parse(&html);
        assert_eq!(pois.len(), 1);
        assert!(pois.contains(&"[Valid] POI".to_string()));
    }
}
