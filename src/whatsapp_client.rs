use crate::{
    models::{
        BusinessProfileData, BusinessProfileResponse, CodeMethod, CodeRequestParams,
        CodeVerifyParams, CreateProductCatalogRequest, ItemProduct, MediaResponse, Message,
        MessageResponse, MessageStatus, MessageStatusResponse, PhoneNumberResponse,
        UpdateBusinessProfileResponse,
    },
    WhatsappError,
};

const FACEBOOK_GRAPH_API_BASE_URL: &str = "https://graph.facebook.com";

pub struct WhatsappClient {
    version: String,
    access_token: String,
    phone_number_id: String,
    business_id: String,
    whatsapp_business_id: String,
}

impl WhatsappClient {
    pub fn new(access_token: &str, phone_number_id: &str) -> Self {
        Self {
            version: "v20.0".into(),
            access_token: access_token.into(),
            phone_number_id: phone_number_id.into(),
            business_id: "".to_string(),
            whatsapp_business_id: "".to_string(),
        }
    }
    pub fn new_with_business(access_token: &str, phone_number_id: &str, business_id: &str) -> Self {
        Self {
            version: "v20.0".into(),
            access_token: access_token.into(),
            phone_number_id: phone_number_id.into(),
            business_id: business_id.into(),
            whatsapp_business_id: "".to_string(),
        }
    }

    pub fn new_with_whatsapp_business(
        access_token: &str,
        phone_number_id: &str,
        business_id: &str,
        whatsapp_business_id: &str,
    ) -> Self {
        Self {
            version: "v20.0".into(),
            access_token: access_token.into(),
            phone_number_id: phone_number_id.into(),
            business_id: business_id.into(),
            whatsapp_business_id: whatsapp_business_id.to_string(),
        }
    }

    pub fn version(&mut self) -> &str {
        &self.version
    }

    pub fn set_version(&mut self, version: &str) {
        self.version = version.into();
    }

    pub fn set_access_token(&mut self, access_token: &str) {
        self.access_token = access_token.into();
    }

    pub fn set_phone_number_id(&mut self, phone_number_id: &str) {
        self.access_token = phone_number_id.into();
    }

    pub async fn send_message(&self, message: &Message) -> Result<MessageResponse, WhatsappError> {
        http_client::post(&self.messages_api_url(), &self.access_token, message).await
    }

    pub async fn request_code(
        &self,
        code_method: CodeMethod,
        language: &str,
    ) -> Result<PhoneNumberResponse, WhatsappError> {
        let params = CodeRequestParams {
            code_method,
            language: language.into(),
        };
        http_client::post(&self.request_code_api_url(), &self.access_token, &params).await
    }

    pub async fn verify_code(&self, code: &str) -> Result<PhoneNumberResponse, WhatsappError> {
        let params = CodeVerifyParams { code: code.into() };
        http_client::post(&self.verify_code_api_url(), &self.access_token, &params).await
    }

    pub async fn mark_message_as_read(
        &self,
        message_id: &str,
    ) -> Result<MessageStatusResponse, WhatsappError> {
        let message_status = MessageStatus::for_read(message_id);
        http_client::post(
            &self.messages_api_url(),
            &self.access_token,
            &message_status,
        )
        .await
    }

    pub async fn get_media(&self, media_id: &str) -> Result<MediaResponse, WhatsappError> {
        http_client::get(&self.media_api_url(media_id), &self.access_token).await
    }

    pub async fn get_business_profile(&self) -> Result<BusinessProfileResponse, WhatsappError> {
        let ans = http_client::get(&self.read_business_profile_url(), &self.access_token).await?;
        Ok(ans)
    }

    pub async fn update_business_profile(
        &self,
        business_profile_data: BusinessProfileData,
    ) -> Result<UpdateBusinessProfileResponse, WhatsappError> {
        http_client::post(
            &self.update_business_profile_url(),
            &self.access_token,
            &business_profile_data,
        )
        .await
    }

    pub async fn create_product_catalog(
        &self,
        data: CreateProductCatalogRequest,
    ) -> Result<serde_json::Value, WhatsappError> {
        http_client::post_form(&self.owned_product_catalog_url(), &self.access_token, &data).await
    }

    pub async fn create_item_product_catalog(
        &self,
        catalog_id: String,
        data: ItemProduct,
    ) -> Result<serde_json::Value, WhatsappError> {
        http_client::post_form(
            &self.item_product_catalog_url(catalog_id),
            &self.access_token,
            &data,
        )
        .await
    }

    fn facebook_api_version_url(&self) -> String {
        format!("{FACEBOOK_GRAPH_API_BASE_URL}/{}", self.version)
    }

    fn messages_api_url(&self) -> String {
        format!(
            "{}/{}/messages",
            self.facebook_api_version_url(),
            self.phone_number_id
        )
    }

    fn media_api_url(&self, media_id: &str) -> String {
        format!("{}/{media_id}", self.facebook_api_version_url())
    }

    fn request_code_api_url(&self) -> String {
        format!(
            "{}/{}/request_code",
            self.facebook_api_version_url(),
            self.phone_number_id
        )
    }

    fn verify_code_api_url(&self) -> String {
        format!(
            "{}/{}/verify_code",
            self.facebook_api_version_url(),
            self.phone_number_id
        )
    }

    fn read_business_profile_url(&self) -> String {
        let url = format!(
            "{}/{}/whatsapp_business_profile?fields=about,address,description,email,profile_picture_url,websites,vertical",
            self.facebook_api_version_url(),
            self.phone_number_id,
        );
        url
    }
    fn update_business_profile_url(&self) -> String {
        let url = format!(
            "{}/{}/whatsapp_business_profile",
            self.facebook_api_version_url(),
            self.phone_number_id,
        );
        url
    }

    fn item_product_catalog_url(&self, catalog_id: String) -> String {
        format!(
            "{}/{}/products",
            self.facebook_api_version_url(),
            catalog_id
        )
    }

    fn owned_product_catalog_url(&self) -> String {
        let url = format!(
            "{}/{}/owned_product_catalogs",
            self.facebook_api_version_url(),
            self.business_id,
        );
        url
    }
}

mod http_client {
    use reqwest::StatusCode;
    use serde::{de::DeserializeOwned, Serialize};

    use crate::WhatsappError;

    pub async fn get<U>(url: &str, bearer_token: &str) -> Result<U, WhatsappError>
    where
        U: DeserializeOwned,
    {
        let client = reqwest::Client::new();
        let resp = client.get(url).bearer_auth(bearer_token).send().await?;

        match resp.status() {
            StatusCode::OK => {
                let json = resp.json::<U>().await?;
                Ok(json)
            }
            _ => {
                log::warn!("{:?}", &resp);
                let error_text = &resp.text().await?;
                log::warn!("{:?}", &error_text);
                Err(WhatsappError::UnexpectedError(error_text.to_string()))
            }
        }
    }

    pub async fn post<T, U>(url: &str, bearer_token: &str, data: &T) -> Result<U, WhatsappError>
    where
        T: Serialize + ?Sized,
        U: DeserializeOwned,
    {
        let client = reqwest::Client::new();
        let resp = client
            .post(url)
            .bearer_auth(bearer_token)
            .json(&data)
            .send()
            .await?;

        match resp.status() {
            StatusCode::OK | StatusCode::CREATED => {
                let json = resp.json::<U>().await?;
                Ok(json)
            }
            _ => {
                log::warn!("{:?}", &resp);
                let error_text = &resp.text().await?;
                log::warn!("{:?}", &error_text);
                Err(WhatsappError::UnexpectedError(error_text.to_string()))
            }
        }
    }
    pub async fn post_form<T, U>(
        url: &str,
        bearer_token: &str,
        data: &T,
    ) -> Result<U, WhatsappError>
    where
        T: Serialize + ?Sized,
        U: DeserializeOwned,
    {
        let client = reqwest::Client::new();
        let resp = client
            .post(url)
            .bearer_auth(bearer_token)
            .form(&data)
            .send()
            .await?;

        match resp.status() {
            StatusCode::OK | StatusCode::CREATED => {
                let json = resp.json::<U>().await?;
                Ok(json)
            }
            _ => {
                log::warn!("{:?}", &resp);
                let error_text = &resp.text().await?;
                log::warn!("{:?}", &error_text);
                Err(WhatsappError::UnexpectedError(error_text.to_string()))
            }
        }
    }
}
