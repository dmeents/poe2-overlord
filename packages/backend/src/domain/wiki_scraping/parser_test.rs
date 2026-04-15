#[cfg(test)]
mod tests {
    use crate::domain::wiki_scraping::parser::*;

    #[test]
    fn test_parse_zone_data_basic() {
        let html = r#"
            <html>
                <head><title>Test Zone</title></head>
                <body>
                    <div class="info-card">
                      <div class="info-card__header">
                        <div class="middle">
                          <div class="heading">Test Zone</div>
                          <div class="subheading">area</div>
                        </div>
                        <div class="right"><span title="Waypoint"></span></div>
                      </div>
                      <div class="info-card__body">
                        <div class="block"><table><tbody>
                          <tr><th>Id</th><td>test_zone_1</td></tr>
                          <tr><th>Act</th><td>1</td></tr>
                          <tr><th>Area level</th><td>10</td></tr>
                        </tbody></table></div>
                      </div>
                    </div>
                    <p>Zone 2, Zone 3</p>
                </body>
            </html>
        "#;

        let result = WikiParser::parse_zone_data("Test Zone", html, "https://wiki.test");
        assert!(result.is_ok());

        let zone_data = result.unwrap();
        assert_eq!(zone_data.zone_name, "Test Zone");
        assert_eq!(zone_data.act, 1);
        assert_eq!(zone_data.area_level, Some(10));
        assert!(zone_data.has_waypoint);
    }

    #[test]
    fn test_parse_zone_data_redirect() {
        let html = r"
            <html>
                <head><title>Redirect - Test</title></head>
                <body></body>
            </html>
        ";

        let result = WikiParser::parse_zone_data("Test Zone", html, "https://wiki.test");
        assert!(result.is_err());
    }

    #[test]
    fn test_parse_zone_data_no_infobox() {
        let html = r"
            <html>
                <head><title>Test Zone</title></head>
                <body>
                    <p>Some content without an infobox</p>
                </body>
            </html>
        ";

        let result = WikiParser::parse_zone_data("Test Zone", html, "https://wiki.test");
        assert!(result.is_ok());

        let zone_data = result.unwrap();
        assert_eq!(zone_data.zone_name, "Test Zone");
        // Default value when no infobox is found
        assert_eq!(zone_data.act, 0);
    }

    #[test]
    fn test_parse_zone_data_complete_example() {
        let html = r#"
            <html>
                <head><title>Cemetery of the Eternals</title></head>
                <body>
                    <div class="info-card">
                      <div class="info-card__header">
                        <div class="middle">
                          <div class="heading">Cemetery of the Eternals</div>
                          <div class="subheading">area</div>
                        </div>
                        <div class="right"></div>
                      </div>
                      <div class="info-card__body">
                        <div class="block">
                          <em class="flavour">The bootprint of the Eternals upon the face of Ogham.</em>
                          <table><tbody>
                            <tr><th>Id</th><td>cemetery_of_the_eternals</td></tr>
                            <tr><th>Act</th><td>1</td></tr>
                            <tr><th>Area level</th><td>5</td></tr>
                          </tbody></table>
                        </div>
                      </div>
                    </div>
                    <h2>NPCs</h2>
                    <ul>
                        <li>Lachlann the Lost - Quest giver</li>
                    </ul>
                    <h2>Points of Interest</h2>
                    <ul>
                        <li>[Boss arena] Memorial of the Lost: Boss location</li>
                        <li>[Notable chest] Ancient Ruin: Contains loot</li>
                    </ul>
                    <h2>Connections</h2>
                    <ul>
                        <li><a>The Grim Tangle</a></li>
                        <li><a>Mausoleum of the Praetor</a></li>
                        <li><a>Tomb of the Consort</a></li>
                        <li><a>Hunting Grounds</a></li>
                    </ul>
                    <img src="https://www.poe2wiki.net/images/7/79/Cemetery_of_the_Eternals_area_screenshot.jpg" alt="Screenshot">
                </body>
            </html>
        "#;

        let result = WikiParser::parse_zone_data(
            "Cemetery of the Eternals",
            html,
            "https://www.poe2wiki.net/wiki/Cemetery_of_the_Eternals",
        );
        assert!(result.is_ok());

        let zone_data = result.unwrap();
        assert_eq!(zone_data.zone_name, "Cemetery of the Eternals");
        assert_eq!(zone_data.act, 1);
        assert_eq!(zone_data.npcs.len(), 1);
        assert_eq!(zone_data.npcs[0], "Lachlann the Lost");
        assert_eq!(zone_data.points_of_interest.len(), 2);
        assert!(zone_data.description.is_some());
        assert!(zone_data
            .description
            .unwrap()
            .contains("bootprint of the Eternals"));
        assert!(zone_data.image_url.is_some());
        assert!(zone_data
            .image_url
            .unwrap()
            .contains("Cemetery_of_the_Eternals_area_screenshot.jpg"));
    }

    #[test]
    fn test_parse_zone_data_town() {
        let html = r#"
            <html>
                <head><title>Clearfell Encampment</title></head>
                <body>
                    <div class="info-card">
                      <div class="info-card__header">
                        <div class="middle">
                          <div class="heading">Clearfell Encampment</div>
                          <div class="subheading">Town area</div>
                        </div>
                        <div class="right"><span title="Town Hub"></span></div>
                      </div>
                      <div class="info-card__body">
                        <div class="block"><table><tbody>
                          <tr><th>Id</th><td>clearfell_encampment</td></tr>
                          <tr><th>Act</th><td>1</td></tr>
                          <tr><th>Area level</th><td>1</td></tr>
                        </tbody></table></div>
                      </div>
                    </div>
                </body>
            </html>
        "#;

        let result = WikiParser::parse_zone_data(
            "Clearfell Encampment",
            html,
            "https://wiki.test/clearfell",
        );
        assert!(result.is_ok());

        let zone_data = result.unwrap();
        assert!(zone_data.is_town);
        assert!(zone_data.has_waypoint); // Town Hub implies waypoint
        use crate::domain::zone_configuration::models::ZoneType;
        assert!(matches!(zone_data.zone_type, ZoneType::Town));
    }
}
