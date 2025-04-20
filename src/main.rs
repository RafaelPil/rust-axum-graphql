// main.rs
mod schema;
mod models;
mod storage;

use std::sync::Arc;
use axum::{routing::get, Router};
use schema::AppSchema;
use storage::BookStorage;
use async_graphql_axum::GraphQL;

async fn graphiql() -> axum::response::Html<String> {
    axum::response::Html(
        async_graphql::http::GraphiQLSource::build()
            .endpoint("/graphql")
            .finish(),
    )
}

#[tokio::main]
async fn main() {
    let storage = Arc::new(BookStorage::default());

    // Create some sample books
    Arc::clone(&storage).create_book(models::CreateBookInput {
        title: "The Rust Programming Language".to_string(),
        author: "Steve Klabnik and Carol Nichols".to_string(),
        published: true,
    }).await;
    
    Arc::clone(&storage).create_book(models::CreateBookInput {
        title: "Zero to Production in Rust".to_string(),
        author: "Luca Palmieri".to_string(),
        published: true,
    }).await;

    // Create GraphQL schema
    let schema = AppSchema::build(
        schema::Query { storage: Arc::clone(&storage) },
        schema::Mutation { storage: Arc::clone(&storage) },
        async_graphql::EmptySubscription,
    )
    .finish();

    // Build our application with a route
    let app = Router::new()
        .route("/graphql", get(graphiql).post_service(GraphQL::new(schema)))
        .route("/", get(|| async { "Try visiting /graphiql" }));

    // Run it
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!("GraphiQL IDE: http://localhost:3000/graphql");
    axum::serve(listener, app).await.unwrap();
}