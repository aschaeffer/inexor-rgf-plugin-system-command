use std::env;
use std::sync::Arc;
use std::sync::RwLock;

use async_trait::async_trait;
use log::debug;

use crate::behaviour::component::component_behaviour_provider::SystemCommandComponentBehaviourProviderImpl;
use crate::di::*;
use crate::plugins::plugin::PluginMetadata;
use crate::plugins::plugin_context::PluginContext;
use crate::plugins::ComponentBehaviourProvider;
use crate::plugins::ComponentProvider;
use crate::plugins::EntityBehaviourProvider;
use crate::plugins::EntityTypeProvider;
use crate::plugins::FlowProvider;
use crate::plugins::Plugin;
use crate::plugins::PluginError;
use crate::plugins::RelationBehaviourProvider;
use crate::plugins::RelationTypeProvider;
use crate::plugins::WebResourceProvider;
use crate::provider::SystemCommandComponentProviderImpl;
use crate::provider::SystemCommandEntityTypeProviderImpl;

#[wrapper]
pub struct PluginContextContainer(RwLock<Option<Arc<dyn PluginContext>>>);

#[provides]
fn create_empty_plugin_context_container() -> PluginContextContainer {
    PluginContextContainer(RwLock::new(None))
}

#[async_trait]
pub trait SystemCommandPlugin: Plugin + Send + Sync {}

#[module]
pub struct SystemCommandPluginImpl {
    component_provider: Wrc<SystemCommandComponentProviderImpl>,
    component_behaviour_provider: Wrc<SystemCommandComponentBehaviourProviderImpl>,
    entity_type_provider: Wrc<SystemCommandEntityTypeProviderImpl>,

    context: PluginContextContainer,
}

impl SystemCommandPluginImpl {}

impl SystemCommandPluginImpl {}

interfaces!(SystemCommandPluginImpl: dyn Plugin);

#[async_trait]
#[provides]
impl SystemCommandPlugin for SystemCommandPluginImpl {}

impl Plugin for SystemCommandPluginImpl {
    fn metadata(&self) -> Result<PluginMetadata, PluginError> {
        Ok(PluginMetadata {
            name: env!("CARGO_PKG_NAME").into(),
            description: env!("CARGO_PKG_DESCRIPTION").into(),
            version: env!("CARGO_PKG_VERSION").into(),
        })
    }

    fn init(&self) -> Result<(), PluginError> {
        debug!("SystemCommandPluginModuleImpl::init()");
        Ok(())
    }

    fn post_init(&self) -> Result<(), PluginError> {
        debug!("SystemCommandPluginModuleImpl::post_init()");
        Ok(())
    }

    fn pre_shutdown(&self) -> Result<(), PluginError> {
        debug!("SystemCommandPluginModuleImpl::pre_shutdown()");
        Ok(())
    }

    fn shutdown(&self) -> Result<(), PluginError> {
        debug!("SystemCommandPluginModuleImpl::shutdown()");
        Ok(())
    }

    fn set_context(&self, context: Arc<dyn PluginContext>) -> Result<(), PluginError> {
        self.context.0.write().unwrap().replace(context.clone());
        Ok(())
    }

    fn get_component_provider(&self) -> Result<Arc<dyn ComponentProvider>, PluginError> {
        let component_provider = self.component_provider.clone();
        let component_provider: Result<Arc<dyn ComponentProvider>, _> = <dyn query_interface::Object>::query_arc(component_provider);
        if component_provider.is_err() {
            return Err(PluginError::NoComponentProvider);
        }
        Ok(component_provider.unwrap())
    }

    fn get_entity_type_provider(&self) -> Result<Arc<dyn EntityTypeProvider>, PluginError> {
        let entity_type_provider = self.entity_type_provider.clone();
        let entity_type_provider: Result<Arc<dyn EntityTypeProvider>, _> = <dyn query_interface::Object>::query_arc(entity_type_provider);
        if entity_type_provider.is_err() {
            return Err(PluginError::NoEntityTypeProvider);
        }
        Ok(entity_type_provider.unwrap())
    }

    fn get_relation_type_provider(&self) -> Result<Arc<dyn RelationTypeProvider>, PluginError> {
        Err(PluginError::NoRelationTypeProvider)
    }

    fn get_component_behaviour_provider(&self) -> Result<Arc<dyn ComponentBehaviourProvider>, PluginError> {
        let component_behaviour_provider = self.component_behaviour_provider.clone();
        let component_behaviour_provider: Result<Arc<dyn ComponentBehaviourProvider>, _> =
            <dyn query_interface::Object>::query_arc(component_behaviour_provider);
        if component_behaviour_provider.is_err() {
            return Err(PluginError::NoComponentBehaviourProvider);
        }
        Ok(component_behaviour_provider.unwrap())
    }

    fn get_entity_behaviour_provider(&self) -> Result<Arc<dyn EntityBehaviourProvider>, PluginError> {
        Err(PluginError::NoEntityBehaviourProvider)
    }

    fn get_relation_behaviour_provider(&self) -> Result<Arc<dyn RelationBehaviourProvider>, PluginError> {
        Err(PluginError::NoRelationBehaviourProvider)
    }

    fn get_flow_provider(&self) -> Result<Arc<dyn FlowProvider>, PluginError> {
        Err(PluginError::NoFlowProvider)
    }

    fn get_web_resource_provider(&self) -> Result<Arc<dyn WebResourceProvider>, PluginError> {
        Err(PluginError::NoWebResourceProvider)
    }
}
