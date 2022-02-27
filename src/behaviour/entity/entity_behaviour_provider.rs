use std::sync::Arc;

use crate::di::*;
use async_trait::async_trait;
use log::debug;
use uuid::Uuid;

use crate::behaviour::entity::system_command::SystemCommand;
use crate::model::ReactiveEntityInstance;
use crate::plugins::EntityBehaviourProvider;

const SYSTEM_COMMAND: &str = "system_command";

#[wrapper]
pub struct SystemCommandStorage(std::sync::RwLock<std::collections::HashMap<Uuid, std::sync::Arc<SystemCommand>>>);

#[provides]
fn create_system_command_storage() -> SystemCommandStorage {
    SystemCommandStorage(std::sync::RwLock::new(std::collections::HashMap::new()))
}

#[async_trait]
pub trait SystemCommandEntityBehaviourProvider: EntityBehaviourProvider + Send + Sync {
    fn create_system_command(&self, entity_instance: Arc<ReactiveEntityInstance>);

    fn remove_system_command(&self, entity_instance: Arc<ReactiveEntityInstance>);

    fn remove_by_id(&self, id: Uuid);
}

// #[derive(Clone)]
pub struct SystemCommandEntityBehaviourProviderImpl {
    system_command: SystemCommandStorage,
}

interfaces!(SystemCommandEntityBehaviourProviderImpl: dyn EntityBehaviourProvider);

#[component]
impl SystemCommandEntityBehaviourProviderImpl {
    #[provides]
    fn new() -> Self {
        Self {
            system_command: create_system_command_storage(),
        }
    }
}

#[async_trait]
#[provides]
impl SystemCommandEntityBehaviourProvider for SystemCommandEntityBehaviourProviderImpl {
    fn create_system_command(&self, entity_instance: Arc<ReactiveEntityInstance>) {
        let id = entity_instance.id;
        if let Ok(device_key) = SystemCommand::new(entity_instance.clone()) {
            let system_command = Arc::new(device_key);
            self.system_command.0.write().unwrap().insert(id, system_command);
            entity_instance.add_behaviour(SYSTEM_COMMAND);
            debug!("Added behaviour {} to entity instance {}", SYSTEM_COMMAND, id);
        }
    }

    fn remove_system_command(&self, entity_instance: Arc<ReactiveEntityInstance>) {
        self.system_command.0.write().unwrap().remove(&entity_instance.id);
        entity_instance.remove_behaviour(SYSTEM_COMMAND);
        debug!("Removed behaviour {} from entity instance {}", SYSTEM_COMMAND, entity_instance.id);
    }

    fn remove_by_id(&self, id: Uuid) {
        if self.system_command.0.write().unwrap().contains_key(&id) {
            self.system_command.0.write().unwrap().remove(&id);
            debug!("Removed behaviour {} from entity instance {}", SYSTEM_COMMAND, id);
        }
    }
}

impl EntityBehaviourProvider for SystemCommandEntityBehaviourProviderImpl {
    fn add_behaviours(&self, entity_instance: Arc<ReactiveEntityInstance>) {
        match entity_instance.clone().type_name.as_str() {
            SYSTEM_COMMAND => self.create_system_command(entity_instance),
            _ => {}
        }
    }

    fn remove_behaviours(&self, entity_instance: Arc<ReactiveEntityInstance>) {
        match entity_instance.clone().type_name.as_str() {
            SYSTEM_COMMAND => self.remove_system_command(entity_instance),
            _ => {}
        }
    }

    fn remove_behaviours_by_id(&self, id: Uuid) {
        self.remove_by_id(id);
    }
}
