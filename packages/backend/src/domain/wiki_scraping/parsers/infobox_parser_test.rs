#[cfg(test)]
mod tests {
    use crate::domain::wiki_scraping::parsers::infobox_parser::*;
    use scraper::Html;

    fn make_info_card_doc(subheading: &str, extra_rows: &str) -> Html {
        Html::parse_document(&format!(
            r#"<html><body>
            <div class="info-card">
              <div class="info-card__card">
                <div class="info-card__header">
                  <div class="middle">
                    <div class="heading">Test Zone</div>
                    <div class="subheading">{subheading}</div>
                  </div>
                  <div class="right"><span title="Waypoint"></span></div>
                </div>
                <div class="info-card__body">
                  <div class="block">
                    <table><tbody>
                      <tr><th>Id</th><td>test_zone_1</td></tr>
                      <tr><th>Act</th><td>1</td></tr>
                      <tr><th>Area level</th><td>10</td></tr>
                      {extra_rows}
                    </tbody></table>
                  </div>
                </div>
              </div>
            </div>
            </body></html>"#,
        ))
    }

    #[test]
    fn test_is_redirect_page_via_title() {
        let redirect_html = Html::parse_document(
            r"<html><head><title>Redirect - Page</title></head><body></body></html>",
        );
        assert!(InfoboxParser::is_redirect_page(&redirect_html));

        let normal_html = Html::parse_document(
            r"<html><head><title>Normal Page</title></head><body></body></html>",
        );
        assert!(!InfoboxParser::is_redirect_page(&normal_html));
    }

    #[test]
    fn test_is_redirect_page_via_redirect_div() {
        let html = Html::parse_document(
            r#"<html><body><div class="redirectMsg">This page redirects</div></body></html>"#,
        );
        assert!(InfoboxParser::is_redirect_page(&html));
    }

    #[test]
    fn test_extract_campaign_zone() {
        let doc = make_info_card_doc("area", "");
        let result = InfoboxParser::extract(&doc);
        assert!(result.is_some());
    }

    #[test]
    fn test_extract_town_zone() {
        let doc = make_info_card_doc("Town area", "");
        let result = InfoboxParser::extract(&doc);
        assert!(result.is_some());
    }

    #[test]
    fn test_extract_map_zone() {
        let doc = make_info_card_doc("Map area", "");
        let result = InfoboxParser::extract(&doc);
        assert!(result.is_some());
    }

    #[test]
    fn test_extract_hideout_selects_first_non_tooltip_card() {
        // Hideout pages have two info-cards: the hideout area card and a map variant
        let html = Html::parse_document(
            r#"<html><body>
            <div class="info-card">
              <div class="info-card__header">
                <div class="middle">
                  <div class="heading">Felled Hideout</div>
                  <div class="subheading">Hideout area</div>
                </div>
              </div>
              <div class="info-card__body"><div class="block"><table><tbody>
                <tr><th>Id</th><td>Hideout_Felled</td></tr>
                <tr><th>Act</th><td>1</td></tr>
                <tr><th>Area level</th><td>1</td></tr>
              </tbody></table></div></div>
            </div>
            <div class="info-card">
              <div class="info-card__header">
                <div class="middle">
                  <div class="heading">Felled Hideout (Map)</div>
                  <div class="subheading">Map area</div>
                </div>
              </div>
              <div class="info-card__body"><div class="block"><table><tbody>
                <tr><th>Id</th><td>Map_Hideout_Felled</td></tr>
                <tr><th>Act</th><td>10</td></tr>
                <tr><th>Area level</th><td>80</td></tr>
              </tbody></table></div></div>
            </div>
            </body></html>"#,
        );
        let result = InfoboxParser::extract(&html);
        assert!(result.is_some());
        // Should have selected the hideout area card (Id = Hideout_Felled), not the map
        let infobox = result.unwrap();
        use crate::domain::wiki_scraping::parsers::base::BaseParser;
        let id_val = BaseParser::extract_table_value(&infobox, "Id");
        assert_eq!(id_val, Some("Hideout_Felled".to_string()));
    }

    #[test]
    fn test_extract_skips_tooltip_cards() {
        let html = Html::parse_document(
            r#"<html><body>
            <div class="info-card">
              <div class="info-card__header">
                <div class="middle"><div class="subheading">Tooltip</div></div>
              </div>
              <div class="info-card__body"><div class="block"><table><tbody>
                <tr><th>Act</th><td>99</td></tr>
                <tr><th>Area level</th><td>99</td></tr>
              </tbody></table></div></div>
            </div>
            <div class="info-card">
              <div class="info-card__header">
                <div class="middle"><div class="subheading">area</div></div>
              </div>
              <div class="info-card__body"><div class="block"><table><tbody>
                <tr><th>Id</th><td>real_zone</td></tr>
                <tr><th>Act</th><td>3</td></tr>
                <tr><th>Area level</th><td>45</td></tr>
              </tbody></table></div></div>
            </div>
            </body></html>"#,
        );
        let result = InfoboxParser::extract(&html);
        assert!(result.is_some());
        use crate::domain::wiki_scraping::parsers::base::BaseParser;
        let act_val = BaseParser::extract_table_value(&result.unwrap(), "Act");
        assert_eq!(act_val, Some("3".to_string()));
    }

    #[test]
    fn test_extract_no_info_card() {
        let html = Html::parse_document(r"<html><body><p>No info card here</p></body></html>");
        let result = InfoboxParser::extract(&html);
        assert!(result.is_none());
    }
}
