use whatsapp_business_cloud_api::{models::CodeMethod, WhatsappClient, WhatsappError};

fn setup() {
    dotenv::dotenv().ok();
    let _ = env_logger::builder().is_test(true).try_init();
}
