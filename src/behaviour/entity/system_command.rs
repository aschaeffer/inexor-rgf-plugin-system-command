use std::convert::AsRef;
use std::process::{Command, Stdio};
use std::sync::Arc;

use inexor_rgf_core_model::PropertyInstanceSetter;
use log::error;
use serde_json::{json, Value};

use crate::behaviour::entity::SystemCommandProperties;
use crate::model::ReactiveEntityInstance;
use crate::reactive::entity::Disconnectable;
use crate::reactive::BehaviourCreationError;

pub const SYSTEM_COMMAND: &'static str = "system_command";

pub struct SystemCommand {
    pub entity: Arc<ReactiveEntityInstance>,

    pub handle_id: u128,
}

impl SystemCommand {
    pub fn new<'a>(e: Arc<ReactiveEntityInstance>) -> Result<SystemCommand, BehaviourCreationError> {
        let command = e.properties.get(SystemCommandProperties::COMMAND.as_ref());
        if command.is_none() {
            error!("Missing property: command");
            return Err(BehaviourCreationError.into());
        }
        let command = command.unwrap().as_string().unwrap();

        let spawn = e.properties.get(SystemCommandProperties::SPAWN.as_ref());
        if spawn.is_none() {
            error!("Missing property: spawn");
            return Err(BehaviourCreationError.into());
        }

        let entity = e.clone();
        let handle_id = e.properties.get(SystemCommandProperties::SPAWN.as_ref()).unwrap().id.as_u128();
        e.properties
            .get(SystemCommandProperties::SPAWN.as_ref())
            .unwrap()
            .stream
            .read()
            .unwrap()
            .observe_with_handle(
                move |command_arguments: &Value| {
                    if !command_arguments.is_array() {
                        // Invalid: expected array of arguments (or at least an empty array)
                        return;
                    }
                    let mut command = Command::new(command.clone());
                    command.stdout(Stdio::piped());
                    let args = command_arguments.as_array().unwrap();
                    for arg in args {
                        if arg.is_string() {
                            let arg = arg.as_str().unwrap();
                            command.arg(arg);
                        }
                    }
                    let output = command.output();
                    if output.is_ok() {
                        let output = output.unwrap();
                        let stdout = String::from_utf8(output.stdout);
                        if stdout.is_ok() {
                            entity.set(SystemCommandProperties::STDOUT, json!(stdout.unwrap()));
                        }
                        let stderr = String::from_utf8(output.stderr);
                        if stderr.is_ok() {
                            entity.set(SystemCommandProperties::STDERR, json!(stderr.unwrap()));
                        }
                    }
                },
                handle_id,
            );

        Ok(SystemCommand { entity: e.clone(), handle_id })
    }

    pub fn type_name(&self) -> String {
        self.entity.type_name.clone()
    }
}

impl Disconnectable for SystemCommand {
    fn disconnect(&self) {
        let property = self.entity.properties.get(SystemCommandProperties::SPAWN.as_ref());
        if property.is_some() {
            property.unwrap().stream.read().unwrap().remove(self.handle_id);
        }
    }
}

/// Automatically disconnect streams on destruction
impl Drop for SystemCommand {
    fn drop(&mut self) {
        self.disconnect();
    }
}
