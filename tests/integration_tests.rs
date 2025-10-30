// Integration tests for leaderboard functionality

#[test]
fn test_leaderboard_data_structures() {
    // Test that leaderboard data structures work correctly
    use typeman::leaderboard::{LeaderboardEntry, TestType};
    use typeman::language::Language;
    
    let entry = LeaderboardEntry {
        wpm: 85.5,
        accuracy: 98.2,
        test_type: TestType::Time(30),
        test_mode: "time".to_string(),
        word_count: 145,
        test_duration: 30.0,
        timestamp: "2025-09-11T10:30:00Z".to_string(),
        language: Language::English,
    };
    
    // Verify the entry was created successfully
    assert_eq!(entry.wpm, 85.5);
    assert_eq!(entry.accuracy, 98.2);
    assert!(matches!(entry.test_type, TestType::Time(30)));
}

#[test]
fn test_leaderboard_storage() {
    // Test that leaderboard storage and retrieval works
    use typeman::leaderboard::{save_entry, load_entries, LeaderboardEntry, TestType};
    use typeman::language::Language;
    
    let entry = LeaderboardEntry {
        wpm: 75.0,
        accuracy: 95.0,
        test_type: TestType::Word(50),
        test_mode: "word".to_string(),
        word_count: 50,
        test_duration: 45.0,
        timestamp: "2025-09-11T11:00:00Z".to_string(),
        language: Language::English,
    };
    
    // Should be able to save and load entries
    assert!(save_entry(&entry).is_ok());
    assert!(load_entries().is_ok());
}

#[test]
fn test_app_state_integration() {
    // Test that the App struct includes leaderboard fields
    use typeman::ui::tui::app::App;
    
    let app = App::new();
    
    // Verify leaderboard fields exist and are initialized
    assert!(!app.leaderboard.open); // Should start closed
    assert_eq!(app.leaderboard.selected, 0); // Should start at first entry
    // leaderboard_entries should be a vector (length >= 0)
    //assert!(app.leaderboard.entries.len() >= 0);
}

#[test]
fn test_test_type_variants() {
    // Test all TestType variants
    use typeman::leaderboard::TestType;
    
    let time_test = TestType::Time(60);
    let word_test = TestType::Word(100);
    let quote_test = TestType::Quote;
    let practice_test = TestType::Practice(5);
    
    // Verify variants can be created
    assert!(matches!(time_test, TestType::Time(60)));
    assert!(matches!(word_test, TestType::Word(100)));
    assert!(matches!(quote_test, TestType::Quote));
    assert!(matches!(practice_test, TestType::Practice(5)));
}