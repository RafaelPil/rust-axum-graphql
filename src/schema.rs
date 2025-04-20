use async_graphql::*;
use crate::{models::{Book, CreateBookInput, UpdateBookInput}, storage::BookStorage};
use std::sync::Arc;

pub struct Query {
    pub storage: Arc<BookStorage>,
}

#[Object]
impl Query {
    async fn books(&self) -> Vec<Book> {
        self.storage.get_books().await
    }

    async fn book(&self, id: ID) -> Option<Book> {
        self.storage.get_book(id).await
    }
}

pub struct Mutation {
    pub storage: Arc<BookStorage>,
}

#[Object]
impl Mutation {
    async fn create_book(&self, input: CreateBookInput) -> Book {
        self.storage.create_book(input).await
    }

    async fn update_book(&self, input: UpdateBookInput) -> Option<Book> {
        self.storage.update_book(input).await
    }

    async fn delete_book(&self, id: ID) -> bool {
        self.storage.delete_book(id).await
    }
}

pub type AppSchema = Schema<Query, Mutation, EmptySubscription>;