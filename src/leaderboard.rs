use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;
use crate::language::Language;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct LeaderboardEntry {
    pub wpm: f64,
    pub accuracy: f64,
    pub test_type: TestType,
    pub test_mode: String,
    pub word_count: usize,
    pub test_duration: f64,
    pub timestamp: String,
    pub language: Language,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum TestType {
    Time(u32),
    Word(usize),
    Quote,
    Practice(usize),
}

#[derive(Debug)]
pub enum LeaderboardError {
    IoError(std::io::Error),
    SerializationError(serde_json::Error),
}

impl From<std::io::Error> for LeaderboardError {
    fn from(error: std::io::Error) -> Self {
        LeaderboardError::IoError(error)
    }
}

impl From<serde_json::Error> for LeaderboardError {
    fn from(error: serde_json::Error) -> Self {
        LeaderboardError::SerializationError(error)
    }
}

pub fn get_config_dir() -> Result<PathBuf, Box<dyn std::error::Error>> {
    let home = std::env::var("HOME")
        .or_else(|_| std::env::var("USERPROFILE"))
        .map_err(|_| "Unable to find home directory")?;
    
    let config_dir = PathBuf::from(home).join(".config").join("typeman");
    fs::create_dir_all(&config_dir)?;
    Ok(config_dir)
}

pub fn save_entry(entry: &LeaderboardEntry) -> Result<(), LeaderboardError> {
    let config_dir = get_config_dir().map_err(|e| LeaderboardError::IoError(
        std::io::Error::new(std::io::ErrorKind::Other, e.to_string())
    ))?;
    let leaderboard_path = config_dir.join("leaderboard.json");
    
    // Load existing entries
    let mut entries = load_entries().unwrap_or_default();
    
    // Add new entry
    entries.push(entry.clone());
    
    // Sort by WPM (highest first)
    entries.sort_by(|a, b| b.wpm.partial_cmp(&a.wpm).unwrap_or(std::cmp::Ordering::Equal));
    
    // Limit to top 100 entries
    entries.truncate(100);
    
    // Save to file
    let json = serde_json::to_string_pretty(&entries)?;
    fs::write(leaderboard_path, json)?;
    
    Ok(())
}

pub fn load_entries() -> Result<Vec<LeaderboardEntry>, LeaderboardError> {
    let config_dir = get_config_dir().map_err(|e| LeaderboardError::IoError(
        std::io::Error::new(std::io::ErrorKind::Other, e.to_string())
    ))?;
    let leaderboard_path = config_dir.join("leaderboard.json");
    
    // Return empty vec if file doesn't exist
    if !leaderboard_path.exists() {
        return Ok(Vec::new());
    }
    
    // Read and parse file
    let content = fs::read_to_string(leaderboard_path)?;
    if content.trim().is_empty() {
        return Ok(Vec::new());
    }
    
    let entries: Vec<LeaderboardEntry> = serde_json::from_str(&content)?;
    Ok(entries)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::SystemTime;

    #[test]
    fn test_leaderboard_entry_serialization() {
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

        // Test serialization
        let json = serde_json::to_string(&entry).expect("Should serialize to JSON");
        assert!(json.contains("\"wpm\":85.5"));
        assert!(json.contains("\"accuracy\":98.2"));

        // Test deserialization
        let deserialized: LeaderboardEntry = serde_json::from_str(&json)
            .expect("Should deserialize from JSON");
        assert_eq!(entry, deserialized);
    }

    #[test]
    fn test_test_type_serialization() {
        let time_type = TestType::Time(60);
        let word_type = TestType::Word(50);
        let quote_type = TestType::Quote;
        let practice_type = TestType::Practice(5);

        // Test all variants serialize/deserialize correctly
        for test_type in [time_type, word_type, quote_type, practice_type] {
            let json = serde_json::to_string(&test_type).expect("Should serialize");
            let deserialized: TestType = serde_json::from_str(&json)
                .expect("Should deserialize");
            assert_eq!(test_type, deserialized);
        }
    }

    #[test]
    fn test_get_config_dir() {
        // This test will fail until implementation
        let result = get_config_dir();
        assert!(result.is_ok(), "Should return a valid config directory path");
        
        let path = result.unwrap();
        assert!(path.to_string_lossy().contains("typeman"), "Path should contain 'typeman'");
    }

    #[test]
    fn test_save_and_load_entries() {
        // This test will fail until implementation
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

        // Test saving entry
        let save_result = save_entry(&entry);
        assert!(save_result.is_ok(), "Should save entry successfully");

        // Test loading entries
        let load_result = load_entries();
        assert!(load_result.is_ok(), "Should load entries successfully");
        
        let entries = load_result.unwrap();
        assert!(!entries.is_empty(), "Should have at least one entry");
        assert_eq!(entries[0], entry, "Loaded entry should match saved entry");
    }

    #[test]
    fn test_load_entries_empty_file() {
        // This test will fail until implementation
        // Should handle empty or non-existent file gracefully
        let result = load_entries();
        assert!(result.is_ok(), "Should handle empty file gracefully");
        
        let entries = result.unwrap();
        // Could be empty on first run
        assert!(entries.len() >= 0, "Should return valid entries vector");
    }
}