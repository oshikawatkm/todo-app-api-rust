use crate::domain::models::invoice::Invoice;
use crate::domain::repositories::invoice_repository::InvoiceRepository;
use crate::infrastructure::db::DbPool;
use async_trait::async_trait;
use uuid::Uuid;

#[derive(Clone)]
pub struct InvoiceRepositoryImpl {
  pub pool: DbPool,
}

impl InvoiceRepositoryImpl {
  pub fn new(pool: DbPool) -> Self {
    Self { pool }
  }
}


#[async_trait]
impl InvoiceRepository for InvoiceRepositoryImpl {
  async fn find_all(&self) -> Result<Vec<Invoice>, sqlx::Error> {
    let invoices = sqlx::query_as::<_, Invoice>(
      "SELECT id, amount, paid, created_at, updated_at FROM invoices"
    )
    .fetch_all(&self.pool)
    .await?;
    Ok(invoices)
  }

  async fn find_by_id(&self, id: Uuid) -> Result<Option<Invoice>, sqlx::Error> {
    let invoice = sqlx::query_as::<_, Invoice>(
      "SELECT id, amount, paid, created_at, updated_at FROM invoices WHERE id = $1"
    )
    .bind(id)
    .fetch_optional(&self.pool)
    .await?;
    Ok(invoice)
  }

  async fn create(&self, invoice: Invoice) -> Result<Invoice, sqlx::Error> {
    let created_invoice = sqlx::query_as::<_, Invoice>(
        "INSERT INTO invoices (id, amount, paid, created_at, updated_at)
          VALUES ($1, $2, $3, $4, $5)
          RETURNING id, amount, paid, created_at, updated_at"
    )
    .bind(invoice.id)
    .bind(&invoice.amount)
    .bind(invoice.paid)
    .bind(invoice.created_at)
    .bind(invoice.updated_at)
    .fetch_one(&self.pool)
    .await?;
    Ok(created_invoice)
  }

  async fn update(&self, invoice: Invoice) -> Result<Invoice, sqlx::Error> {
    let updated_invoice = sqlx::query_as::<_, Invoice>(
        "UPDATE invoices SET amount = $1, paid = $2, updated_at = (NOW() AT TIME ZONE 'Asia/Tokyo')
          WHERE id = $3
          RETURNING id, amount, paid, created_at, updated_at"
    )
    .bind(&invoice.amount)
    .bind(invoice.paid)
    .bind(invoice.id)
    .fetch_one(&self.pool)
    .await?;
    Ok(updated_invoice)
  }

  async fn delete(&self, id: Uuid) -> Result<(), sqlx::Error> {
    sqlx::query("DELETE FROM invoices WHERE id = $1")
        .bind(id)
        .execute(&self.pool)
        .await?;
    Ok(())
  }
}
