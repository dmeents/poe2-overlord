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

    #[test]
    fn test_parse_webp_thumbnail_with_png_conversion() {
        // This is the actual format from Felled Hideout wiki page
        let html = Html::parse_document(
            r#"
            <html>
                <body>
                    <img src="/images/thumb/b/bc/Felled_Hideout_area_screenshot.webp/250px-Felled_Hideout_area_screenshot.webp.png"
                         alt="Screenshot"
                         width="250"
                         height="136"
                         data-file-width="589"
                         data-file-height="320" />
                </body>
            </html>
        "#,
        );

        let image_url = ImageUrlParser::parse(&html);
        assert!(image_url.is_some());
        let url = image_url.unwrap();

        // Should still contain thumbnail path since webp originals aren't accessible
        assert!(url.contains("/thumb/"));

        // Should use the maximum width from data-file-width
        assert!(url.contains("589px"));

        // Should be the PNG conversion
        assert!(url.contains(".webp.png"));
        assert!(url.ends_with(".png"));

        // Full expected URL
        assert_eq!(
            url,
            "https://www.poe2wiki.net/images/thumb/b/bc/Felled_Hideout_area_screenshot.webp/589px-Felled_Hideout_area_screenshot.webp.png"
        );
    }

    #[test]
    fn test_parse_hideout_with_multiple_images() {
        // Hideout pages have waypoint icons and screenshots
        let html = Html::parse_document(
            r#"
            <html>
                <body>
                    <img src="/images/thumb/d/d6/No_waypoint_area_icon.png/45px-No_waypoint_area_icon.png" alt="No Waypoint" />
                    <img src="/images/thumb/1/11/AtlasLoadingScreen_loading_screen.png/250px-AtlasLoadingScreen_loading_screen.png" alt="Loading" />
                    <img src="/images/thumb/b/bc/Felled_Hideout_area_screenshot.webp/250px-Felled_Hideout_area_screenshot.webp.png"
                         alt="Screenshot"
                         data-file-width="589"
                         data-file-height="320" />
                </body>
            </html>
        "#,
        );

        let image_url = ImageUrlParser::parse(&html);
        assert!(image_url.is_some());
        let url = image_url.unwrap();

        // Should pick the area_screenshot, not the icon or loading screen
        assert!(url.contains("area_screenshot"));
        assert!(url.contains(".webp"));
        assert!(!url.contains("icon"));
        assert!(!url.contains("loading_screen"));
    }

    #[test]
    fn test_skip_waypoint_icons() {
        let html = Html::parse_document(
            r#"
            <html>
                <body>
                    <img src="/images/waypoint_area_icon.png" alt="Waypoint" />
                    <img src="/images/area_screenshot.jpg" alt="Screenshot" />
                </body>
            </html>
        "#,
        );

        let image_url = ImageUrlParser::parse(&html);
        assert!(image_url.is_some());
        let url = image_url.unwrap();

        // Should skip waypoint icon and get screenshot
        assert!(url.contains("area_screenshot.jpg"));
        assert!(!url.contains("waypoint"));
    }

    #[test]
    fn test_extract_thumbnail_extension() {
        assert_eq!(
            ImageUrlParser::extract_thumbnail_extension(
                "250px-Felled_Hideout_area_screenshot.webp.png"
            ),
            Some("Felled_Hideout_area_screenshot.webp.png".to_string())
        );
        assert_eq!(
            ImageUrlParser::extract_thumbnail_extension("300px-Image.jpg"),
            Some("Image.jpg".to_string())
        );
        assert_eq!(
            ImageUrlParser::extract_thumbnail_extension("File.png"),
            Some("File.png".to_string())
        );
    }

    #[test]
    fn test_convert_thumbnail_jpg() {
        let html = Html::parse_document(
            r#"
            <html>
                <body>
                    <img src="/images/thumb/7/79/Cemetery_area_screenshot.jpg/300px-Cemetery_area_screenshot.jpg" alt="Screenshot" />
                </body>
            </html>
        "#,
        );

        let document = html;
        let img_selector = scraper::Selector::parse("img").unwrap();
        let img = document.select(&img_selector).next().unwrap();

        let url =
            "/images/thumb/7/79/Cemetery_area_screenshot.jpg/300px-Cemetery_area_screenshot.jpg";
        let full = ImageUrlParser::convert_thumbnail_to_full(url, &img);

        // JPG files should have thumb removed to get full resolution
        assert_eq!(full, "/images/7/79/Cemetery_area_screenshot.jpg");
    }

    #[test]
    fn test_convert_thumbnail_webp_with_data_width() {
        let html = Html::parse_document(
            r#"
            <html>
                <body>
                    <img src="/images/thumb/b/bc/Test.webp/250px-Test.webp.png"
                         data-file-width="800"
                         data-file-height="600" />
                </body>
            </html>
        "#,
        );

        let document = html;
        let img_selector = scraper::Selector::parse("img").unwrap();
        let img = document.select(&img_selector).next().unwrap();

        let url = "/images/thumb/b/bc/Test.webp/250px-Test.webp.png";
        let full = ImageUrlParser::convert_thumbnail_to_full(url, &img);

        // Should use the data-file-width for maximum thumbnail size
        assert_eq!(full, "/images/thumb/b/bc/Test.webp/800px-Test.webp.png");
    }

    #[test]
    fn test_convert_non_thumbnail_url() {
        let html = Html::parse_document(
            r#"
            <html>
                <body>
                    <img src="/images/area_screenshot.jpg" />
                </body>
            </html>
        "#,
        );

        let document = html;
        let img_selector = scraper::Selector::parse("img").unwrap();
        let img = document.select(&img_selector).next().unwrap();

        let url = "/images/area_screenshot.jpg";
        let result = ImageUrlParser::convert_thumbnail_to_full(url, &img);

        assert_eq!(result, url);
    }

    #[test]
    fn test_parse_prioritizes_area_screenshot_over_generic_screenshot() {
        let html = Html::parse_document(
            r#"
            <html>
                <body>
                    <img src="/images/generic_screenshot.jpg" alt="Generic" />
                    <img src="/images/zone_area_screenshot.jpg" alt="Zone Screenshot" />
                </body>
            </html>
        "#,
        );

        let image_url = ImageUrlParser::parse(&html);
        assert!(image_url.is_some());
        let url = image_url.unwrap();

        // Should prioritize area_screenshot
        assert!(url.contains("area_screenshot"));
    }

    #[test]
    fn test_webp_without_data_width_returns_as_is() {
        let html = Html::parse_document(
            r#"
            <html>
                <body>
                    <img src="/images/thumb/b/bc/Test.webp/250px-Test.webp.png" alt="Screenshot" />
                </body>
            </html>
        "#,
        );

        let document = html;
        let img_selector = scraper::Selector::parse("img").unwrap();
        let img = document.select(&img_selector).next().unwrap();

        let url = "/images/thumb/b/bc/Test.webp/250px-Test.webp.png";
        let result = ImageUrlParser::convert_thumbnail_to_full(url, &img);

        // Without data-file-width, should return the URL as-is
        assert_eq!(result, url);
    }
}
