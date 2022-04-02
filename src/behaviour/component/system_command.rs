use std::process::Command;
use std::process::Stdio;
use std::sync::Arc;

use log::trace;
use serde_json::json;
use serde_json::Value;

use crate::behaviour::component::SystemCommandProperties;
use crate::model::PropertyInstanceSetter;
use crate::model::ReactiveEntityInstance;
use crate::reactive::entity::Disconnectable;
use crate::reactive::BehaviourCreationError;

pub const SYSTEM_COMMAND: &str = "system_command";

pub struct SystemCommand {
    pub entity: Arc<ReactiveEntityInstance>,

    pub handle_id: u128,
}

impl SystemCommand {
    pub fn new<'a>(e: Arc<ReactiveEntityInstance>) -> Result<SystemCommand, BehaviourCreationError> {
        let command = e
            .properties
            .get(SystemCommandProperties::COMMAND.as_ref())
            .ok_or(BehaviourCreationError)?
            .as_string()
            .ok_or(BehaviourCreationError)?;
        let trigger = e.properties.get(SystemCommandProperties::TRIGGER.as_ref()).ok_or(BehaviourCreationError)?;
        let entity = e.clone();
        let handle_id = trigger.id.as_u128();
        trigger.stream.read().unwrap().observe_with_handle(
            move |trigger: &Value| {
                if !trigger.as_bool().unwrap_or(false) {
                    // Assert: only execute on trigger=true
                    return;
                }
                if let Some(command_args) = entity
                    .properties
                    .get(SystemCommandProperties::PARAMETERS.as_ref())
                    .and_then(|r| r.as_array())
                    .and_then(|args| Some(args.into_iter().map(|v| v.as_str().map(|v| v.to_string())).filter_map(|arg| arg)))
                {
                    let mut command = Command::new(command.clone());
                    command.stdout(Stdio::piped());
                    command.args(command_args);
                    if let Ok(output) = command.output() {
                        let stdout = String::from_utf8(output.stdout);
                        if stdout.is_ok() {
                            entity.set(SystemCommandProperties::STDOUT, json!(stdout.unwrap()));
                        }
                        let stderr = String::from_utf8(output.stderr);
                        if stderr.is_ok() {
                            entity.set(SystemCommandProperties::STDERR, json!(stderr.unwrap()));
                        }
                    }
                }
            },
            handle_id,
        );

        Ok(SystemCommand { entity: e.clone(), handle_id })
    }
}

impl Disconnectable for SystemCommand {
    fn disconnect(&self) {
        trace!("Disconnecting {} with id {}", SYSTEM_COMMAND, self.entity.id);
        if let Some(property) = self.entity.properties.get(SystemCommandProperties::TRIGGER.as_ref()) {
            property.stream.read().unwrap().remove(self.handle_id);
        }
    }
}

impl Drop for SystemCommand {
    fn drop(&mut self) {
        self.disconnect();
    }
}
