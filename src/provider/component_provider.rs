use crate::di::*;
use async_trait::async_trait;
use log::debug;
use log::error;
use rust_embed::RustEmbed;

use crate::plugins::ComponentProvider;

use crate::model::component::Component;

#[derive(RustEmbed)]
#[folder = "./assets/types/components"]
struct SystemCommandComponentAsset;

#[async_trait]
pub trait SystemCommandComponentProvider: ComponentProvider + Send + Sync {}

#[derive(Clone)]
pub struct SystemCommandComponentProviderImpl {}

interfaces!(SystemCommandComponentProviderImpl: dyn ComponentProvider);

#[component]
impl SystemCommandComponentProviderImpl {
    #[provides]
    fn new() -> Self {
        Self {}
    }
}

#[async_trait]
#[provides]
impl SystemCommandComponentProvider for SystemCommandComponentProviderImpl {}

impl ComponentProvider for SystemCommandComponentProviderImpl {
    fn get_components(&self) -> Vec<Component> {
        let mut components = Vec::new();
        for file in SystemCommandComponentAsset::iter() {
            let filename = file.as_ref();
            debug!("Loading component from resource {}", filename);
            let asset = SystemCommandComponentAsset::get(filename).unwrap();
            let json_str = std::str::from_utf8(asset.data.as_ref());
            if json_str.is_err() {
                error!("Could not decode UTF-8 {}", filename);
                continue;
            }
            let component: Component = match serde_json::from_str(json_str.unwrap()) {
                Result::Ok(component) => component,
                Result::Err(err) => {
                    error!("Error in parsing JSON file {}: {}", filename, err);
                    continue;
                }
            };
            components.push(component);
        }
        components
    }
}
