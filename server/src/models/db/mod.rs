pub use product::*;
mod product;
pub use recipe::*;
mod recipe;

pub type _DbError = Box<dyn std::error::Error + Send + Sync>;
