use crate::domain::models::invoice::Invoice;
use uuid::Uuid;
use async_trait::async_trait;


#[async_trait]
pub trait InvoiceRepository {
  async fn find_all(&self) -> Result<Vec<Invoice>, sqlx::Error>;
  async fn find_by_id(&self, id: Uuid) -> Result<Option<Invoice>, sqlx::Error>;
  async fn create(&self, invoice: Invoice) -> Result<Invoice, sqlx::Error>;
  async fn update(&self, invoice: Invoice) -> Result<Invoice, sqlx::Error>;
  async fn delete(&self, id: Uuid) -> Result<(), sqlx::Error>;
}
