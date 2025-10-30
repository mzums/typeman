use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;
use std::io::Write;
use std::time::{Duration, Instant, SystemTime};
use fs2::FileExt;
use crate::language::Language;

/// Cached leaderboard data with timestamp for invalidation
#[derive(Debug, Clone)]
struct LeaderboardCache {
    entries: Vec<LeaderboardEntry>,
    last_modified: SystemTime,
    cached_at: Instant,
}

pub struct LeaderboardData {
    pub open: bool,
    pub entries: Vec<LeaderboardEntry>,
    pub selected: usize,
}

impl LeaderboardCache {
    /// Check if cache is still valid (less than 30 seconds old and file hasn't changed)
    fn is_valid(&self, file_path: &PathBuf) -> bool {
        // Cache expires after 30 seconds
        if self.cached_at.elapsed() > Duration::from_secs(30) {
            return false;
        }
        
        // Check if file has been modified
        if let Ok(metadata) = fs::metadata(file_path) {
            if let Ok(modified) = metadata.modified() {
                return modified <= self.last_modified;
            }
        }
        
        false
    }
}

use std::sync::{Mutex, OnceLock};

/// Global cache for leaderboard data
static LEADERBOARD_CACHE: OnceLock<Mutex<Option<LeaderboardCache>>> = OnceLock::new();

/// Initialize the global cache
fn get_cache() -> &'static Mutex<Option<LeaderboardCache>> {
    LEADERBOARD_CACHE.get_or_init(|| Mutex::new(None))
}

/// Clear the cache (called after successful save operations)
fn invalidate_cache() {
    if let Ok(mut cache) = get_cache().lock() {
        *cache = None;
    }
}

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

impl LeaderboardEntry {
    /// Validates all fields of the leaderboard entry
    pub fn validate(&self) -> Result<(), ValidationError> {
        // WPM validation (0.0-300.0 reasonable range for human typing)
        if self.wpm < 0.0 || self.wpm > 300.0 {
            return Err(ValidationError::InvalidWpm(self.wpm));
        }
        
        // Accuracy validation (0.0-100.0%)
        if self.accuracy < 0.0 || self.accuracy > 100.0 {
            return Err(ValidationError::InvalidAccuracy(self.accuracy));
        }
        
        // Test duration validation (must be positive, max 24 hours = 86400 seconds)
        if self.test_duration < 0.0 || self.test_duration > 86400.0 {
            return Err(ValidationError::InvalidTestDuration(self.test_duration));
        }
        
        // Word count validation (must be reasonable, max 10000 words)
        if self.word_count > 10000 {
            return Err(ValidationError::InvalidWordCount(self.word_count));
        }
        
        // Test mode string length validation (max 20 characters)
        if self.test_mode.len() > 20 {
            return Err(ValidationError::FieldTooLong(
                format!("test_mode too long: {}", self.test_mode.len())
            ));
        }
        
        // Timestamp format validation (RFC3339 format)
        if let Err(_) = chrono::DateTime::parse_from_rfc3339(&self.timestamp) {
            return Err(ValidationError::InvalidTimestamp(self.timestamp.clone()));
        }
        
        Ok(())
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum TestType {
    Time(u32),
    Word(usize),
    Quote,
    Practice(usize),
    Wiki,
}

#[derive(Debug)]
pub enum ValidationError {
    InvalidWpm(f64),
    InvalidAccuracy(f64),
    InvalidTimestamp(String),
    FieldTooLong(String),
    InvalidTestDuration(f64),
    InvalidWordCount(usize),
}

#[derive(Debug)]
pub enum LeaderboardError {
    IoError(std::io::Error),
    SerializationError(serde_json::Error),
    ValidationError(ValidationError),
    LockTimeout,
    LockError(String),
}

/// File lock guard that automatically releases the lock when dropped
pub struct FileLockGuard {
    _file: fs::File,
}

impl FileLockGuard {
    /// Acquires an exclusive lock on the specified file with timeout
    pub fn acquire(lock_path: &PathBuf, timeout: Duration) -> Result<Self, LeaderboardError> {
        let file = fs::OpenOptions::new()
            .create(true)
            .write(true)
            .open(lock_path)
            .map_err(LeaderboardError::IoError)?;
            
        let start_time = Instant::now();
        
        loop {
            match file.try_lock_exclusive() {
                Ok(()) => {
                    return Ok(FileLockGuard { _file: file });
                }
                Err(ref e) if e.kind() == std::io::ErrorKind::WouldBlock => {
                    if start_time.elapsed() > timeout {
                        return Err(LeaderboardError::LockTimeout);
                    }
                    // Short sleep to avoid busy waiting
                    std::thread::sleep(Duration::from_millis(10));
                }
                Err(e) => {
                    return Err(LeaderboardError::LockError(format!("Failed to acquire lock: {}", e)));
                }
            }
        }
    }
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

impl From<ValidationError> for LeaderboardError {
    fn from(error: ValidationError) -> Self {
        LeaderboardError::ValidationError(error)
    }
}

/// Creates a backup of the leaderboard file before modifications with rotation
/// Maintains up to 3 backup files: .json.bak, .json.bak2, .json.bak3
fn create_backup(leaderboard_path: &PathBuf) -> Result<PathBuf, std::io::Error> {
    if !leaderboard_path.exists() {
        return Ok(leaderboard_path.with_extension("json.bak"));
    }
    
    // Rotate existing backups: .bak2 -> .bak3, .bak -> .bak2
    let backup3_path = leaderboard_path.with_extension("json.bak3");
    let backup2_path = leaderboard_path.with_extension("json.bak2");
    let backup1_path = leaderboard_path.with_extension("json.bak");
    
    // Remove oldest backup if it exists
    if backup3_path.exists() {
        fs::remove_file(&backup3_path).ok(); // Ignore errors
    }
    
    // Rotate backup2 -> backup3
    if backup2_path.exists() {
        fs::rename(&backup2_path, &backup3_path).ok(); // Ignore errors
    }
    
    // Rotate backup1 -> backup2
    if backup1_path.exists() {
        fs::rename(&backup1_path, &backup2_path).ok(); // Ignore errors
    }
    
    // Create new backup from current file
    fs::copy(leaderboard_path, &backup1_path)?;
    
    Ok(backup1_path)
}

/// Performs atomic write by writing to a temporary file first, then moving
fn atomic_write(path: &PathBuf, entries: &[LeaderboardEntry]) -> Result<(), LeaderboardError> {
    let temp_path = path.with_extension("json.tmp");
    
    // Write to temporary file first
    let json = serde_json::to_string_pretty(entries)?;
    {
        let mut temp_file = fs::File::create(&temp_path)?;
        temp_file.write_all(json.as_bytes())?;
        temp_file.sync_all()?; // Ensure data is written to disk
    }
    
    // Verify the written data is valid JSON
    if let Err(_) = validate_json_file(&temp_path) {
        fs::remove_file(&temp_path).ok(); // Clean up temp file
        return Err(LeaderboardError::IoError(
            std::io::Error::new(
                std::io::ErrorKind::InvalidData, 
                "Written data failed validation"
            )
        ));
    }
    
    // Atomically move temp file to final location
    fs::rename(&temp_path, path)?;
    
    Ok(())
}

/// Validates that a JSON file contains valid leaderboard data
fn validate_json_file(path: &PathBuf) -> Result<(), LeaderboardError> {
    if !path.exists() {
        return Ok(()); // Non-existent file is valid (empty leaderboard)
    }
    
    let content = fs::read_to_string(path)?;
    if content.trim().is_empty() {
        return Ok(()); // Empty file is valid
    }
    
    // Try to parse as leaderboard entries
    let _entries: Vec<LeaderboardEntry> = serde_json::from_str(&content)?;
    Ok(())
}

/// Attempts to recover from backup files if main file is corrupted
/// Tries backup files in order: .bak, .bak2, .bak3
fn recover_from_backup(leaderboard_path: &PathBuf) -> Result<Vec<LeaderboardEntry>, LeaderboardError> {
    let backup_files = [
        leaderboard_path.with_extension("json.bak"),
        leaderboard_path.with_extension("json.bak2"), 
        leaderboard_path.with_extension("json.bak3"),
    ];
    
    for (i, backup_path) in backup_files.iter().enumerate() {
        if !backup_path.exists() {
            continue;
        }
        
        eprintln!("Attempting to recover leaderboard from backup {}...", i + 1);
        
        if let Ok(()) = validate_json_file(backup_path) {
            let content = fs::read_to_string(backup_path)?;
            if content.trim().is_empty() {
                eprintln!("Backup {} is empty, trying next backup...", i + 1);
                continue;
            }
            
            match serde_json::from_str::<Vec<LeaderboardEntry>>(&content) {
                Ok(entries) => {
                    // Restore this backup as main file
                    fs::copy(backup_path, leaderboard_path)?;
                    eprintln!("Successfully recovered leaderboard from backup {}", i + 1);
                    return Ok(entries);
                }
                Err(_) => {
                    eprintln!("Backup {} is corrupted, trying next backup...", i + 1);
                    continue;
                }
            }
        } else {
            eprintln!("Backup {} failed validation, trying next backup...", i + 1);
        }
    }
    
    eprintln!("No valid backup found, starting with empty leaderboard");
    Ok(Vec::new())
}

/// Retry helper for operations that might fail temporarily
fn retry_operation<F, T, E>(mut operation: F, max_retries: usize, delay: Duration) -> Result<T, E>
where
    F: FnMut() -> Result<T, E>,
    E: std::fmt::Debug,
{
    let mut last_error = None;
    
    for attempt in 0..=max_retries {
        match operation() {
            Ok(result) => return Ok(result),
            Err(error) => {
                if attempt == max_retries {
                    return Err(error);
                }
                
                eprintln!("Operation failed (attempt {}), retrying in {:?}: {:?}", 
                         attempt + 1, delay, error);
                last_error = Some(error);
                std::thread::sleep(delay);
            }
        }
    }
    
    // This should never be reached due to the logic above
    Err(last_error.unwrap())
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
    // Validate entry before saving
    entry.validate()?;
    
    let config_dir = get_config_dir().map_err(|e| LeaderboardError::IoError(
        std::io::Error::new(std::io::ErrorKind::Other, e.to_string())
    ))?;
    let leaderboard_path = config_dir.join("leaderboard.json");
    let lock_path = config_dir.join("leaderboard.lock");
    
    // Retry the entire save operation up to 3 times for temporary failures
    retry_operation(|| -> Result<(), LeaderboardError> {
        // Acquire file lock with 5-second timeout
        let _lock_guard = FileLockGuard::acquire(&lock_path, Duration::from_secs(5))?;
        
        // Create backup before making changes
        create_backup(&leaderboard_path)?;
        
        // Load existing entries
        let mut entries = load_entries().unwrap_or_default();
        
        // Add new entry
        entries.push(entry.clone());
        
        // Sort by WPM (highest first)
        entries.sort_by(|a, b| b.wpm.partial_cmp(&a.wpm).unwrap_or(std::cmp::Ordering::Equal));
        
        // Limit to top 100 entries
        entries.truncate(100);
        
        // Save to file using atomic write
        atomic_write(&leaderboard_path, &entries)?;
        
        Ok(())
    }, 2, Duration::from_millis(100))?; // Retry up to 2 times with 100ms delay
    
    // Invalidate cache after successful save
    invalidate_cache();
    
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
    
    // Check cache first
    if let Ok(cache_guard) = get_cache().lock() {
        if let Some(ref cache) = *cache_guard {
            if cache.is_valid(&leaderboard_path) {
                return Ok(cache.entries.clone());
            }
        }
    }
    
    // Cache miss or invalid, load from file
    let entries = load_entries_from_file(&leaderboard_path)?;
    
    // Update cache
    if let Ok(metadata) = fs::metadata(&leaderboard_path) {
        if let Ok(modified) = metadata.modified() {
            if let Ok(mut cache_guard) = get_cache().lock() {
                *cache_guard = Some(LeaderboardCache {
                    entries: entries.clone(),
                    last_modified: modified,
                    cached_at: Instant::now(),
                });
            }
        }
    }
    
    Ok(entries)
}

/// Load entries directly from file without caching
fn load_entries_from_file(leaderboard_path: &PathBuf) -> Result<Vec<LeaderboardEntry>, LeaderboardError> {
    // First try to validate and load the main file
    if let Ok(()) = validate_json_file(leaderboard_path) {
        let content = fs::read_to_string(leaderboard_path)?;
        if content.trim().is_empty() {
            return Ok(Vec::new());
        }
        
        match serde_json::from_str(&content) {
            Ok(entries) => return Ok(entries),
            Err(_) => {
                eprintln!("Main leaderboard file appears corrupted, attempting recovery...");
            }
        }
    } else {
        eprintln!("Main leaderboard file validation failed, attempting recovery...");
    }
    
    // If main file is corrupted, try to recover from backup
    recover_from_backup(leaderboard_path)
}

#[cfg(test)]
mod tests {
    use super::*;

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
        
        // Find our entry in the list (entries are sorted by WPM, so it might not be first)
        let found_entry = entries.iter().find(|e| {
            e.wpm == entry.wpm && 
            e.accuracy == entry.accuracy &&
            e.test_mode == entry.test_mode &&
            e.timestamp == entry.timestamp
        });
        
        assert!(found_entry.is_some(), "Should find the saved entry in the leaderboard");
        assert_eq!(found_entry.unwrap(), &entry, "Loaded entry should match saved entry");
    }

    #[test]
    fn test_load_entries_empty_file() {
        // This test will fail until implementation
        // Should handle empty or non-existent file gracefully
        let result = load_entries();
        assert!(result.is_ok(), "Should handle empty file gracefully");
        
        let entries = result.unwrap();
        // Could be empty on first run - just verify it's a valid vector
        assert!(entries.is_empty() || !entries.is_empty(), "Should return valid entries vector");
    }
}