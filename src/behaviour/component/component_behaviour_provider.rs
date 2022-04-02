use std::collections::HashMap;
use std::sync::Arc;
use std::sync::RwLock;

use async_trait::async_trait;
use log::debug;
use uuid::Uuid;

use crate::behaviour::component::system_command::SystemCommand;
use crate::behaviour::component::system_command::SYSTEM_COMMAND;
use crate::di::*;
use crate::model::ReactiveEntityInstance;
use crate::plugins::ComponentBehaviourProvider;

#[wrapper]
pub struct SystemCommandStorage(RwLock<HashMap<Uuid, Arc<SystemCommand>>>);

#[provides]
fn create_system_command_storage() -> SystemCommandStorage {
    SystemCommandStorage(RwLock::new(HashMap::new()))
}

#[async_trait]
pub trait SystemCommandComponentBehaviourProvider: ComponentBehaviourProvider + Send + Sync {
    fn create_system_command(&self, entity_instance: Arc<ReactiveEntityInstance>);

    fn remove_system_command(&self, entity_instance: Arc<ReactiveEntityInstance>);

    fn remove_by_id(&self, id: Uuid);
}

pub struct SystemCommandComponentBehaviourProviderImpl {
    system_command: SystemCommandStorage,
}

interfaces!(SystemCommandComponentBehaviourProviderImpl: dyn ComponentBehaviourProvider);

#[component]
impl SystemCommandComponentBehaviourProviderImpl {
    #[provides]
    fn new() -> Self {
        Self {
            system_command: create_system_command_storage(),
        }
    }
}

#[async_trait]
#[provides]
impl SystemCommandComponentBehaviourProvider for SystemCommandComponentBehaviourProviderImpl {
    fn create_system_command(&self, entity_instance: Arc<ReactiveEntityInstance>) {
        let id = entity_instance.id;
        debug!("x");
        match SystemCommand::new(entity_instance.clone()) {
            Ok(system_command) => {
                self.system_command.0.write().unwrap().insert(id, Arc::new(system_command));
                entity_instance.add_behaviour(SYSTEM_COMMAND);
                debug!("Added component behaviour {} to entity instance {}", SYSTEM_COMMAND, id);
            }
            Err(_) => {
                debug!("Failed to add component behaviour {} to entity instance {}", SYSTEM_COMMAND, id);
            }
        }
    }

    fn remove_system_command(&self, entity_instance: Arc<ReactiveEntityInstance>) {
        self.system_command.0.write().unwrap().remove(&entity_instance.id);
        entity_instance.remove_behaviour(SYSTEM_COMMAND);
        debug!("Removed component behaviour {} from entity instance {}", SYSTEM_COMMAND, entity_instance.id);
    }

    fn remove_by_id(&self, id: Uuid) {
        if self.system_command.0.write().unwrap().contains_key(&id) {
            self.system_command.0.write().unwrap().remove(&id);
            debug!("Removed component behaviour {} from entity instance {}", SYSTEM_COMMAND, id);
        }
    }
}

impl ComponentBehaviourProvider for SystemCommandComponentBehaviourProviderImpl {
    fn add_behaviours_to_entity(&self, entity_instance: Arc<ReactiveEntityInstance>) {
        if entity_instance.is_a(SYSTEM_COMMAND) {
            self.create_system_command(entity_instance);
        }
    }

    fn add_behaviours_to_entity_component(&self, entity_instance: Arc<ReactiveEntityInstance>, component: crate::model::Component) {
        if component.name == SYSTEM_COMMAND {
            self.create_system_command(entity_instance)
        }
    }

    // fn add_behaviours_to_relation(&self, relation_instance: Arc<ReactiveRelationInstance>) {}

    // fn add_behaviours_to_relation_component(
    //     &self,
    //     relation_instance: Arc<ReactiveRelationInstance>,
    //     component: crate::model::Component,
    // ) {
    // }

    fn remove_behaviours_from_entity(&self, entity_instance: Arc<ReactiveEntityInstance>) {
        if entity_instance.behaves_as(SYSTEM_COMMAND) {
            self.remove_system_command(entity_instance);
        }
    }

    fn remove_behaviours_from_entity_component(&self, entity_instance: Arc<ReactiveEntityInstance>, component: crate::model::Component) {
        if component.name == SYSTEM_COMMAND {
            self.remove_system_command(entity_instance);
        }
    }

    // fn remove_behaviours_from_relation(&self, relation_instance: Arc<ReactiveRelationInstance>) {}

    // fn remove_behaviours_from_relation_component(
    //     &self,
    //     relation_instance: Arc<ReactiveRelationInstance>,
    //     component: crate::model::Component,
    // ) {
    // }

    fn remove_behaviours_by_id(&self, id: Uuid) {
        self.remove_by_id(id);
    }

    // fn remove_behaviours_by_key(&self, edge_key: EdgeKey) {}
}
