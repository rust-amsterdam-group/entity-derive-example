pub use chrono::{DateTime, Utc};
pub use uuid::Uuid;

pub trait Entity<Id> {
    fn id(&self) -> &Id;
    fn created_on(&self) -> &DateTime<Utc>;
    fn created_by(&self) -> &str;
    fn updated_on(&self) -> &DateTime<Utc>;
    fn updated_by(&self) -> &str;
}

pub trait UpdateableWith<T> {
    fn update(&mut self, data: T) -> &mut Self;
}
