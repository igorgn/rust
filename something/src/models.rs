use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

/// Represents a shortened URL with all its metadata
/// 
/// This struct demonstrates:
/// - Serde for JSON serialization/deserialization
/// - Chrono for timestamp handling
/// - Builder pattern consideration for complex structs
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ShortenedUrl {
    /// Unique identifier for this URL mapping
    pub id: String,
    
    /// The original long URL
    pub original_url: String,
    
    /// The short code (e.g., "abc123")
    pub short_code: String,
    
    /// When this URL was created
    pub created_at: DateTime<Utc>,
    
    /// Number of times this short URL has been accessed
    pub access_count: u64,
    
    /// Last time this URL was accessed (None if never accessed)
    pub last_accessed: Option<DateTime<Utc>>,
    
    /// Optional custom description
    pub description: Option<String>,
}

impl ShortenedUrl {
    /// Creates a new ShortenedUrl
    /// 
    /// Note: This is a simple constructor. In a real app, you might use
    /// the builder pattern for more complex initialization.
    pub fn new(original_url: String, short_code: String) -> Self {
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            original_url,
            short_code,
            created_at: Utc::now(),
            access_count: 0,
            last_accessed: None,
            description: None,
        }
    }
    
    /// Records an access to this URL
    /// 
    /// This demonstrates mutable methods that update internal state
    pub fn record_access(&mut self) {
        self.access_count += 1;
        self.last_accessed = Some(Utc::now());
    }
}

/// Request body for creating a new short URL
/// 
/// This demonstrates API request modeling with validation
#[derive(Debug, Deserialize)]
pub struct CreateUrlRequest {
    /// The URL to shorten (required)
    pub url: String,
    
    /// Optional custom short code (if not provided, one will be generated)
    pub custom_code: Option<String>,
    
    /// Optional description
    pub description: Option<String>,
}

impl CreateUrlRequest {
    /// Validates the request
    /// 
    /// This shows how to implement validation logic
    pub fn validate(&self) -> Result<(), String> {
        // Basic URL validation
        if self.url.is_empty() {
            return Err("URL cannot be empty".to_string());
        }
        
        if !self.url.starts_with("http://") && !self.url.starts_with("https://") {
            return Err("URL must start with http:// or https://".to_string());
        }
        
        // Validate custom code if provided
        if let Some(ref code) = self.custom_code {
            if code.is_empty() {
                return Err("Custom code cannot be empty".to_string());
            }
            
            if code.len() > 20 {
                return Err("Custom code cannot exceed 20 characters".to_string());
            }
            
            // Only allow alphanumeric and hyphens
            if !code.chars().all(|c| c.is_alphanumeric() || c == '-') {
                return Err("Custom code can only contain letters, numbers, and hyphens".to_string());
            }
        }
        
        Ok(())
    }
}

/// Response body for successful URL creation
/// 
/// This separates internal models from API responses (important for API design)
#[derive(Debug, Serialize)]
pub struct CreateUrlResponse {
    pub short_code: String,
    pub short_url: String,
    pub original_url: String,
    pub created_at: DateTime<Utc>,
}

/// Statistics for a shortened URL
/// 
/// Shows how to create different views of the same data
#[derive(Debug, Serialize)]
pub struct UrlStats {
    pub short_code: String,
    pub original_url: String,
    pub access_count: u64,
    pub created_at: DateTime<Utc>,
    pub last_accessed: Option<DateTime<Utc>>,
    pub description: Option<String>,
}

impl From<&ShortenedUrl> for UrlStats {
    /// Converts a ShortenedUrl to UrlStats
    /// 
    /// This demonstrates the From trait for type conversion
    fn from(url: &ShortenedUrl) -> Self {
        Self {
            short_code: url.short_code.clone(),
            original_url: url.original_url.clone(),
            access_count: url.access_count,
            created_at: url.created_at,
            last_accessed: url.last_accessed,
            description: url.description.clone(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_empty_url() {
        let request = CreateUrlRequest {
            url: String::new(),
            custom_code: None,
            description: None,
        };
        assert!(request.validate().is_err());
    }

    #[test]
    fn test_validate_invalid_protocol() {
        let request = CreateUrlRequest {
            url: "ftp://example.com".to_string(),
            custom_code: None,
            description: None,
        };
        assert!(request.validate().is_err());
    }

    #[test]
    fn test_validate_valid_url() {
        let request = CreateUrlRequest {
            url: "https://example.com".to_string(),
            custom_code: Some("test-123".to_string()),
            description: Some("Test URL".to_string()),
        };
        assert!(request.validate().is_ok());
    }

    #[test]
    fn test_record_access() {
        let mut url = ShortenedUrl::new(
            "https://example.com".to_string(),
            "test".to_string(),
        );
        
        assert_eq!(url.access_count, 0);
        assert!(url.last_accessed.is_none());
        
        url.record_access();
        
        assert_eq!(url.access_count, 1);
        assert!(url.last_accessed.is_some());
    }
}
