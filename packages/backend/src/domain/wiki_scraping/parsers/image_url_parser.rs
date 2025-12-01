use crate::domain::wiki_scraping::url_utils;
use scraper::{Html, Selector};

pub struct ImageUrlParser;

impl ImageUrlParser {
    pub fn parse(document: &Html) -> Option<String> {
        // Strategy 1: Look for area_screenshot images (most reliable)
        if let Some(url) = Self::find_area_screenshot(document) {
            return Some(url);
        }

        // Strategy 2: Look for any screenshot image
        if let Some(url) = Self::find_screenshot(document) {
            return Some(url);
        }

        None
    }

    fn find_area_screenshot(document: &Html) -> Option<String> {
        let img_selector = Selector::parse("img").unwrap();

        for img in document.select(&img_selector) {
            if let Some(src) = img.value().attr("src") {
                if src.contains("area_screenshot") {
                    let full_url = Self::convert_thumbnail_to_full(src, &img);
                    let absolute_url = url_utils::to_absolute_url(&full_url);
                    return Some(absolute_url);
                }
            }
        }

        None
    }

    fn find_screenshot(document: &Html) -> Option<String> {
        let img_selector = Selector::parse("img").unwrap();

        for img in document.select(&img_selector) {
            if let Some(src) = img.value().attr("src") {
                // Skip icons and UI elements
                if src.contains("icon") || src.contains("waypoint_area") {
                    continue;
                }

                if src.contains("screenshot") {
                    let full_url = Self::convert_thumbnail_to_full(src, &img);
                    let absolute_url = url_utils::to_absolute_url(&full_url);
                    return Some(absolute_url);
                }
            }
        }

        None
    }

    pub(crate) fn convert_thumbnail_to_full(
        url: &str,
        element: &scraper::element_ref::ElementRef,
    ) -> String {
        // Handle MediaWiki thumbnail URLs
        // For webp files that are served as PNG conversions, use the largest available thumbnail
        // based on data-file-width attribute
        //
        // Example:
        // /images/thumb/b/bc/File.webp/250px-File.webp.png with data-file-width="589"
        // Should become:
        // /images/thumb/b/bc/File.webp/589px-File.webp.png

        if !url.contains("/thumb/") {
            return url.to_string();
        }

        // Check if this is a webp file being served as png (common for hideouts)
        if url.contains(".webp") && url.ends_with(".png") {
            // Try to get the original file width from data-file-width attribute
            if let Some(file_width) = element.value().attr("data-file-width") {
                if let Ok(width) = file_width.parse::<u32>() {
                    // Construct the largest thumbnail URL
                    if let Some(thumb_pos) = url.find("/thumb/") {
                        let before_thumb = &url[..thumb_pos];
                        let after_thumb = &url[thumb_pos + 7..];

                        if let Some(last_slash) = after_thumb.rfind('/') {
                            let file_path = &after_thumb[..last_slash];

                            // Extract the extension pattern from the thumbnail name
                            let thumbnail_name = &after_thumb[last_slash + 1..];
                            if let Some(extension_part) =
                                Self::extract_thumbnail_extension(thumbnail_name)
                            {
                                return format!(
                                    "{}/thumb/{}/{}px-{}",
                                    before_thumb, file_path, width, extension_part
                                );
                            }
                        }
                    }
                }
            }
        }

        // For standard jpg/png thumbnails, try to get full resolution
        if let Some(thumb_pos) = url.find("/thumb/") {
            let base = &url[..thumb_pos];
            let after_thumb = &url[thumb_pos + 7..];

            if let Some(last_slash) = after_thumb.rfind('/') {
                let original_path = &after_thumb[..last_slash];

                // If the path ends with a standard extension, use it as full path
                if original_path.ends_with(".jpg")
                    || original_path.ends_with(".jpeg")
                    || original_path.ends_with(".png")
                    || original_path.ends_with(".gif")
                {
                    return format!("{}/{}", base, original_path);
                }
            }
        }

        url.to_string()
    }

    pub(crate) fn extract_thumbnail_extension(filename: &str) -> Option<String> {
        // Extract the extension pattern from thumbnail filename
        // e.g., "250px-File.webp.png" -> "File.webp.png"
        // e.g., "300px-Image.jpg" -> "Image.jpg"

        if let Some(dash_pos) = filename.find('-') {
            let prefix = &filename[..dash_pos];
            // Check if prefix is a size indicator (ends with "px")
            if prefix.ends_with("px") {
                return Some(filename[dash_pos + 1..].to_string());
            }
        }

        Some(filename.to_string())
    }
}
