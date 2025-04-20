# BookGraphQL

A high-performance book management API built with Rust, Axum, and async-GraphQL.

## Features
- 📚 CRUD operations for books
- ⚡ GraphQL queries/mutations
- 🔒 Thread-safe storage with `Arc<Mutex>`

## Tech Stack
- **Rust** (latest stable)
- **Axum** (web framework)
- **async-GraphQL**
- **Tokio** (async runtime)

## Quick Start
```bash
git clone https://github.com/yourname/bookgraphql
cd bookgraphql
cargo run```


## Endpoints
POST /graphql - GraphQL endpoint
GET /graphql - GraphiQL IDE