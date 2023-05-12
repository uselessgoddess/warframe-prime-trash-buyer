use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Order {
    pub visible: bool,
    pub creation_date: String,
    pub quantity: i32,
    pub user: crate::warframe_market::User,
    pub last_update: String,
    pub platinum: i32,
    pub order_type: String,
    pub platform: String,
    pub id: String,
    pub region: String,
    pub item: Option<crate::warframe_market::ItemsItem>
}

impl From<crate::warframe_market::Order> for Order{
    fn from(order: crate::warframe_market::Order) -> Self {
        Order {
            visible: order.visible,
            creation_date: order.creation_date,
            quantity: order.quantity,
            user: crate::warframe_market::User::from(order.user),
            last_update: order.last_update,
            platinum: order.platinum,
            order_type: order.order_type,
            platform: order.platform,
            id: order.id,
            region: order.region,
            item: None
        }
    }
}