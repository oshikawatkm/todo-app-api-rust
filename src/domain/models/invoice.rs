use serde::{Serialize, Deserialize};
use uuid::Uuid;
use chrono::{DateTime, Utc, FixedOffset, TimeZone};
use sqlx::FromRow;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Invoice {
  pub id: Uuid,
  pub amount: i32,
  pub paid: bool,
  pub created_at: DateTime<Utc>,
  pub updated_at: DateTime<Utc>,
}

impl Invoice {
  pub fn new(amount: i32) -> Self {
    let jst = FixedOffset::east_opt(9 * 3600).unwrap();
    let now_jst = jst.from_utc_datetime(&Utc::now().naive_utc());
    let now_utc = now_jst.with_timezone(&Utc);
    
    Self {
      id: Uuid::now_v7(),
      amount,
      paid: false,
      created_at: now_utc,
      updated_at: now_utc
    }
  }
}