#[cfg(test)]
mod tests {
    use crate::domain::wiki_scraping::parsers::is_town_parser::*;
    use scraper::Html;

    fn make_doc(subheading: &str, icon_title: &str, zone_id: &str) -> Html {
        Html::parse_document(&format!(
            r#"<html><body>
            <div class="info-card">
              <div class="info-card__header">
                <div class="middle">
                  <div class="heading">Test Zone</div>
                  <div class="subheading">{subheading}</div>
                </div>
                <div class="right"><span title="{icon_title}"></span></div>
              </div>
              <div class="info-card__body">
                <div class="block"><table><tbody>
                  <tr><th>Id</th><td>{zone_id}</td></tr>
                  <tr><th>Act</th><td>1</td></tr>
                  <tr><th>Area level</th><td>1</td></tr>
                </tbody></table></div>
              </div>
            </div>
            </body></html>"#,
        ))
    }

    #[test]
    fn test_parse_town_area_subheading() {
        let html = make_doc("Town area", "Town Hub", "clearfell_encampment");
        assert!(IsTownParser::parse(&html));
    }

    #[test]
    fn test_parse_town_hub_icon_fallback() {
        // Icon is "Town Hub" but subheading doesn't say "Town area"
        let html = make_doc("area", "Town Hub", "some_zone");
        assert!(IsTownParser::parse(&html));
    }

    #[test]
    fn test_parse_town_id_fallback() {
        // Id contains "_town" but no other signals
        let html = make_doc("area", "Waypoint", "clearfell_town");
        assert!(IsTownParser::parse(&html));
    }

    #[test]
    fn test_parse_not_a_town() {
        let html = make_doc("area", "Waypoint", "clearfell_zone_1");
        assert!(!IsTownParser::parse(&html));
    }

    #[test]
    fn test_parse_no_info_card() {
        let html = Html::parse_document(r#"<html><body><p>No info card</p></body></html>"#);
        assert!(!IsTownParser::parse(&html));
    }
}
