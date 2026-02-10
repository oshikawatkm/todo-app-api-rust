use axum::{
    extract::{Path, State},
    response::IntoResponse,
    routing::get,
    Json, Router,
};
use http::StatusCode;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use uuid::Uuid;
use utoipa::ToSchema;

use crate::usecase::invoice_usecase::InvoiceService;
use crate::domain::models::invoice::Invoice;


#[derive(Clone)]
pub struct AppState<T: InvoiceService> {
  pub invoice_service: Arc<T>,
}

pub fn create_invoice_router<T: InvoiceService + Send + Sync + 'static + Clone>(invoice_service: T) -> Router {
  let state = AppState {
    invoice_service: Arc::new(invoice_service),
  };

  Router::new()
    .route("/invoices", get(get_all_invoices::<T>).post(create_invoice::<T>))
    .route("/invoices/{id}", get(get_invoice_by_id::<T>)
      .put(update_invoice::<T>)
      .delete(delete_invoice::<T>))
    .with_state(state)
}

#[derive(Deserialize, ToSchema)]
pub struct CreateInvoiceRequest {
  amount: i32,
}

#[derive(Deserialize, ToSchema)]
pub struct UpdateInvoiceRequest {
  amount: i32,
  paid: bool,
}

#[derive(Serialize, ToSchema)]
struct InvoiceResponse {
  id: Uuid,
  amount: i32,
  paid: bool,
}

impl From<Invoice> for InvoiceResponse {
  fn from(invoice: Invoice) -> Self {
    Self {
      id: invoice.id,
      amount: invoice.amount,
      paid: invoice.paid,
    }
  }
}


#[utoipa::path(
    get,
    path = "/api/invoices",
    responses(
        (status = 200, description = "全請求書を取得", body = Vec<InvoiceResponse>),
        (status = 500, description = "サーバーエラー")
    ),
    tag = "invoices"
)]
pub async fn get_all_invoices<T: InvoiceService>(
  State(state): State<AppState<T>>,
) -> impl IntoResponse {
  match state.invoice_service.get_all_invoices().await {
    Ok(invoices) => {
      let response: Vec<InvoiceResponse> = invoices.into_iter().map(InvoiceResponse::from).collect();
      Json(response).into_response()
    }
    Err(_) => (StatusCode::INTERNAL_SERVER_ERROR, "Failed to fetch invoices").into_response(),
  }
}

#[utoipa::path(
    get,
    path = "/api/invoices/{id}",
    params(("id" = Uuid, Path, description = "Invoice ID")),
    responses(
        (status = 200, description = "請求書を取得", body = InvoiceResponse),
        (status = 404, description = "請求書が見つからない"),
        (status = 500, description = "サーバーエラー")
    ),
    tag = "invoices"
)]
pub async fn get_invoice_by_id<T: InvoiceService>(
  State(state): State<AppState<T>>,
  Path(id): Path<Uuid>,
) -> impl IntoResponse {
  match state.invoice_service.get_invoice_by_id(id).await {
    Ok(Some(invoice)) => Json(InvoiceResponse::from(invoice)).into_response(),
    Ok(None) => (StatusCode::NOT_FOUND, "Invoice not found").into_response(),
    Err(_) => (StatusCode::INTERNAL_SERVER_ERROR, "Failed to fetch invoice").into_response(),
  }
}

#[utoipa::path(
    post,
    path = "/api/invoices",
    request_body = CreateInvoiceRequest,
    responses(
        (status = 201, description = "請求書を作成", body = InvoiceResponse),
        (status = 500, description = "サーバーエラー")
    ),
    tag = "invoices"
)]
pub async fn create_invoice<T: InvoiceService>(
  State(state): State<AppState<T>>,
  Json(payload): Json<CreateInvoiceRequest>,
) -> impl IntoResponse {
  match state.invoice_service.create_invoice(payload.amount).await {
    Ok(invoice) => (StatusCode::CREATED, Json(InvoiceResponse::from(invoice))).into_response(),
    Err(_) => (StatusCode::INTERNAL_SERVER_ERROR, "Failed to create invoice").into_response(),
  }
}

#[utoipa::path(
    put,
    path = "/api/invoices/{id}",
    params(("id" = Uuid, Path, description = "Invoice ID")),
    request_body = UpdateInvoiceRequest,
    responses(
        (status = 200, description = "請求書を更新", body = InvoiceResponse),
        (status = 404, description = "請求書が見つからない"),
        (status = 500, description = "サーバーエラー")
    ),
    tag = "invoices"
)]
pub async fn update_invoice<T: InvoiceService>(
  State(state): State<AppState<T>>,
  Path(id): Path<Uuid>,
  Json(payload): Json<UpdateInvoiceRequest>,
) -> impl IntoResponse {
  match state.invoice_service.update_invoice(id, payload.amount, payload.paid).await {
    Ok(invoice) => Json(InvoiceResponse::from(invoice)).into_response(),
    Err(sqlx::Error::RowNotFound) => (StatusCode::NOT_FOUND, "Invoice not found").into_response(),
    Err(_) => (StatusCode::INTERNAL_SERVER_ERROR, "Failed to update invoice").into_response(),
  }
}

#[utoipa::path(
    delete,
    path = "/api/invoices/{id}",
    params(("id" = Uuid, Path, description = "Invoice ID")),
    responses(
        (status = 204, description = "請求書を削除"),
        (status = 404, description = "請求書が見つからない"),
        (status = 500, description = "サーバーエラー")
    ),
    tag = "invoices"
)]
pub async fn delete_invoice<T: InvoiceService>(
  State(state): State<AppState<T>>,
  Path(id): Path<Uuid>,
) -> impl IntoResponse {
  match state.invoice_service.delete_invoice(id).await {
    Ok(_) => StatusCode::NO_CONTENT.into_response(),
    Err(sqlx::Error::RowNotFound) => (StatusCode::NOT_FOUND, "Invoice not found").into_response(),
    Err(_) => (StatusCode::INTERNAL_SERVER_ERROR, "Failed to delete invoice").into_response(),
  }
}
