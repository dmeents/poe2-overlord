use log::{debug, info};
use scraper::{Html, Selector};

pub struct InfoboxParser;

impl InfoboxParser {
    /// Extracts the infobox table from the document
    pub fn extract(document: &Html) -> Option<Html> {
        let selectors = [
            "table.infobox",
            "table.wikitable",
            "table[class*='infobox']",
            "table[class*='wikitable']",
            "table.responsive-table",
            "table",
        ];

        for selector_str in &selectors {
            debug!("InfoboxParser: Trying selector '{}'", selector_str);

            if let Ok(selector) = Selector::parse(selector_str) {
                for element in document.select(&selector) {
                    let html = element.html();

                    if Self::is_valid_zone_infobox(&html) {
                        info!(
                            "InfoboxParser: Found valid infobox using selector '{}'",
                            selector_str
                        );
                        return Some(Html::parse_fragment(&html));
                    }
                }
            }
        }

        debug!("InfoboxParser: No valid infobox found");
        None
    }

    pub fn is_redirect_page(document: &Html) -> bool {
        let title_selector = Selector::parse("title").unwrap();
        if let Some(title) = document.select(&title_selector).next() {
            let title_text = title.text().collect::<String>();
            let is_redirect = title_text.contains("redirect") || title_text.contains("Redirect");

            if is_redirect {
                debug!("InfoboxParser: Detected redirect page");
            }

            return is_redirect;
        }
        false
    }

    fn is_valid_zone_infobox(html: &str) -> bool {
        let zone_indicators = ["Act", "Area level", "Id", "Connections", "Waypoint"];

        let matches = zone_indicators
            .iter()
            .filter(|indicator| html.contains(*indicator))
            .count();

        matches >= 2
    }
}
