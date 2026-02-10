use crate::domain::models::invoice::Invoice;
use crate::domain::repositories::invoice_repository::InvoiceRepository;
use async_trait::async_trait;
use uuid::Uuid;


#[derive(Clone)]
pub struct InvoiceUsecase<T: InvoiceRepository + Clone> {
  repository: T,
}

impl<T: InvoiceRepository + Clone> InvoiceUsecase<T> {
  pub fn new(repository: T) -> Self {
    Self { repository }
  }
}

#[async_trait]
pub trait InvoiceService {
  async fn get_all_invoices(&self) -> Result<Vec<Invoice>, sqlx::Error>;
  async fn get_invoice_by_id(&self, id: Uuid) -> Result<Option<Invoice>, sqlx::Error>;
  async fn create_invoice(&self, amount: i32) -> Result<Invoice, sqlx::Error>;
  async fn update_invoice(&self, id: Uuid, amount: i32, paid: bool) -> Result<Invoice, sqlx::Error>;
  async fn delete_invoice(&self, id: Uuid) -> Result<(), sqlx::Error>;
}

#[async_trait]
impl<T: InvoiceRepository + Send + Sync + Clone> InvoiceService for InvoiceUsecase<T> {
  async fn get_all_invoices(&self) -> Result<Vec<Invoice>, sqlx::Error> {
    self.repository.find_all().await
  }

  async fn get_invoice_by_id(&self, id: Uuid) -> Result<Option<Invoice>, sqlx::Error> {
    self.repository.find_by_id(id).await
  }

  async fn create_invoice(&self, amount: i32) -> Result<Invoice, sqlx::Error> {
    let new_invoice = Invoice::new(amount);
    self.repository.create(new_invoice).await
  }

  async fn update_invoice(&self, id: Uuid, amount: i32, paid: bool) -> Result<Invoice, sqlx::Error> {
    let existing_invoice = self.repository.find_by_id(id).await?;
    if let Some(mut invoice) = existing_invoice {
      invoice.amount = amount;
      invoice.paid = paid;
      return self.repository.update(invoice).await;
    }
    Err(sqlx::Error::RowNotFound)
  }

  async fn delete_invoice(&self, id: Uuid) -> Result<(), sqlx::Error> {
    self.repository.delete(id).await
  }
}
