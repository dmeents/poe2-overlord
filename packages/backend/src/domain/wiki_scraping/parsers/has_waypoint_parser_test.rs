#[cfg(test)]
mod tests {
    use crate::domain::wiki_scraping::parsers::has_waypoint_parser::*;
    use scraper::Html;

    fn make_doc(icon_title: &str) -> Html {
        Html::parse_document(&format!(
            r#"<html><body>
            <div class="info-card">
              <div class="info-card__header">
                <div class="middle">
                  <div class="heading">Test Zone</div>
                  <div class="subheading">area</div>
                </div>
                <div class="right"><span title="{icon_title}"></span></div>
              </div>
              <div class="info-card__body">
                <div class="block"><table><tbody>
                  <tr><th>Id</th><td>test_zone_1</td></tr>
                  <tr><th>Act</th><td>1</td></tr>
                  <tr><th>Area level</th><td>10</td></tr>
                </tbody></table></div>
              </div>
            </div>
            </body></html>"#,
        ))
    }

    #[test]
    fn test_parse_has_waypoint_icon() {
        let html = make_doc("Waypoint");
        assert!(HasWaypointParser::parse(&html));
    }

    #[test]
    fn test_parse_town_hub_icon() {
        // Towns always have waypoints — "Town Hub" icon means waypoint
        let html = make_doc("Town Hub");
        assert!(HasWaypointParser::parse(&html));
    }

    #[test]
    fn test_parse_no_waypoint_icon() {
        let html = make_doc("No Waypoint");
        assert!(!HasWaypointParser::parse(&html));
    }

    #[test]
    fn test_parse_absent_icon() {
        // No span in .right — zone has no waypoint
        let html = Html::parse_document(
            r#"<html><body>
            <div class="info-card">
              <div class="info-card__header">
                <div class="middle">
                  <div class="heading">Test Zone</div>
                  <div class="subheading">area</div>
                </div>
                <div class="right"></div>
              </div>
              <div class="info-card__body">
                <div class="block"><table><tbody>
                  <tr><th>Id</th><td>no_waypoint_zone</td></tr>
                  <tr><th>Act</th><td>2</td></tr>
                  <tr><th>Area level</th><td>15</td></tr>
                </tbody></table></div>
              </div>
            </div>
            </body></html>"#,
        );
        assert!(!HasWaypointParser::parse(&html));
    }

    #[test]
    fn test_parse_no_info_card() {
        let html = Html::parse_document(r#"<html><body><p>No info card</p></body></html>"#);
        assert!(!HasWaypointParser::parse(&html));
    }
}
