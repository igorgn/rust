use std::collections::HashMap;
use std::sync::RwLock;
use sha2::{Sha256, Digest};
use base64::{Engine as _, engine::general_purpose};
use crate::models::ShortenedUrl;

/// Thread-safe in-memory URL storage
/// 
/// This demonstrates several important Rust concepts:
/// - RwLock for concurrent read/write access
/// - Interior mutability pattern
/// - HashMap for efficient lookups
/// - Thread safety without garbage collection
pub struct UrlStore {
    /// Maps short_code -> ShortenedUrl
    /// 
    /// RwLock allows multiple readers OR one writer at a time.
    /// This is more efficient than Mutex when reads are common.
    urls: RwLock<HashMap<String, ShortenedUrl>>,
    
    /// Reverse mapping: original_url -> short_code
    /// Prevents duplicate URLs from getting multiple short codes
    url_to_code: RwLock<HashMap<String, String>>,
}

impl UrlStore {
    /// Creates a new empty URL store
    pub fn new() -> Self {
        Self {
            urls: RwLock::new(HashMap::new()),
            url_to_code: RwLock::new(HashMap::new()),
        }
    }
    
    /// Generates a short code from a URL using SHA-256 hashing
    /// 
    /// This demonstrates:
    /// - Cryptographic hashing
    /// - Base64 encoding
    /// - String manipulation
    pub fn generate_short_code(url: &str) -> String {
        let mut hasher = Sha256::new();
        hasher.update(url.as_bytes());
        let result = hasher.finalize();
        
        // Convert to base64 and take first 8 characters
        // This gives us ~281 trillion possible combinations
        let encoded = general_purpose::URL_SAFE_NO_PAD.encode(result);
        encoded.chars().take(8).collect()
    }
    
    /// Checks if a short code already exists
    pub fn exists(&self, short_code: &str) -> Result<bool, String> {
        let urls = self.urls.read()
            .map_err(|e| format!("Failed to acquire read lock: {}", e))?;
        Ok(urls.contains_key(short_code))
    }
    
    /// Checks if an original URL is already shortened
    pub fn get_existing_code(&self, original_url: &str) -> Result<Option<String>, String> {
        let url_to_code = self.url_to_code.read()
            .map_err(|e| format!("Failed to acquire read lock: {}", e))?;
        Ok(url_to_code.get(original_url).cloned())
    }
    
    /// Stores a new shortened URL
    /// 
    /// This demonstrates:
    /// - Write lock acquisition
    /// - Error handling with Result
    /// - Ownership and cloning considerations
    pub fn store(&self, url: ShortenedUrl) -> Result<(), String> {
        let short_code = url.short_code.clone();
        let original_url = url.original_url.clone();
        
        // Acquire write locks for both maps
        let mut urls = self.urls.write()
            .map_err(|e| format!("Failed to acquire write lock: {}", e))?;
        let mut url_to_code = self.url_to_code.write()
            .map_err(|e| format!("Failed to acquire write lock: {}", e))?;
        
        // Check if short code already exists
        if urls.contains_key(&short_code) {
            return Err(format!("Short code '{}' already exists", short_code));
        }
        
        // Store in both maps
        urls.insert(short_code.clone(), url);
        url_to_code.insert(original_url, short_code);
        
        Ok(())
    }
    
    /// Retrieves a URL by its short code
    /// 
    /// Returns a clone to avoid holding the lock longer than necessary
    pub fn get(&self, short_code: &str) -> Result<Option<ShortenedUrl>, String> {
        let urls = self.urls.read()
            .map_err(|e| format!("Failed to acquire read lock: {}", e))?;
        Ok(urls.get(short_code).cloned())
    }
    
    /// Records an access to a shortened URL and returns the original URL
    /// 
    /// This demonstrates:
    /// - Mutable access within an immutable method
    /// - Careful lock handling to avoid deadlocks
    pub fn record_access(&self, short_code: &str) -> Result<Option<String>, String> {
        // First, try to get the URL
        let mut urls = self.urls.write()
            .map_err(|e| format!("Failed to acquire write lock: {}", e))?;
        
        if let Some(url) = urls.get_mut(short_code) {
            url.record_access();
            Ok(Some(url.original_url.clone()))
        } else {
            Ok(None)
        }
    }
    
    /// Lists all shortened URLs
    /// 
    /// Returns clones to avoid holding the lock during serialization
    pub fn list_all(&self) -> Result<Vec<ShortenedUrl>, String> {
        let urls = self.urls.read()
            .map_err(|e| format!("Failed to acquire read lock: {}", e))?;
        Ok(urls.values().cloned().collect())
    }
    
    /// Gets the total count of stored URLs
    pub fn count(&self) -> Result<usize, String> {
        let urls = self.urls.read()
            .map_err(|e| format!("Failed to acquire read lock: {}", e))?;
        Ok(urls.len())
    }
}

impl Default for UrlStore {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_short_code() {
        let code1 = UrlStore::generate_short_code("https://example.com");
        let code2 = UrlStore::generate_short_code("https://example.com");
        let code3 = UrlStore::generate_short_code("https://different.com");
        
        // Same URL should generate same code
        assert_eq!(code1, code2);
        
        // Different URLs should generate different codes
        assert_ne!(code1, code3);
        
        // Code should be 8 characters
        assert_eq!(code1.len(), 8);
    }

    #[test]
    fn test_store_and_retrieve() {
        let store = UrlStore::new();
        let url = ShortenedUrl::new(
            "https://example.com".to_string(),
            "test123".to_string(),
        );
        
        // Store should succeed
        assert!(store.store(url.clone()).is_ok());
        
        // Should be able to retrieve
        let retrieved = store.get("test123").unwrap();
        assert!(retrieved.is_some());
        assert_eq!(retrieved.unwrap().original_url, "https://example.com");
        
        // Duplicate short code should fail
        let duplicate = ShortenedUrl::new(
            "https://different.com".to_string(),
            "test123".to_string(),
        );
        assert!(store.store(duplicate).is_err());
    }

    #[test]
    fn test_record_access() {
        let store = UrlStore::new();
        let url = ShortenedUrl::new(
            "https://example.com".to_string(),
            "test123".to_string(),
        );
        
        store.store(url).unwrap();
        
        // Initial access count should be 0
        let url = store.get("test123").unwrap().unwrap();
        assert_eq!(url.access_count, 0);
        
        // Record an access
        let result = store.record_access("test123").unwrap();
        assert_eq!(result, Some("https://example.com".to_string()));
        
        // Access count should be incremented
        let url = store.get("test123").unwrap().unwrap();
        assert_eq!(url.access_count, 1);
    }

    #[test]
    fn test_get_existing_code() {
        let store = UrlStore::new();
        let url = ShortenedUrl::new(
            "https://example.com".to_string(),
            "test123".to_string(),
        );
        
        store.store(url).unwrap();
        
        // Should find existing code
        let code = store.get_existing_code("https://example.com").unwrap();
        assert_eq!(code, Some("test123".to_string()));
        
        // Should return None for non-existent URL
        let code = store.get_existing_code("https://notfound.com").unwrap();
        assert_eq!(code, None);
    }
}
