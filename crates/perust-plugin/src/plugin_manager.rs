//! Plugin manager — lifecycle management for plugins.
//!
//! The [`PluginManager`] is responsible for discovering, loading, enabling,
//! and disabling plugins. It handles dependency resolution (including
//! topological sorting) and ensures plugins are loaded in the correct order.

use std::collections::{HashMap, HashSet};
use std::path::Path;

use crate::error::PluginError;
use crate::plugin::{Plugin, PluginContext};
use crate::plugin_description::PluginDescription;

/// Central lifecycle manager for plugins.
pub struct PluginManager {
    /// Loaded plugin instances keyed by name.
    plugins: HashMap<String, Box<dyn Plugin>>,
    /// Plugin descriptions keyed by name.
    descriptions: HashMap<String, PluginDescription>,
    /// Names of currently enabled plugins.
    enabled: HashSet<String>,
    /// Load order (determined by dependency resolution).
    load_order: Vec<String>,
}

impl PluginManager {
    /// Creates a new, empty `PluginManager`.
    pub fn new() -> Self {
        Self {
            plugins: HashMap::new(),
            descriptions: HashMap::new(),
            enabled: HashSet::new(),
            load_order: Vec::new(),
        }
    }

    /// Loads all plugins found in the given directory.
    ///
    /// The directory is scanned for sub-directories, each of which should
    /// contain a `plugin.json` descriptor file. After parsing all descriptors,
    /// the method resolves dependencies and determines the correct load order
    /// via topological sort.
    ///
    /// Returns a list of plugin names that were successfully loaded.
    pub fn load_plugins(&mut self, directory: &Path) -> Result<Vec<String>, PluginError> {
        if !directory.exists() {
            log::warn!("Plugin directory does not exist: {:?}", directory);
            return Ok(Vec::new());
        }

        let entries = std::fs::read_dir(directory)
            .map_err(|e| PluginError::LoadError {
                name: "directory".to_string(),
                reason: e.to_string(),
            })?;

        // Phase 1: Parse all descriptions.
        let mut found: Vec<(String, PluginDescription)> = Vec::new();

        for entry in entries.flatten() {
            let path = entry.path();
            if !path.is_dir() {
                continue;
            }

            // Try plugin.json first, then plugin.yml.
            let json_path = path.join("plugin.json");
            let yaml_path = path.join("plugin.yml");

            let desc_result = if json_path.exists() {
                let content = std::fs::read_to_string(&json_path).map_err(|e| PluginError::LoadError {
                    name: path.display().to_string(),
                    reason: e.to_string(),
                })?;
                PluginDescription::from_json(&content)
            } else if yaml_path.exists() {
                let content = std::fs::read_to_string(&yaml_path).map_err(|e| PluginError::LoadError {
                    name: path.display().to_string(),
                    reason: e.to_string(),
                })?;
                PluginDescription::from_yaml(&content)
            } else {
                log::warn!("No plugin.json or plugin.yml found in {:?}", path);
                continue;
            };

            match desc_result {
                Ok(desc) => {
                    log::info!("Found plugin: {} v{}", desc.name, desc.version);
                    found.push((desc.name.clone(), desc));
                }
                Err(e) => {
                    log::error!("Failed to parse plugin description in {:?}: {}", path, e);
                }
            }
        }

        // Phase 2: Store descriptions.
        for (name, desc) in &found {
            if self.descriptions.contains_key(name) {
                return Err(PluginError::AlreadyLoaded(name.clone()));
            }
            self.descriptions.insert(name.clone(), desc.clone());
        }

        // Phase 3: Topological sort by dependencies.
        let names: Vec<String> = found.iter().map(|(n, _)| n.clone()).collect();
        let sorted = self.topological_sort(&names)?;

        // Phase 4: Mark as loaded (actual plugin instances would be created
        // by the server's dynamic loading mechanism; here we just record
        // the load order and descriptions).
        self.load_order = sorted.clone();
        for name in &sorted {
            log::info!("Loaded plugin: {}", name);
        }

        Ok(sorted)
    }

    /// Enables a single plugin by name.
    ///
    /// Returns an error if the plugin is not loaded or is already enabled.
    pub fn enable_plugin(&mut self, name: &str) -> Result<(), PluginError> {
        if self.enabled.contains(name) {
            return Ok(()); // Already enabled — idempotent.
        }

        if !self.descriptions.contains_key(name) {
            return Err(PluginError::NotFound(name.to_string()));
        }

        if let Some(plugin) = self.plugins.get_mut(name) {
            let _desc = self.descriptions.get(name).unwrap();
            let data_folder = Path::new("plugins").join(name);
            let mut context = PluginContext::new(name, data_folder.clone());

            // Load config if available.
            let config_path = data_folder.join("config.json");
            if config_path.exists() {
                if let Ok(content) = std::fs::read_to_string(&config_path) {
                    if let Ok(config) = serde_json::from_str(&content) {
                        context.config = Some(config);
                    }
                }
            }

            plugin.on_enable(&mut context);
        }

        self.enabled.insert(name.to_string());
        log::info!("Enabled plugin: {}", name);
        Ok(())
    }

    /// Disables a single plugin by name.
    ///
    /// Returns an error if the plugin is not loaded.
    pub fn disable_plugin(&mut self, name: &str) -> Result<(), PluginError> {
        if !self.descriptions.contains_key(name) {
            return Err(PluginError::NotFound(name.to_string()));
        }

        if !self.enabled.contains(name) {
            return Ok(()); // Already disabled — idempotent.
        }

        if let Some(plugin) = self.plugins.get_mut(name) {
            let data_folder = Path::new("plugins").join(name);
            let mut context = PluginContext::new(name, data_folder);
            plugin.on_disable(&mut context);
        }

        self.enabled.remove(name);
        log::info!("Disabled plugin: {}", name);
        Ok(())
    }

    /// Enables all loaded plugins in dependency order.
    pub fn enable_all(&mut self) {
        let order = self.load_order.clone();
        for name in &order {
            if let Err(e) = self.enable_plugin(name) {
                log::error!("Failed to enable plugin '{}': {}", name, e);
            }
        }
    }

    /// Disables all enabled plugins in reverse dependency order.
    pub fn disable_all(&mut self) {
        let order: Vec<String> = self.load_order.iter().rev().cloned().collect();
        for name in &order {
            if let Err(e) = self.disable_plugin(name) {
                log::error!("Failed to disable plugin '{}': {}", name, e);
            }
        }
    }

    /// Returns a reference to a loaded plugin, or `None`.
    pub fn get_plugin(&self, name: &str) -> Option<&dyn Plugin> {
        self.plugins.get(name).map(|p| p.as_ref())
    }

    /// Returns `true` if the named plugin is currently enabled.
    pub fn is_enabled(&self, name: &str) -> bool {
        self.enabled.contains(name)
    }

    /// Returns the number of loaded plugins.
    pub fn plugin_count(&self) -> usize {
        self.plugins.len()
    }

    /// Returns the names of all loaded plugins in load order.
    pub fn get_plugin_names(&self) -> Vec<&str> {
        self.load_order.iter().map(|s| s.as_str()).collect()
    }

    /// Returns a reference to a plugin's description, if available.
    pub fn get_description(&self, name: &str) -> Option<&PluginDescription> {
        self.descriptions.get(name)
    }

    /// Inserts a pre-constructed plugin instance.
    ///
    /// This is useful for testing or for built-in plugins that don't need
    /// dynamic loading.
    pub fn insert_plugin(&mut self, name: &str, plugin: Box<dyn Plugin>, description: PluginDescription) {
        self.descriptions.insert(name.to_string(), description);
        self.load_order.push(name.to_string());
        self.plugins.insert(name.to_string(), plugin);
    }

    // -----------------------------------------------------------------------
    // Private helpers
    // -----------------------------------------------------------------------

    /// Topological sort of plugin names based on their `depend` lists.
    ///
    /// Uses a standard DFS-based topological sort with cycle detection.
    fn topological_sort(&self, names: &[String]) -> Result<Vec<String>, PluginError> {
        let mut visited: HashSet<String> = HashSet::new();
        let mut visiting: HashSet<String> = HashSet::new();
        let mut result: Vec<String> = Vec::new();

        for name in names {
            if !visited.contains(name) {
                self.visit(name, &mut visited, &mut visiting, &mut result)?;
            }
        }

        // Reverse to get the correct load order (dependencies first).
        result.reverse();
        Ok(result)
    }

    fn visit(
        &self,
        name: &str,
        visited: &mut HashSet<String>,
        visiting: &mut HashSet<String>,
        result: &mut Vec<String>,
    ) -> Result<(), PluginError> {
        if visited.contains(name) {
            return Ok(());
        }

        if visiting.contains(name) {
            return Err(PluginError::CircularDependency(name.to_string()));
        }

        visiting.insert(name.to_string());

        if let Some(desc) = self.descriptions.get(name) {
            for dep in &desc.depend {
                if !self.descriptions.contains_key(dep) {
                    return Err(PluginError::DependencyMissing {
                        plugin: name.to_string(),
                        dependency: dep.clone(),
                    });
                }
                self.visit(dep, visited, visiting, result)?;
            }
        }

        visiting.remove(name);
        visited.insert(name.to_string());
        result.push(name.to_string());
        Ok(())
    }
}

impl Default for PluginManager {
    fn default() -> Self {
        Self::new()
    }
}
