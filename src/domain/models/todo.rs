use serde::{Serialize, Deserialize};
use uuid::Uuid;
use chrono::{DateTime, Utc, FixedOffset, TimeZone};
use sqlx::FromRow;


#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Todo {
  pub id: Uuid,
  pub title: String,
  pub description: Option<String>,
  pub completed: bool,
  pub created_at: DateTime<Utc>,
  pub updated_at: DateTime<Utc>,
}

impl Todo {
  pub fn new(title: String, description: String) -> Self {
    // 日本時間のオフセット（UTC+9時間）
    let jst = FixedOffset::east_opt(9 * 3600).unwrap();
    // 現在の日本時間を取得し、UTCに変換
    let now_jst = jst.from_utc_datetime(&Utc::now().naive_utc());
    let now_utc = now_jst.with_timezone(&Utc);
        
    Self {
      id: Uuid::now_v7(),
      title,
      description: Some(description),
      completed: false,
      created_at: now_utc,
      updated_at: now_utc
    }
  }
}