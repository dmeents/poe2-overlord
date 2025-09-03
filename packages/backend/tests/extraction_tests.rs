use app_lib::parsers::utils::extraction::extract_content_between_delimiters;

#[test]
fn test_scene_extraction_with_bracket_in_pattern() {
    let test_line = "2025/09/03 22:40:27 246654700 775aed1e [INFO Client 320] [SCENE] Set Source [Clearfell Encampment]";
    let pattern = "[SCENE] Set Source [";

    let result = extract_content_between_delimiters(test_line, pattern, '[', ']');

    match result {
        Ok(content) => {
            assert_eq!(content, "Clearfell Encampment");
            println!("✅ Successfully extracted: '{}'", content);
        }
        Err(e) => {
            panic!("❌ Failed to extract content: {:?}", e);
        }
    }
}

#[test]
fn test_scene_extraction_with_load_source() {
    let test_line = "2025/09/03 22:40:27 246654700 775aed1e [INFO Client 320] [SCENE] Load Source [Town Square]";
    let pattern = "[SCENE] Load Source [";

    let result = extract_content_between_delimiters(test_line, pattern, '[', ']');

    match result {
        Ok(content) => {
            assert_eq!(content, "Town Square");
            println!("✅ Successfully extracted: '{}'", content);
        }
        Err(e) => {
            panic!("❌ Failed to extract content: {:?}", e);
        }
    }
}

#[test]
fn test_scene_extraction_with_multiple_brackets() {
    let test_line = "2025/09/03 22:40:27 246654700 775aed1e [INFO Client 320] [SCENE] Set Source [Act 1 - The Coast]";
    let pattern = "[SCENE] Set Source [";

    let result = extract_content_between_delimiters(test_line, pattern, '[', ']');

    match result {
        Ok(content) => {
            assert_eq!(content, "Act 1 - The Coast");
            println!("✅ Successfully extracted: '{}'", content);
        }
        Err(e) => {
            panic!("❌ Failed to extract content: {:?}", e);
        }
    }
}

#[test]
fn test_scene_extraction_with_hideout() {
    let test_line =
        "2025/09/03 22:40:27 246654700 775aed1e [INFO Client 320] [SCENE] Set Source [hideout]";
    let pattern = "[SCENE] Set Source [";

    let result = extract_content_between_delimiters(test_line, pattern, '[', ']');

    match result {
        Ok(content) => {
            assert_eq!(content, "hideout");
            println!("✅ Successfully extracted: '{}'", content);
        }
        Err(e) => {
            panic!("❌ Failed to extract content: {:?}", e);
        }
    }
}
