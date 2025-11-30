#[cfg(test)]
mod tests {
    use crate::domain::wiki_scraping::parsers::image_url_parser::*;
    use scraper::Html;

    #[test]
    fn test_parse_image_url() {
        let html = Html::parse_document(
            r#"
            <html>
                <body>
                    <img src="/images/area_screenshot.jpg" alt="Zone Screenshot">
                    <img src="/images/other.jpg" alt="Other Image">
                </body>
            </html>
        "#,
        );

        let image_url = ImageUrlParser::parse(&html);
        assert!(image_url.is_some());
        let url = image_url.unwrap();
        assert!(url.contains("area_screenshot.jpg"));
        assert!(url.starts_with("https://"));
    }

    #[test]
    fn test_parse_no_image() {
        let html = Html::parse_document(
            r#"
            <html>
                <body>
                    <img src="/images/icon.png" alt="Icon">
                </body>
            </html>
        "#,
        );

        let image_url = ImageUrlParser::parse(&html);
        assert!(image_url.is_none());
    }

    #[test]
    fn test_parse_thumbnail_image() {
        let html = Html::parse_document(
            r#"
            <html>
                <body>
                    <img src="/images/thumb/a/ab/area_screenshot.jpg/300px-area_screenshot.jpg" alt="Screenshot">
                </body>
            </html>
        "#,
        );

        let image_url = ImageUrlParser::parse(&html);
        assert!(image_url.is_some());
        let url = image_url.unwrap();
        assert!(!url.contains("/thumb/"));
        assert!(!url.contains("300px"));
        assert!(url.contains("/images/a/ab/area_screenshot.jpg"));
    }

    #[test]
    fn test_parse_real_example() {
        let html = Html::parse_document(
            r#"
            <html>
                <body>
                    <img src="https://www.poe2wiki.net/images/7/79/Cemetery_of_the_Eternals_area_screenshot.jpg" alt="Screenshot">
                </body>
            </html>
        "#,
        );

        let image_url = ImageUrlParser::parse(&html);
        assert!(image_url.is_some());
        assert_eq!(
            image_url.unwrap(),
            "https://www.poe2wiki.net/images/7/79/Cemetery_of_the_Eternals_area_screenshot.jpg"
        );
    }

    #[test]
    fn test_parse_webp_format() {
        let html = Html::parse_document(
            r#"
            <html>
                <body>
                    <img src="https://www.poe2wiki.net/images/b/bc/Felled_Hideout_area_screenshot.webp" alt="Screenshot">
                </body>
            </html>
        "#,
        );

        let image_url = ImageUrlParser::parse(&html);
        assert!(image_url.is_some());
        assert_eq!(
            image_url.unwrap(),
            "https://www.poe2wiki.net/images/b/bc/Felled_Hideout_area_screenshot.webp"
        );
    }
}
