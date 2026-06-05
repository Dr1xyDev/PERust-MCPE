//! Plugin description — metadata loaded from descriptor files.
//!
//! A [`PluginDescription`] is typically loaded from a `plugin.json` or
//! `plugin.yml` file bundled with a plugin. It contains the information the
//! server needs to identify, load, and manage the plugin.

use crate::error::PluginError;

/// Metadata describing a plugin.
///
/// This struct is usually deserialized from a JSON or YAML file shipped with
/// the plugin jar/folder.
#[derive(Debug, Clone)]
pub struct PluginDescription {
    /// The plugin's unique name.
    pub name: String,
    /// The plugin's version string.
    pub version: String,
    /// The main entry-point identifier (e.g. path to the plugin struct).
    pub main: String,
    /// The API version this plugin targets.
    pub api_version: String,
    /// An optional human-readable description.
    pub description: Option<String>,
    /// The plugin author's name.
    pub author: Option<String>,
    /// The plugin's website URL.
    pub website: Option<String>,
    /// Commands declared by this plugin.
    pub commands: Vec<CommandEntry>,
    /// Permissions declared by this plugin.
    pub permissions: Vec<PermissionEntry>,
    /// Hard dependencies — the plugin will not load without these.
    pub depend: Vec<String>,
    /// Soft dependencies — the plugin will load without these but enables
    /// after them if they are present.
    pub soft_depend: Vec<String>,
    /// When the plugin should be loaded relative to world loading.
    pub load_order: PluginLoadOrder,
}

impl PluginDescription {
    /// Parses a [`PluginDescription`] from a JSON string.
    pub fn from_json(json: &str) -> Result<Self, PluginError> {
        let value: serde_json::Value =
            serde_json::from_str(json).map_err(|e| PluginError::InvalidDescription(e.to_string()))?;

        Self::from_json_value(&value)
    }

    /// Parses a [`PluginDescription`] from a YAML string.
    pub fn from_yaml(yaml: &str) -> Result<Self, PluginError> {
        let value: serde_json::Value =
            serde_yaml::from_str(yaml).map_err(|e| PluginError::InvalidDescription(e.to_string()))?;

        Self::from_json_value(&value)
    }

    fn from_json_value(value: &serde_json::Value) -> Result<Self, PluginError> {
        let obj = value
            .as_object()
            .ok_or_else(|| PluginError::InvalidDescription("expected a JSON object".to_string()))?;

        let name = obj
            .get("name")
            .and_then(|v| v.as_str())
            .ok_or_else(|| PluginError::InvalidDescription("missing 'name' field".to_string()))?
            .to_string();

        let version = obj
            .get("version")
            .and_then(|v| v.as_str())
            .ok_or_else(|| PluginError::InvalidDescription("missing 'version' field".to_string()))?
            .to_string();

        let main = obj
            .get("main")
            .and_then(|v| v.as_str())
            .ok_or_else(|| PluginError::InvalidDescription("missing 'main' field".to_string()))?
            .to_string();

        let api_version = obj
            .get("api_version")
            .or_else(|| obj.get("api-version"))
            .and_then(|v| v.as_str())
            .unwrap_or("0.1")
            .to_string();

        let description = obj.get("description").and_then(|v| v.as_str()).map(String::from);
        let author = obj.get("author").and_then(|v| v.as_str()).map(String::from);
        let website = obj.get("website").and_then(|v| v.as_str()).map(String::from);

        let commands = obj
            .get("commands")
            .and_then(|v| v.as_object())
            .map(|map| {
                map.iter()
                    .map(|(name, entry)| CommandEntry::from_json(name, entry))
                    .collect()
            })
            .unwrap_or_default();

        let permissions = obj
            .get("permissions")
            .and_then(|v| v.as_object())
            .map(|map| {
                map.iter()
                    .map(|(name, entry)| PermissionEntry::from_json(name, entry))
                    .collect()
            })
            .unwrap_or_default();

        let depend = obj
            .get("depend")
            .and_then(|v| v.as_array())
            .map(|arr| arr.iter().filter_map(|v| v.as_str().map(String::from)).collect())
            .unwrap_or_default();

        let soft_depend = obj
            .get("soft_depend")
            .or_else(|| obj.get("soft-depend"))
            .and_then(|v| v.as_array())
            .map(|arr| arr.iter().filter_map(|v| v.as_str().map(String::from)).collect())
            .unwrap_or_default();

        let load_order = obj
            .get("load_order")
            .or_else(|| obj.get("load-order"))
            .and_then(|v| v.as_str())
            .map(|s| match s {
                "startup" => PluginLoadOrder::Startup,
                "postworld" | "post_world" => PluginLoadOrder::PostWorld,
                _ => PluginLoadOrder::PostWorld,
            })
            .unwrap_or(PluginLoadOrder::PostWorld);

        Ok(Self {
            name,
            version,
            main,
            api_version,
            description,
            author,
            website,
            commands,
            permissions,
            depend,
            soft_depend,
            load_order,
        })
    }
}

/// When a plugin should be loaded relative to world loading.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PluginLoadOrder {
    /// Load the plugin at server startup, before worlds are loaded.
    Startup,
    /// Load the plugin after worlds have been loaded.
    PostWorld,
}

/// A command declared in a plugin description.
#[derive(Debug, Clone)]
pub struct CommandEntry {
    /// The command name.
    pub name: String,
    /// Optional command description.
    pub description: Option<String>,
    /// Optional usage string.
    pub usage: Option<String>,
    /// Optional permission required to run this command.
    pub permission: Option<String>,
    /// Alternative names for this command.
    pub aliases: Vec<String>,
}

impl CommandEntry {
    fn from_json(name: &str, value: &serde_json::Value) -> Self {
        let obj = value.as_object();
        Self {
            name: name.to_string(),
            description: obj.and_then(|o| o.get("description")).and_then(|v| v.as_str()).map(String::from),
            usage: obj.and_then(|o| o.get("usage")).and_then(|v| v.as_str()).map(String::from),
            permission: obj.and_then(|o| o.get("permission")).and_then(|v| v.as_str()).map(String::from),
            aliases: obj
                .and_then(|o| o.get("aliases"))
                .and_then(|v| v.as_array())
                .map(|arr| arr.iter().filter_map(|v| v.as_str().map(String::from)).collect())
                .unwrap_or_default(),
        }
    }
}

/// A permission declared in a plugin description.
#[derive(Debug, Clone)]
pub struct PermissionEntry {
    /// The permission node name.
    pub name: String,
    /// Optional description of what this permission controls.
    pub description: Option<String>,
    /// The default assignment behaviour.
    pub default: PermissionDefault,
}

impl PermissionEntry {
    fn from_json(name: &str, value: &serde_json::Value) -> Self {
        let obj = value.as_object();
        let default_str = obj
            .and_then(|o| o.get("default"))
            .and_then(|v| v.as_str())
            .unwrap_or("op");

        let default = match default_str {
            "true" => PermissionDefault::True,
            "false" => PermissionDefault::False,
            "op" => PermissionDefault::Op,
            "notop" | "not_op" => PermissionDefault::NotOp,
            _ => PermissionDefault::Op,
        };

        Self {
            name: name.to_string(),
            description: obj.and_then(|o| o.get("description")).and_then(|v| v.as_str()).map(String::from),
            default,
        }
    }
}

/// Default assignment behaviour for a permission.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PermissionDefault {
    /// Everyone gets this permission by default.
    True,
    /// No one gets this permission by default.
    False,
    /// Only operators get this permission by default.
    Op,
    /// Only non-operators get this permission by default.
    NotOp,
}
