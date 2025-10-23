#!/bin/bash

# Example API Usage for URL Shortener
# Make sure the server is running: cargo run

BASE_URL="http://localhost:3000"

echo "🔗 URL Shortener API Examples"
echo "================================"
echo ""

# 1. Health Check
echo "1️⃣  Health Check:"
echo "   curl $BASE_URL/health"
curl -s $BASE_URL/health
echo ""
echo ""

# 2. Create a short URL
echo "2️⃣  Create a short URL:"
echo "   curl -X POST $BASE_URL/api/shorten \\"
echo "     -H 'Content-Type: application/json' \\"
echo "     -d '{\"url\": \"https://www.rust-lang.org\"}'"
RESPONSE=$(curl -s -X POST $BASE_URL/api/shorten \
  -H "Content-Type: application/json" \
  -d '{"url": "https://www.rust-lang.org"}')
echo "$RESPONSE"
SHORT_CODE=$(echo "$RESPONSE" | grep -o '"short_code":"[^"]*"' | cut -d'"' -f4)
echo ""
echo ""

# 3. Create a short URL with custom code
echo "3️⃣  Create a short URL with custom code:"
echo "   curl -X POST $BASE_URL/api/shorten \\"
echo "     -H 'Content-Type: application/json' \\"
echo "     -d '{\"url\": \"https://github.com/rust-lang/rust\", \"custom_code\": \"rust-gh\"}'"
curl -s -X POST $BASE_URL/api/shorten \
  -H "Content-Type: application/json" \
  -d '{"url": "https://github.com/rust-lang/rust", "custom_code": "rust-gh", "description": "Rust GitHub Repository"}'
echo ""
echo ""

# 4. List all URLs
echo "4️⃣  List all URLs:"
echo "   curl $BASE_URL/api/urls"
curl -s $BASE_URL/api/urls | jq '.'
echo ""

# 5. Get statistics for a URL
if [ -n "$SHORT_CODE" ]; then
  echo "5️⃣  Get statistics for short code '$SHORT_CODE':"
  echo "   curl $BASE_URL/api/stats/$SHORT_CODE"
  curl -s $BASE_URL/api/stats/$SHORT_CODE | jq '.'
  echo ""
fi

# 6. Access a short URL (this will redirect)
echo "6️⃣  Access a short URL (redirect):"
echo "   curl -L $BASE_URL/rust-gh"
echo "   (Opens in browser or use -L flag to follow redirect)"
echo ""

# 7. Check statistics again to see access count
echo "7️⃣  Check statistics again to see updated access count:"
echo "   curl $BASE_URL/api/stats/rust-gh"
curl -s $BASE_URL/api/stats/rust-gh | jq '.'
echo ""

echo ""
echo "✅ Examples complete!"
echo ""
echo "💡 Try these yourself:"
echo "   - Create URLs with different custom codes"
echo "   - Visit a short URL in your browser"
echo "   - Watch the access_count increase"
echo "   - Try to create a duplicate custom code (should fail)"
