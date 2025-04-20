use std::sync::Arc;
use tokio::sync::Mutex;
use crate::models::Book;
use async_graphql::ID;

#[derive(Default, Clone)]
pub struct BookStorage {
    books: Arc<Mutex<Vec<Book>>>,
}

impl BookStorage {
    pub async fn get_books(&self) -> Vec<Book> {
        let books = self.books.lock().await;
        books.clone()
    }

    pub async fn get_book(&self, id: ID) -> Option<Book> {
        let books = self.books.lock().await;
        books.iter().find(|book| book.id == id).cloned()
    }

    pub async fn create_book(&self, input: crate::models::CreateBookInput) -> Book {
        let mut books = self.books.lock().await;
        let book = Book {
            id: ID::from(uuid::Uuid::new_v4()),
            title: input.title,
            author: input.author,
            published: input.published,
        };
        books.push(book.clone());
        book
    }

    pub async fn update_book(&self, input: crate::models::UpdateBookInput) -> Option<Book> {
        let mut books = self.books.lock().await;
        
        if let Some(index) = books.iter().position(|book| book.id == input.id) {
            let book = &mut books[index];
            
            if let Some(title) = input.title {
                book.title = title;
            }
            if let Some(author) = input.author {
                book.author = author;
            }
            if let Some(published) = input.published {
                book.published = published;
            }
            
            Some(book.clone())
        } else {
            None
        }
    }

    pub async fn delete_book(&self, id: ID) -> bool {
        let mut books = self.books.lock().await;
        if let Some(index) = books.iter().position(|book| book.id == id) {
            books.remove(index);
            true
        } else {
            false
        }
    }
}