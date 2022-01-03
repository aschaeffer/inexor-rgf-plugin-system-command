use indradb::NamedProperty;
use inexor_rgf_core_reactive::NamedProperties;
use serde_json::{json, Value};
use strum_macros::{AsRefStr, Display, IntoStaticStr};

#[allow(non_camel_case_types)]
#[derive(AsRefStr, IntoStaticStr, Display)]
pub enum SystemCommandProperties {
    #[strum(serialize = "name")]
    NAME,
    #[strum(serialize = "current_dir")]
    CURRENT_DIR,
    #[strum(serialize = "command")]
    COMMAND,
    #[strum(serialize = "spawn")]
    SPAWN,
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
            SystemCommandProperties::CURRENT_DIR => json!(""),
            SystemCommandProperties::COMMAND => json!(""),
            SystemCommandProperties::SPAWN => json!([]),
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
            NamedProperty::from(SystemCommandProperties::SPAWN),
            NamedProperty::from(SystemCommandProperties::STDIN),
            NamedProperty::from(SystemCommandProperties::STDOUT),
            NamedProperty::from(SystemCommandProperties::STDERR),
        ]
    }
}

impl From<SystemCommandProperties> for NamedProperty {
    fn from(p: SystemCommandProperties) -> Self {
        NamedProperty {
            name: p.to_string(),
            value: p.default_value(),
        }
    }
}

impl From<SystemCommandProperties> for String {
    fn from(p: SystemCommandProperties) -> Self {
        p.to_string()
    }
}
