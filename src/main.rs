use axum::{Router, routing::get};
use dotenvy::dotenv;
use sqlx::PgPool;
use std::env;
use std::net::SocketAddr;
use tokio::net::TcpListener;
use tracing::{info, Level};
use tracing_subscriber::FmtSubscriber;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

use crate::infrastructure::todo_repository::TodoRepositoryImpl;
use crate::infrastructure::invoice_repository::InvoiceRepositoryImpl;
use crate::presentation::handlers::todo_handler::create_todo_router;
use crate::presentation::handlers::invoice_handler::create_invoice_router;
use crate::usecase::todo_usecase::TodoUsecase;
use crate::usecase::invoice_usecase::InvoiceUsecase;

mod domain;
mod infrastructure;
mod presentation;
mod usecase;

#[derive(OpenApi)]
#[openapi(
    paths(
        presentation::handlers::todo_handler::get_all_todos,
        presentation::handlers::todo_handler::get_todo_by_id,
        presentation::handlers::todo_handler::create_todo,
        presentation::handlers::todo_handler::update_todo,
        presentation::handlers::todo_handler::delete_todo,
        presentation::handlers::invoice_handler::get_all_invoices,
        presentation::handlers::invoice_handler::get_invoice_by_id,
        presentation::handlers::invoice_handler::create_invoice,
        presentation::handlers::invoice_handler::update_invoice,
        presentation::handlers::invoice_handler::delete_invoice,
    ),
    tags(
        (name = "todos", description = "Todo API"),
        (name = "invoices", description = "Invoice API")
    )
)]
struct ApiDoc;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();

    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::INFO)
        .finish();
    tracing::subscriber::set_global_default(subscriber)?;

    let database_url = env::var("DATABASE_URL")?;
    let pool = PgPool::connect(&database_url).await?;

    let todo_repository = TodoRepositoryImpl::new(pool.clone());
    let todo_service = TodoUsecase::new(todo_repository);

    let invoice_repository = InvoiceRepositoryImpl::new(pool.clone());
    let invoice_service = InvoiceUsecase::new(invoice_repository);

    let app = Router::new()
        .merge(SwaggerUi::new("/swagger-ui").url("/api-docs/openapi.json", ApiDoc::openapi()))
        .route("/", get(|| async { "Hello, Axum!!!!" }))
        .nest("/api", create_todo_router(todo_service)
            .merge(create_invoice_router(invoice_service)));

    let addr = SocketAddr::from(([127, 0, 0, 1], 3001));
    info!("Server running at http://{}", addr);
    let listener = TcpListener::bind(addr).await?;
    axum::serve(listener, app.into_make_service()).await?;

    Ok(())
}
