pub mod models;
pub mod connection;

// Re-export important items
pub use connection::UniversalPathDb;

// Re-export models properly
pub use models::{
    Article, ArticleWithAuthor, NewArticle, UpdateArticle,
    Category, CategoryWithCounts, CategoryTreeItem, NewCategory, UpdateCategory,
    Term, NewTerm, UpdateTerm,
    User, LoginUser, UserToken
};