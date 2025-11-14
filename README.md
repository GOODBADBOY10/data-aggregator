# 🌐 Crypto Data Aggregator

A high-performance API aggregator built with Rust that fetches cryptocurrency data from multiple sources in parallel and returns a unified response.

## 🎯 Overview

This service demonstrates the **API Gateway pattern** by aggregating data from multiple third-party APIs into a single endpoint. Instead of clients making multiple requests to different services, they call one endpoint and receive combined, formatted data.

## 🏗️ Architecture
```
Client Request
     ↓
[Axum Server - Port 3000]
     ↓
Parallel HTTP Requests (via Reqwest)
     ├─→ CoinGecko API (Bitcoin price)
     ├─→ CoinCap API (Ethereum price)
     └─→ Trending Tokens API
     ↓
Data Aggregation & Transformation
     ↓
Unified JSON Response
```

## ✨ Features

- **Parallel API Calls**: Uses `tokio::join!` to fetch data concurrently
- **Request Timeouts**: Prevents slow APIs from hanging the server
- **Error Handling**: Graceful degradation when external APIs fail
- **JSON Serialization**: Clean data transformation using Serde
- **Type Safety**: Strongly-typed responses with Rust structs

## 🚀 Tech Stack

- **Axum** - Web framework
- **Reqwest** - HTTP client for API calls
- **Tokio** - Async runtime
- **Serde** - JSON serialization/deserialization
- **Anyhow** - Error handling

## 📦 Installation
```bash
# Clone the repository
git clone <your-repo-url>
cd crypto-aggregator

# Build the project
cargo build --release

# Run the server
cargo run
```

## 🔧 API Endpoints

### `GET /summary`

Returns aggregated cryptocurrency data from multiple sources.

**Response:**
```json
{
  "bitcoin_price": 45000.50,
  "ethereum_price": 2800.25,
  "trending_tokens": ["SOL", "DOGE", "MATIC"],
  "fetched_at": "2025-11-14T10:30:00Z"
}
```

**Status Codes:**
- `200 OK` - Successfully aggregated data
- `500 Internal Server Error` - Failed to fetch from external APIs
- `504 Gateway Timeout` - External API timeout

## 🧪 Testing
```bash
# Test the aggregator endpoint
curl http://localhost:3000/summary

# Pretty print JSON (requires jq)
curl http://localhost:3000/summary | jq
```

## 📊 Performance

- **Sequential requests**: ~6 seconds (2s + 2s + 2s)
- **Parallel requests**: ~2 seconds (max of all requests)
- **3x faster** using concurrent execution

## 🎓 Key Learnings

1. **Parallel HTTP Requests**: How to make multiple API calls simultaneously using async Rust
2. **Data Aggregation**: Combining responses from different sources into a unified format
3. **Error Handling**: Gracefully handling network failures and timeouts
4. **Type Safety**: Using Rust's type system to ensure data consistency
5. **Real-world Patterns**: Understanding the API Gateway/BFF pattern used in production systems

## 🔮 Future Enhancements

- [ ] Add request retries with exponential backoff
- [ ] Implement circuit breaker pattern
- [ ] Add response caching (Redis)
- [ ] Distributed tracing (OpenTelemetry)
- [ ] Rate limiting for external APIs
- [ ] Health check endpoints
- [ ] Prometheus metrics

## 📝 Dependencies
```toml
[dependencies]
axum = "0.7"
tokio = { version = "1", features = ["full"] }
reqwest = { version = "0.12", features = ["json"] }
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
anyhow = "1.0"
```

## 🤝 Real-World Applications

This pattern is used by major tech companies:

- **Netflix**: Aggregates recommendations, continue watching, and trending content
- **Uber**: Combines driver location, pricing, and ETA data
- **Stripe**: Merges payment status, fraud detection, and transaction history

## 📚 Resources

- [Axum Documentation](https://docs.rs/axum)
- [Reqwest Documentation](https://docs.rs/reqwest)
- [Tokio Async Book](https://tokio.rs/tokio/tutorial)
- [API Gateway Pattern](https://microservices.io/patterns/apigateway.html)

## 📄 License

MIT

---

**Built with 🦀 Rust**