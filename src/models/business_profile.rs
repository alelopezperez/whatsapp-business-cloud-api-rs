use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct BusinessProfileResponse {
    pub data: Vec<BusinessProfileData>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct BusinessProfileData {
    pub about: Option<String>,
    pub address: Option<String>,
    pub description: Option<String>,
    pub email: Option<String>,
    #[serde(default = "default_messaging_product")]
    pub messaging_product: String,
    pub profile_picture_url: Option<String>,
    pub vertical: Option<String>,
    pub websites: Option<Vec<String>>,
}
fn default_messaging_product() -> String {
    "whatsapp".to_string()
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct UpdateBusinessProfileResponse {
    pub success: bool,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ConnectCatalogToWhatsappBusiness {
    pub catalog_id: String,
}
