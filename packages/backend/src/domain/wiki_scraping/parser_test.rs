#[cfg(test)]
mod tests {
    use crate::domain::wiki_scraping::parser::*;

    #[test]
    fn test_parse_zone_data_basic() {
        let html = r#"
            <html>
                <head><title>Test Zone</title></head>
                <body>
                    <table class="infobox">
                        <tr><td>Id</td><td>test_zone_1</td></tr>
                        <tr><td>Act</td><td>1</td></tr>
                        <tr><td>Area level</td><td>10</td></tr>
                        <tr><td>Waypoint</td><td>Yes</td></tr>
                        <tr><td>Connections</td><td><a>Zone 2</a>, <a>Zone 3</a></td></tr>
                    </table>
                    <h2>Monsters</h2>
                    <ul>
                        <li>Boss the Destroyer</li>
                        <li>Regular Monster</li>
                    </ul>
                </body>
            </html>
        "#;

        let result = WikiParser::parse_zone_data("Test Zone", html, "https://wiki.test");
        assert!(result.is_ok());

        let zone_data = result.unwrap();
        assert_eq!(zone_data.zone_name, "Test Zone");
        assert_eq!(zone_data.area_id, Some("test_zone_1".to_string()));
        assert_eq!(zone_data.act, 1);
        assert_eq!(zone_data.area_level, Some(10));
        assert!(zone_data.has_waypoint);
        assert_eq!(zone_data.connected_zones.len(), 2);
    }

    #[test]
    fn test_parse_zone_data_redirect() {
        let html = r#"
            <html>
                <head><title>Redirect - Test</title></head>
                <body></body>
            </html>
        "#;

        let result = WikiParser::parse_zone_data("Test Zone", html, "https://wiki.test");
        assert!(result.is_err());
    }

    #[test]
    fn test_parse_zone_data_no_infobox() {
        let html = r#"
            <html>
                <head><title>Test Zone</title></head>
                <body>
                    <p>Some content without an infobox</p>
                </body>
            </html>
        "#;

        let result = WikiParser::parse_zone_data("Test Zone", html, "https://wiki.test");
        assert!(result.is_ok());

        let zone_data = result.unwrap();
        assert_eq!(zone_data.zone_name, "Test Zone");
        // Should have default values when no infobox is found
        assert_eq!(zone_data.area_id, None);
        assert_eq!(zone_data.act, 0);
    }

    #[test]
    fn test_parse_zone_data_complete_example() {
        let html = r#"
            <html>
                <head><title>Cemetery of the Eternals</title></head>
                <body>
                    <p><i>The bootprint of the Eternals upon the face of Ogham.</i></p>
                    <table class="infobox">
                        <tr><td>Act</td><td>1</td></tr>
                        <tr><td>Connections</td><td>
                            <a>The Grim Tangle</a>,
                            <a>Mausoleum of the Praetor</a>,
                            <a>Tomb of the Consort</a>,
                            <a>Hunting Grounds</a>
                        </td></tr>
                    </table>
                    <h2>Monsters</h2>
                    <ul>
                        <li>Lachlann of Endless Lament</li>
                        <li>Burdened Wretch</li>
                        <li>Death Knight</li>
                    </ul>
                    <h2>NPCs</h2>
                    <ul>
                        <li>Lachlann the Lost - Quest giver</li>
                    </ul>
                    <h2>Points of Interest</h2>
                    <ul>
                        <li>[Boss arena] Memorial of the Lost: Boss location</li>
                        <li>[Notable chest] Ancient Ruin: Contains loot</li>
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
        assert_eq!(zone_data.connected_zones.len(), 4);
        assert_eq!(zone_data.monsters.len(), 3);
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
}
