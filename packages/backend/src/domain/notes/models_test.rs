#[cfg(test)]
mod tests {
    use chrono::Utc;

    use crate::domain::notes::models::{CreateNoteParams, NoteData, UpdateNoteParams};

    fn make_note() -> NoteData {
        let now = Utc::now();
        NoteData {
            id: "note-1".to_string(),
            title: "Test Note".to_string(),
            content: "## Hello\n\n- item 1\n- item 2".to_string(),
            is_pinned: false,
            character_id: None,
            created_at: now,
            updated_at: now,
        }
    }

    // ============= NoteData Tests =============

    #[test]
    fn test_note_data_fields() {
        let note = make_note();

        assert_eq!(note.id, "note-1");
        assert_eq!(note.title, "Test Note");
        assert!(!note.is_pinned);
        assert!(note.character_id.is_none());
    }

    #[test]
    fn test_note_data_with_character_id() {
        let mut note = make_note();
        note.character_id = Some("char-1".to_string());

        assert_eq!(note.character_id, Some("char-1".to_string()));
    }

    #[test]
    fn test_note_data_pinned() {
        let mut note = make_note();
        assert!(!note.is_pinned);

        note.is_pinned = true;
        assert!(note.is_pinned);
    }

    #[test]
    fn test_note_data_serialization_roundtrip() {
        let note = make_note();

        let json = serde_json::to_string(&note).unwrap();
        let deserialized: NoteData = serde_json::from_str(&json).unwrap();

        assert_eq!(deserialized.id, note.id);
        assert_eq!(deserialized.title, note.title);
        assert_eq!(deserialized.content, note.content);
        assert_eq!(deserialized.is_pinned, note.is_pinned);
        assert_eq!(deserialized.character_id, note.character_id);
    }

    #[test]
    fn test_note_data_serialization_with_character_id() {
        let mut note = make_note();
        note.character_id = Some("char-abc".to_string());
        note.is_pinned = true;

        let json = serde_json::to_string(&note).unwrap();
        let deserialized: NoteData = serde_json::from_str(&json).unwrap();

        assert_eq!(deserialized.character_id, Some("char-abc".to_string()));
        assert!(deserialized.is_pinned);
    }

    #[test]
    fn test_note_data_null_character_id_serialization() {
        let note = make_note();
        let json = serde_json::to_string(&note).unwrap();

        // Ensure null character_id serializes correctly
        assert!(json.contains("\"character_id\":null"));
    }

    #[test]
    fn test_note_data_clone() {
        let note = make_note();
        let cloned = note.clone();

        assert_eq!(cloned.id, note.id);
        assert_eq!(cloned.title, note.title);
        assert_eq!(cloned.content, note.content);
        assert_eq!(cloned.is_pinned, note.is_pinned);
    }

    #[test]
    fn test_note_data_equality() {
        let note1 = make_note();
        let note2 = note1.clone();

        assert_eq!(note1, note2);
    }

    // ============= CreateNoteParams Tests =============

    #[test]
    fn test_create_note_params_serialization() {
        let params = CreateNoteParams {
            title: "My Note".to_string(),
            content: "Some content".to_string(),
            character_id: None,
        };

        let json = serde_json::to_string(&params).unwrap();
        let deserialized: CreateNoteParams = serde_json::from_str(&json).unwrap();

        assert_eq!(deserialized.title, "My Note");
        assert_eq!(deserialized.content, "Some content");
        assert!(deserialized.character_id.is_none());
    }

    #[test]
    fn test_create_note_params_with_character_id() {
        let params = CreateNoteParams {
            title: "Character Note".to_string(),
            content: String::new(),
            character_id: Some("char-1".to_string()),
        };

        let json = serde_json::to_string(&params).unwrap();
        let deserialized: CreateNoteParams = serde_json::from_str(&json).unwrap();

        assert_eq!(deserialized.character_id, Some("char-1".to_string()));
    }

    // ============= UpdateNoteParams Tests =============

    #[test]
    fn test_update_note_params_serialization() {
        let params = UpdateNoteParams {
            title: "Updated Title".to_string(),
            content: "Updated content".to_string(),
            character_id: Some("char-2".to_string()),
        };

        let json = serde_json::to_string(&params).unwrap();
        let deserialized: UpdateNoteParams = serde_json::from_str(&json).unwrap();

        assert_eq!(deserialized.title, "Updated Title");
        assert_eq!(deserialized.content, "Updated content");
        assert_eq!(deserialized.character_id, Some("char-2".to_string()));
    }

    #[test]
    fn test_update_note_params_clear_character_id() {
        let params = UpdateNoteParams {
            title: "Note".to_string(),
            content: String::new(),
            character_id: None,
        };

        let json = serde_json::to_string(&params).unwrap();
        let deserialized: UpdateNoteParams = serde_json::from_str(&json).unwrap();

        assert!(deserialized.character_id.is_none());
    }
}
