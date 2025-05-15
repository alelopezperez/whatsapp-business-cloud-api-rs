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

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ProductCatalog {
    pub id: String,
}
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Success {
    pub success: bool,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct EditItemProduct {
    pub name: Option<String>,
    pub price: Option<i32>,
    pub image_url: Option<String>,
}
