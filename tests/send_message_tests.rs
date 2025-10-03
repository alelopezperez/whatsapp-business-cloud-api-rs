use whatsapp_business_cloud_api::{
    models::{
        Component, ComponentType, Image, Interactive, InteractiveActionButton,
        InteractiveActionSection, InteractiveActionSectionRow, Message, Parameter, Template, Text,
    },
    WhatsappClient, WhatsappError,
};

fn setup() {
    dotenv::dotenv().ok();
    let _ = env_logger::builder().is_test(true).try_init();
}
