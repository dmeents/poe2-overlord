#[cfg(test)]
mod tests {
    use crate::domain::wiki_scraping::parsers::zone_type_parser::ZoneTypeParser;
    use crate::domain::zone_configuration::models::ZoneType;
    use scraper::Html;

    fn make_doc(subheading: &str, icon_title: &str) -> Html {
        Html::parse_document(&format!(
            r#"<html><body>
            <div class="info-card">
              <div class="info-card__card">
                <div class="info-card__header">
                  <div class="middle">
                    <div class="heading">Test Zone</div>
                    <div class="subheading">{subheading}</div>
                  </div>
                  <div class="right">
                    <span title="{icon_title}"></span>
                  </div>
                </div>
                <div class="info-card__body">
                  <div class="block">
                    <table><tbody>
                      <tr><th>Act</th><td>1</td></tr>
                      <tr><th>Area level</th><td>5</td></tr>
                    </tbody></table>
                  </div>
                </div>
              </div>
            </div>
            </body></html>"#,
        ))
    }

    #[test]
    fn test_zone_type_campaign() {
        let doc = make_doc("area", "Waypoint");
        assert_eq!(ZoneTypeParser::parse(&doc), ZoneType::Campaign);
    }

    #[test]
    fn test_zone_type_town() {
        let doc = make_doc("Town area", "Town Hub");
        assert_eq!(ZoneTypeParser::parse(&doc), ZoneType::Town);
    }

    #[test]
    fn test_zone_type_map() {
        let doc = make_doc("Map area", "No Waypoint");
        assert_eq!(ZoneTypeParser::parse(&doc), ZoneType::Map);
    }

    #[test]
    fn test_zone_type_hideout() {
        let doc = make_doc("Hideout area", "No Waypoint");
        assert_eq!(ZoneTypeParser::parse(&doc), ZoneType::Hideout);
    }

    #[test]
    fn test_zone_type_unknown_no_info_card() {
        let doc = Html::parse_document("<html><body><p>No info card here</p></body></html>");
        assert_eq!(ZoneTypeParser::parse(&doc), ZoneType::Unknown);
    }

    #[test]
    fn test_zone_type_skips_tooltip_card() {
        let doc = Html::parse_document(
            r#"<html><body>
            <div class="info-card">
              <div class="info-card__header">
                <div class="middle"><div class="subheading">Tooltip</div></div>
              </div>
            </div>
            <div class="info-card">
              <div class="info-card__header">
                <div class="middle">
                  <div class="subheading">Map area</div>
                </div>
              </div>
            </div>
            </body></html>"#,
        );
        assert_eq!(ZoneTypeParser::parse(&doc), ZoneType::Map);
    }
}
