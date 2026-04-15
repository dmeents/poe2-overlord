use log::debug;

const BASE_URL: &str = "https://www.poe2wiki.net";
const WIKI_PATH: &str = "/wiki";

/// Formats a zone name for use in wiki URLs
pub fn format_zone_name_for_url(zone_name: &str) -> String {
    zone_name
        .replace(' ', "_")
        .replace('\'', "%27")
        .replace('-', "_")
}

/// Constructs the full wiki URL for a zone
pub fn get_wiki_url(zone_name: &str) -> String {
    let formatted_name = format_zone_name_for_url(zone_name);
    format!("{BASE_URL}{WIKI_PATH}/{formatted_name}")
}

/// Converts a relative URL to an absolute URL for the `PoE2` wiki
pub fn to_absolute_url(url: &str) -> String {
    if url.starts_with("http") {
        url.to_string()
    } else if url.starts_with("//") {
        format!("https:{url}")
    } else {
        format!("{BASE_URL}{url}")
    }
}

/// Checks if zone data should be refreshed based on last update time
pub fn should_refresh(last_updated: chrono::DateTime<chrono::Utc>) -> bool {
    let now = chrono::Utc::now();
    let week_ago = now - chrono::Duration::weeks(1);
    let should = last_updated < week_ago;

    if should {
        debug!("Zone data last updated {last_updated} is older than one week, should refresh");
    }

    should
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_format_zone_name_for_url() {
        assert_eq!(
            format_zone_name_for_url("The Tidal Island"),
            "The_Tidal_Island"
        );
        assert_eq!(format_zone_name_for_url("Una's Stash"), "Una%27s_Stash");
        assert_eq!(format_zone_name_for_url("Town-Area"), "Town_Area");
    }

    #[test]
    fn test_get_wiki_url() {
        assert_eq!(
            get_wiki_url("The Tidal Island"),
            "https://www.poe2wiki.net/wiki/The_Tidal_Island"
        );
    }

    #[test]
    fn test_to_absolute_url() {
        assert_eq!(to_absolute_url("http://example.com"), "http://example.com");
        assert_eq!(
            to_absolute_url("//example.com/image.jpg"),
            "https://example.com/image.jpg"
        );
        assert_eq!(
            to_absolute_url("/images/test.jpg"),
            "https://www.poe2wiki.net/images/test.jpg"
        );
    }
}
