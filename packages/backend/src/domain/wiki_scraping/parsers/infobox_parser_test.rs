#[cfg(test)]
mod tests {
    use crate::domain::wiki_scraping::parsers::infobox_parser::*;
    use scraper::Html;

    #[test]
    fn test_is_redirect_page() {
        let redirect_html = Html::parse_document(
            r#"
            <html>
                <head><title>Redirect - Page</title></head>
                <body></body>
            </html>
        "#,
        );

        assert!(InfoboxParser::is_redirect_page(&redirect_html));

        let normal_html = Html::parse_document(
            r#"
            <html>
                <head><title>Normal Page</title></head>
                <body></body>
            </html>
        "#,
        );

        assert!(!InfoboxParser::is_redirect_page(&normal_html));
    }

    #[test]
    fn test_extract_infobox() {
        let html_with_infobox = Html::parse_document(
            r#"
            <html>
                <body>
                    <table class="infobox">
                        <tr><td>Act</td><td>1</td></tr>
                        <tr><td>Area level</td><td>42</td></tr>
                        <tr><td>Id</td><td>zone_1</td></tr>
                    </table>
                </body>
            </html>
        "#,
        );

        let result = InfoboxParser::extract(&html_with_infobox);
        assert!(result.is_some());
    }

    #[test]
    fn test_extract_no_infobox() {
        let html_without_infobox = Html::parse_document(
            r#"
            <html>
                <body>
                    <table class="other">
                        <tr><td>Name</td><td>Value</td></tr>
                    </table>
                </body>
            </html>
        "#,
        );

        let result = InfoboxParser::extract(&html_without_infobox);
        assert!(result.is_none());
    }
}
