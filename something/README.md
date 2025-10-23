# URL Shortener - Rust Study Project 🔗

A production-quality URL shortener service built with Rust, designed as an educational codebase for studying modern Rust development practices.

## 🎯 What This Project Teaches

This codebase demonstrates:

- **Web Development with Axum**: Modern async HTTP server framework
- **Concurrency & Thread Safety**: RwLock, Arc, and safe concurrent data structures
- **Error Handling**: Custom error types, Result, and HTTP error responses
- **Data Modeling**: Separation of concerns between models, storage, and API layers
- **Testing**: Unit tests for business logic and handlers
- **Type Safety**: Leveraging Rust's type system for correctness
- **Async/Await**: Tokio runtime and async programming patterns
- **REST API Design**: Clean API endpoints with proper HTTP methods

## 🏗️ Project Structure

```
src/
├── main.rs         # Application entry point, server setup, routing
├── models.rs       # Data structures (URL, requests, responses)
├── storage.rs      # Thread-safe in-memory storage with RwLock
├── handlers.rs     # HTTP request handlers (business logic)
└── error.rs        # Custom error types and HTTP error responses
```

## 📚 Key Concepts to Study

### 1. Concurrency with RwLock (`storage.rs`)

**Look for:**
- `RwLock<HashMap<...>>` - Multiple readers OR single writer pattern
- `.read()` and `.write()` - Lock acquisition patterns
- Error handling with lock poisoning

**Why it matters:** Shows how Rust achieves thread safety without garbage collection.

```rust
// Multiple readers can access simultaneously
let urls = self.urls.read()?;

// Only one writer at a time
let mut urls = self.urls.write()?;
```

### 2. Arc for Shared Ownership (`main.rs`)

**Look for:**
- `Arc<UrlStore>` - Atomic Reference Counting
- `#[derive(Clone)]` on `AppState` - Cheap cloning of Arc

**Why it matters:** Learn how to share state across async tasks safely.

### 3. Type-Driven API Design (`models.rs`)

**Look for:**
- Separate types for requests, responses, and internal models
- `#[derive(Serialize, Deserialize)]` - Automatic JSON conversion
- Validation methods on request types
- `From` trait for type conversions

**Why it matters:** Type safety prevents bugs at compile time.

### 4. Error Handling Patterns (`error.rs`, `handlers.rs`)

**Look for:**
- `thiserror::Error` derive macro
- `IntoResponse` trait implementation
- `Result<T, AppError>` return types
- `.map_err()` for error conversion

**Why it matters:** Rust's error handling is explicit and composable.

### 5. Async HTTP Handlers (`handlers.rs`)

**Look for:**
- `async fn` keyword
- `State<AppState>` extractor - Dependency injection
- `Json<T>` extractor and response
- `Path<String>` extractor - URL parameters
- `Redirect` response type

**Why it matters:** Modern Rust web development patterns.

### 6. Testing (`*.rs` - see `#[cfg(test)]` sections)

**Look for:**
- Unit tests in each module
- `#[tokio::test]` for async tests
- Test organization with `mod tests`

**Why it matters:** Learn testing patterns in Rust.

## 🚀 Running the Project

1. **Build the project:**
   ```bash
   cargo build
   ```

2. **Run the server:**
   ```bash
   cargo run
   ```

3. **Test the API:**

   Create a short URL:
   ```bash
   curl -X POST http://localhost:3000/api/shorten \
     -H "Content-Type: application/json" \
     -d '{"url": "https://www.rust-lang.org"}'
   ```

   Use the short URL (visit in browser or curl):
   ```bash
   curl -L http://localhost:3000/<short_code>
   ```

   Get statistics:
   ```bash
   curl http://localhost:3000/api/stats/<short_code>
   ```

   List all URLs:
   ```bash
   curl http://localhost:3000/api/urls
   ```

4. **Run tests:**
   ```bash
   cargo test
   ```

## 🔍 Study Path Recommendations

### Beginner Path
1. Start with `main.rs` - understand the application setup
2. Read `models.rs` - learn data structures
3. Study `handlers.rs` - see how HTTP requests are handled
4. Run the tests to see everything in action

### Intermediate Path
1. Study `storage.rs` - deep dive into RwLock and concurrency
2. Analyze `error.rs` - learn error handling patterns
3. Trace a request flow: route → handler → storage → response
4. Modify a handler to add new functionality

### Advanced Path
1. Add persistence (save to file on shutdown, load on startup)
2. Add expiration dates for URLs
3. Add rate limiting
4. Add authentication
5. Replace in-memory storage with Redis/database

## 🎓 Learning Exercises

1. **Add a DELETE endpoint** to remove a short URL
2. **Add URL expiration** - URLs expire after N days
3. **Add analytics** - track which referrers access your URLs
4. **Add custom domains** - Support multiple short domains
5. **Add rate limiting** - Prevent abuse with token bucket algorithm
6. **Add persistence** - Save/load from JSON file or SQLite
7. **Add metrics** - Integrate with Prometheus for monitoring

## 📖 Additional Resources

- [Axum Documentation](https://docs.rs/axum/)
- [Tokio Documentation](https://tokio.rs/)
- [Rust Book on Concurrency](https://doc.rust-lang.org/book/ch16-00-concurrency.html)
- [RwLock vs Mutex](https://doc.rust-lang.org/std/sync/struct.RwLock.html)

## 🔑 Key Takeaways

After studying this codebase, you should understand:

1. ✅ How to structure a Rust web application
2. ✅ Thread-safe concurrent data structures
3. ✅ Async/await patterns in Rust
4. ✅ Type-driven development
5. ✅ Error handling best practices
6. ✅ Testing strategies
7. ✅ REST API design patterns

## 💡 Questions to Consider While Studying

- Why use `RwLock` instead of `Mutex`?
- Why clone the data when reading from storage?
- How does `Arc` differ from `Rc`?
- Why separate request/response types from internal models?
- What happens if two requests try to create URLs simultaneously?
- How does the `?` operator work with custom error types?
- Why is `AppState` Clone but contains Arc?

Happy studying! 🦀
