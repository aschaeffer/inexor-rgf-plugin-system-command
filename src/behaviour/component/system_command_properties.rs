use indradb::Identifier;
use indradb::NamedProperty;
use serde_json::json;
use serde_json::Value;
use strum_macros::{AsRefStr, Display, IntoStaticStr};

use crate::reactive::NamedProperties;

#[allow(non_camel_case_types)]
#[derive(AsRefStr, IntoStaticStr, Display)]
pub enum SystemCommandProperties {
    #[strum(serialize = "name")]
    NAME,
    #[strum(serialize = "current_dir")]
    CURRENT_DIR,
    #[strum(serialize = "command")]
    COMMAND,
    #[strum(serialize = "parameters")]
    PARAMETERS,
    #[strum(serialize = "trigger")]
    TRIGGER,
    #[strum(serialize = "stdin")]
    STDIN,
    #[strum(serialize = "stdout")]
    STDOUT,
    #[strum(serialize = "stderr")]
    STDERR,
}

impl SystemCommandProperties {
    pub fn default_value(&self) -> Value {
        match self {
            SystemCommandProperties::NAME => json!(""),
            SystemCommandProperties::CURRENT_DIR => json!("."),
            SystemCommandProperties::COMMAND => json!(""),
            SystemCommandProperties::PARAMETERS => json!([]),
            SystemCommandProperties::TRIGGER => json!(false),
            SystemCommandProperties::STDIN => json!(""),
            SystemCommandProperties::STDOUT => json!(""),
            SystemCommandProperties::STDERR => json!(""),
        }
    }
    pub fn properties() -> NamedProperties {
        vec![
            NamedProperty::from(SystemCommandProperties::NAME),
            NamedProperty::from(SystemCommandProperties::CURRENT_DIR),
            NamedProperty::from(SystemCommandProperties::COMMAND),
            NamedProperty::from(SystemCommandProperties::PARAMETERS),
            NamedProperty::from(SystemCommandProperties::TRIGGER),
            NamedProperty::from(SystemCommandProperties::STDIN),
            NamedProperty::from(SystemCommandProperties::STDOUT),
            NamedProperty::from(SystemCommandProperties::STDERR),
        ]
    }
}

impl From<SystemCommandProperties> for NamedProperty {
    fn from(p: SystemCommandProperties) -> Self {
        NamedProperty {
            name: Identifier::new(p.to_string()).unwrap(),
            value: p.default_value(),
        }
    }
}

impl From<SystemCommandProperties> for String {
    fn from(p: SystemCommandProperties) -> Self {
        p.to_string()
    }
}
