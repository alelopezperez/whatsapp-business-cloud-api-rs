use reqwest::{
    header::{self, AUTHORIZATION},
    Request, StatusCode,
};

use crate::{
    models::{
        BusinessProfileData, BusinessProfileResponse, CodeMethod, CodeRequestParams,
        CodeVerifyParams, ConnectCatalogToWhatsappBusiness, CreateProductCatalogRequest,
        EditItemProduct, ItemProduct, MediaResponse, Message, MessageResponse, MessageStatus,
        MessageStatusResponse, PhoneNumberResponse, ProductCatalog, Success,
        UpdateBusinessProfileResponse,
    },
    WhatsappError,
};

const FACEBOOK_GRAPH_API_BASE_URL: &str = "https://graph.facebook.com";

pub struct WhatsappClient {
    version: String,
    access_token: String,
    //whatsapp_business_id: String,
    client: reqwest::Client,
}

impl WhatsappClient {
    pub fn new(access_token: &str) -> Self {
        let auth_token: String = access_token.into();

        let mut headers = header::HeaderMap::new();
        headers.insert(
            AUTHORIZATION,
            header::HeaderValue::from_str(&format!("Bearer {}", auth_token.as_str()))
                .expect("Error: could not parse auth token into a valid request header."),
        );

        let client = reqwest::Client::builder()
            .default_headers(headers)
            .build()
            .expect("Error: could not initialize PedidosYa API client. Please try again!");

        Self {
            version: "v20.0".into(),
            access_token: access_token.into(),
            client,
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

    pub async fn send_message(
        &self,
        phone_number_id: String,
        message: &Message,
    ) -> Result<MessageResponse, WhatsappError> {
        //http_client::post(&self.messages_api_url(), &self.access_token, message).await
        let url = self.messages_api_url(&phone_number_id);
        let req = self
            .client
            .request(reqwest::Method::POST, url)
            .json(message)
            .build()?;

        self.send_request(req).await
    }

    pub async fn request_code(
        &self,
        phone_number_id: String,
        code_method: CodeMethod,
        language: &str,
    ) -> Result<PhoneNumberResponse, WhatsappError> {
        let params = CodeRequestParams {
            code_method,
            language: language.into(),
        };

        let url = self.messages_api_url(&phone_number_id);
        let req = self
            .client
            .request(reqwest::Method::POST, url)
            .json(&params)
            .build()?;

        self.send_request(req).await
    }

    pub async fn verify_code(
        &self,
        phone_number_id: String,
        code: &str,
    ) -> Result<PhoneNumberResponse, WhatsappError> {
        let params = CodeVerifyParams { code: code.into() };
        let url = self.verify_code_api_url(&phone_number_id);
        let req = self
            .client
            .request(reqwest::Method::POST, url)
            .json(&params)
            .build()?;

        self.send_request(req).await
    }

    pub async fn mark_message_as_read(
        &self,
        phone_number_id: String,
        message_id: &str,
    ) -> Result<MessageStatusResponse, WhatsappError> {
        let message_status = MessageStatus::for_read(message_id);
        let url = self.messages_api_url(&phone_number_id);
        let req = self
            .client
            .request(reqwest::Method::POST, url)
            .json(&message_status)
            .build()?;

        self.send_request(req).await
    }

    pub async fn get_media(&self, media_id: &str) -> Result<MediaResponse, WhatsappError> {
        let url = self.media_api_url(media_id);
        let req = self.client.get(url).build()?;
        self.send_request(req).await
    }

    pub async fn get_business_profile(
        &self,
        phone_number_id: String,
    ) -> Result<BusinessProfileResponse, WhatsappError> {
        let url = self.read_business_profile_url(&phone_number_id);
        let req = self.client.get(url).build()?;

        self.send_request(req).await
    }

    pub async fn update_business_profile(
        &self,
        phone_number_id: String,
        business_profile_data: BusinessProfileData,
    ) -> Result<UpdateBusinessProfileResponse, WhatsappError> {
        let url = self.update_business_profile_url(&phone_number_id);
        let req = self
            .client
            .request(reqwest::Method::POST, url)
            .json(&business_profile_data)
            .build()?;
        self.send_request(req).await
    }

    pub async fn start_upload_img() {}

    pub async fn create_product_catalog(
        &self,
        business_id: String,
        data: CreateProductCatalogRequest,
    ) -> Result<ProductCatalog, WhatsappError> {
        let url = self.owned_product_catalog_url(&business_id);
        let req = self.client.post(url).form(&data).build()?;

        self.send_request(req).await
    }

    pub async fn create_item_product_catalog(
        &self,
        catalog_id: String,
        data: ItemProduct,
    ) -> Result<ProductCatalog, WhatsappError> {
        let url = self.item_product_catalog_url(catalog_id);
        let req = self.client.post(url).form(&data).build()?;

        self.send_request(req).await
    }

    pub async fn delete_item_product_catalog(
        &self,
        product_id: String,
    ) -> Result<Success, WhatsappError> {
        let url = self.delete_item_catalog_url(product_id);
        let req = self.client.delete(url).build()?;

        self.send_request(req).await

        /*http_client::delete(
            &self.delete_item_catalog_url(product_id),
            &self.access_token,
        )
        .await*/
    }

    pub async fn edit_item_product_catalog(
        &self,
        product_id: String,
        edit_item_product: EditItemProduct,
    ) -> Result<Success, WhatsappError> {
        let url = self.delete_item_catalog_url(product_id);
        let req = self
            .client
            .request(reqwest::Method::POST, url)
            .json(&edit_item_product)
            .build()?;

        self.send_request(req).await
    }

    pub async fn connect_catalog_to_whatsapp_business(
        &self,
        whatsapp_business_id: String,
        catalog_id: String,
    ) -> Result<UpdateBusinessProfileResponse, WhatsappError> {
        let data = ConnectCatalogToWhatsappBusiness { catalog_id };
        let url = self.product_catalogs_url(&whatsapp_business_id);
        let req = self
            .client
            .request(reqwest::Method::POST, url)
            .json(&data)
            .build()?;

        self.send_request(req).await
    }

    fn product_catalogs_url(&self, whatsapp_business_id: &str) -> String {
        // {whatsapp_business_id}/product_catalogs'
        format!(
            "{}/{}/product_catalogs",
            self.facebook_api_version_url(),
            whatsapp_business_id,
        )
    }

    fn delete_item_catalog_url(&self, product_item_id: String) -> String {
        // {whatsapp_business_id}/product_catalogs'
        // curl -X DELETE  'https://graph.facebook.com/v22.0/9725215490925023'
        format!("{}/{}", self.facebook_api_version_url(), product_item_id)
    }

    fn facebook_api_version_url(&self) -> String {
        format!("{FACEBOOK_GRAPH_API_BASE_URL}/{}", self.version)
    }

    fn messages_api_url(&self, phone_number_id: &str) -> String {
        format!(
            "{}/{}/messages",
            self.facebook_api_version_url(),
            phone_number_id
        )
    }

    fn media_api_url(&self, media_id: &str) -> String {
        format!("{}/{media_id}", self.facebook_api_version_url())
    }

    fn request_code_api_url(&self, phone_number_id: &str) -> String {
        format!(
            "{}/{}/request_code",
            self.facebook_api_version_url(),
            phone_number_id
        )
    }

    fn verify_code_api_url(&self, phone_number_id: &str) -> String {
        format!(
            "{}/{}/verify_code",
            self.facebook_api_version_url(),
            phone_number_id
        )
    }

    fn read_business_profile_url(&self, phone_number_id: &str) -> String {
        let url = format!(
            "{}/{}/whatsapp_business_profile?fields=about,address,description,email,profile_picture_url,websites,vertical",
            self.facebook_api_version_url(),
            phone_number_id,
        );
        url
    }
    fn update_business_profile_url(&self, phone_number_id: &str) -> String {
        let url = format!(
            "{}/{}/whatsapp_business_profile",
            self.facebook_api_version_url(),
            phone_number_id,
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

    fn owned_product_catalog_url(&self, business_id: &str) -> String {
        let url = format!(
            "{}/{}/owned_product_catalogs",
            self.facebook_api_version_url(),
            business_id,
        );
        url
    }

    async fn send_request<Res>(&self, request: Request) -> Result<Res, WhatsappError>
    where
        Res: serde::de::DeserializeOwned,
    {
        let response = self.client.execute(request).await?;

        match response.status() {
            StatusCode::OK => {
                let data = response.json::<Res>().await?;
                Ok(data)
            }
            _ => {
                log::warn!("{:?}", &response);
                let error_text = &response.text().await?;
                log::warn!("{:?}", &error_text);
                Err(WhatsappError::UnexpectedError(error_text.to_string()))
            }
        }
    }
}
