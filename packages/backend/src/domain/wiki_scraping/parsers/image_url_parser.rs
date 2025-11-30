use crate::domain::wiki_scraping::url_utils;
use scraper::{Html, Selector};

pub struct ImageUrlParser;

impl ImageUrlParser {
    pub fn parse(document: &Html) -> Option<String> {
        let img_selector = Selector::parse("img").unwrap();

        for img in document.select(&img_selector) {
            if let Some(src) = img.value().attr("src") {
                if src.contains("area_screenshot") || src.contains("screenshot") {
                    let full_url = Self::convert_thumbnail_to_full(src);
                    let absolute_url = url_utils::to_absolute_url(&full_url);
                    return Some(absolute_url);
                }
            }
        }

        None
    }

    fn convert_thumbnail_to_full(url: &str) -> String {
        if url.contains("/thumb/") {
            if let Some(thumb_pos) = url.find("/thumb/") {
                let base = &url[..thumb_pos];
                let after_thumb = &url[thumb_pos + 7..];

                if let Some(last_slash) = after_thumb.rfind('/') {
                    let original_path = &after_thumb[..last_slash];
                    return format!("{}/{}", base, original_path);
                }
            }
        }

        url.to_string()
    }
}
