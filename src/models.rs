use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct HealthResponse {
    pub status: &'static str,
    pub service: &'static str,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct Gift {
    pub id: i64,
    pub slug: String,
    pub name: String,
    pub description: String,
    pub image_url: Option<String>,
    pub price: i64,
    pub rarity_level: String,
    pub is_available: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct CreateGiftRequest {
    pub slug: String,
    pub name: String,
    pub description: String,
    pub image_url: Option<String>,
    pub price: i64,
    pub rarity_level: String,
    pub is_available: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct UpdateGiftRequest {
    pub slug: Option<String>,
    pub name: Option<String>,
    pub description: Option<String>,
    pub image_url: Option<String>,
    pub price: Option<i64>,
    pub rarity_level: Option<String>,
    pub is_available: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct User {
    pub tg_id: i64,
    pub username: Option<String>,
    pub wallet_address: Option<String>,
    pub created_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct UpsertUserRequest {
    pub tg_id: i64,
    pub username: Option<String>,
    pub wallet_address: Option<String>,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "lowercase")]
pub enum OrderStatus {
    Pending,
    Paid,
    Delivered,
}

impl OrderStatus {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Pending => "pending",
            Self::Paid => "paid",
            Self::Delivered => "delivered",
        }
    }

    pub fn parse(value: &str) -> Option<Self> {
        match value {
            "pending" => Some(Self::Pending),
            "paid" => Some(Self::Paid),
            "delivered" => Some(Self::Delivered),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct Order {
    pub id: i64,
    pub user_id: i64,
    pub gift_id: i64,
    pub status: OrderStatus,
    pub tx_hash: Option<String>,
    pub created_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct PurchaseRequest {
    pub user_id: i64,
    pub gift_id: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct PurchaseResponse {
    pub order: Order,
    pub gift: Gift,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct UpdateOrderStatusRequest {
    pub status: OrderStatus,
    pub tx_hash: Option<String>,
}
