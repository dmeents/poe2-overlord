use crate::domain::wiki_scraping::parsers::base::BaseParser;
use log::{debug, info};
use scraper::{Html, Selector};

pub struct InfoboxParser;

impl InfoboxParser {
    /// Extracts the infobox table from the document.
    ///
    /// The wiki uses `div.info-card` containers. On pages with multiple info-cards
    /// (hideouts, mechanic zones), the first non-"Tooltip" card is used.
    pub fn extract(document: &Html) -> Option<Html> {
        let card_html = BaseParser::find_primary_info_card_html(document)?;
        let card_doc = Html::parse_fragment(&card_html);

        let table_selector = Selector::parse("table").unwrap();
        for table in card_doc.select(&table_selector) {
            let table_html = table.html();
            if Self::is_valid_zone_infobox(&table_html) {
                info!("InfoboxParser: Found valid infobox in div.info-card");
                return Some(Html::parse_fragment(&table_html));
            }
        }

        debug!("InfoboxParser: No valid infobox found");
        None
    }

    pub fn is_redirect_page(document: &Html) -> bool {
        // Check page title
        let title_selector = Selector::parse("title").unwrap();
        if let Some(title) = document.select(&title_selector).next() {
            let title_text = title.text().collect::<String>();
            if title_text.contains("redirect") || title_text.contains("Redirect") {
                debug!("InfoboxParser: Detected redirect page via title");
                return true;
            }
        }

        // Check for #REDIRECT in first paragraph
        let p_selector = Selector::parse("p").unwrap();
        if let Some(first_p) = document.select(&p_selector).next() {
            let p_text = first_p.text().collect::<String>();
            if p_text.contains("#REDIRECT") || p_text.contains("#redirect") {
                debug!("InfoboxParser: Detected redirect page via #REDIRECT in <p>");
                return true;
            }
        }

        // Check for redirect div elements
        let redirect_selector = Selector::parse("div.redirectMsg, div.mw-redirect").unwrap();
        if document.select(&redirect_selector).next().is_some() {
            debug!("InfoboxParser: Detected redirect page via redirect div");
            return true;
        }

        false
    }

    /// A valid zone infobox must have at least 2 of the key field headers: Id, Act, Area level.
    fn is_valid_zone_infobox(html: &str) -> bool {
        let fragment = Html::parse_fragment(html);
        let th_selector = Selector::parse("th").unwrap();
        let key_fields = ["Id", "Act", "Area level"];

        let matches = key_fields
            .iter()
            .filter(|key| {
                fragment
                    .select(&th_selector)
                    .any(|th| BaseParser::extract_text(&th) == **key)
            })
            .count();

        matches >= 2
    }
}
