use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CreateProductCatalogRequest {
    pub name: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ItemProduct {
    pub retailer_id: String,
    pub name: String,
    pub price: i64,
    pub currency: String,
    pub image_url: String,
}
