use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::{IntoResponse, Redirect},
    Json,
};
use crate::{
    AppState,
    error::AppError,
    models::{CreateUrlRequest, CreateUrlResponse, ShortenedUrl, UrlStats},
    storage::UrlStore,
};

/// Health check endpoint
/// 
/// This is a simple handler that demonstrates:
/// - Async function syntax
/// - Simple status responses
/// - No state required
pub async fn health_check() -> impl IntoResponse {
    (StatusCode::OK, "OK")
}

/// Creates a new shortened URL
/// 
/// This handler demonstrates:
/// - Extracting JSON request body
/// - Accessing shared application state
/// - Validation
/// - Error handling with custom error types
/// - Returning JSON responses
pub async fn create_short_url(
    State(state): State<AppState>,
    Json(request): Json<CreateUrlRequest>,
) -> Result<Json<CreateUrlResponse>, AppError> {
    // Validate the request
    request.validate()
        .map_err(|e| AppError::BadRequest(e))?;
    
    // Check if this URL is already shortened
    if let Some(existing_code) = state.store
        .get_existing_code(&request.url)
        .map_err(|e| AppError::StorageError(e))? 
    {
        // Return the existing short code
        let url = state.store
            .get(&existing_code)
            .map_err(|e| AppError::StorageError(e))?
            .ok_or_else(|| AppError::NotFound("URL not found".to_string()))?;
        
        return Ok(Json(CreateUrlResponse {
            short_code: url.short_code.clone(),
            short_url: format!("http://localhost:3000/{}", url.short_code),
            original_url: url.original_url.clone(),
            created_at: url.created_at,
        }));
    }
    
    // Determine the short code
    let short_code = if let Some(custom) = request.custom_code {
        // Check if custom code is available
        if state.store.exists(&custom)
            .map_err(|e| AppError::StorageError(e))? 
        {
            return Err(AppError::BadRequest(
                format!("Custom code '{}' is already taken", custom)
            ));
        }
        custom
    } else {
        // Generate a short code
        UrlStore::generate_short_code(&request.url)
    };
    
    // Create the shortened URL
    let mut url = ShortenedUrl::new(request.url.clone(), short_code.clone());
    url.description = request.description;
    
    // Store it
    state.store.store(url.clone())
        .map_err(|e| AppError::StorageError(e))?;
    
    // Return the response
    Ok(Json(CreateUrlResponse {
        short_code: url.short_code.clone(),
        short_url: format!("http://localhost:3000/{}", url.short_code),
        original_url: url.original_url.clone(),
        created_at: url.created_at,
    }))
}

/// Redirects to the original URL
/// 
/// This handler demonstrates:
/// - Path parameter extraction
/// - Conditional responses (redirect or not found)
/// - Side effects (recording access)
pub async fn redirect_to_url(
    State(state): State<AppState>,
    Path(short_code): Path<String>,
) -> Result<Redirect, AppError> {
    // Record the access and get the original URL
    let original_url = state.store
        .record_access(&short_code)
        .map_err(|e| AppError::StorageError(e))?
        .ok_or_else(|| AppError::NotFound(format!("Short code '{}' not found", short_code)))?;
    
    // Redirect to the original URL
    Ok(Redirect::temporary(&original_url))
}

/// Lists all shortened URLs
/// 
/// This handler demonstrates:
/// - Returning collections as JSON
/// - Mapping between internal and API models
pub async fn list_urls(
    State(state): State<AppState>,
) -> Result<Json<Vec<UrlStats>>, AppError> {
    let urls = state.store.list_all()
        .map_err(|e| AppError::StorageError(e))?;
    
    // Convert to stats format
    let stats: Vec<UrlStats> = urls.iter()
        .map(|url| UrlStats::from(url))
        .collect();
    
    Ok(Json(stats))
}

/// Gets statistics for a specific shortened URL
/// 
/// This handler demonstrates:
/// - Retrieving a single resource
/// - 404 handling
pub async fn get_stats(
    State(state): State<AppState>,
    Path(short_code): Path<String>,
) -> Result<Json<UrlStats>, AppError> {
    let url = state.store.get(&short_code)
        .map_err(|e| AppError::StorageError(e))?
        .ok_or_else(|| AppError::NotFound(format!("Short code '{}' not found", short_code)))?;
    
    Ok(Json(UrlStats::from(&url)))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_health_check() {
        let response = health_check().await.into_response();
        assert_eq!(response.status(), StatusCode::OK);
    }

    // Note: More comprehensive integration tests would require axum::test
    // and are better suited for integration test files
}
