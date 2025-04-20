use async_graphql::*;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, SimpleObject)]
pub struct Book {
    pub id: ID,
    pub title: String,
    pub author: String,
    pub published: bool,
}

#[derive(InputObject)]
pub struct CreateBookInput {
    pub title: String,
    pub author: String,
    pub published: bool,
}

#[derive(InputObject)]
pub struct UpdateBookInput {
    pub id: ID,
    pub title: Option<String>,
    pub author: Option<String>,
    pub published: Option<bool>,
}