//! Agent configuration value object
//!
//! Runtime configuration and settings for agents.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Agent configuration
///
/// Runtime settings that control agent behavior.
/// Configuration is mutable (unlike metadata) and can be updated during agent lifecycle.
///
/// # Examples
///
/// ```
/// use cim_domain_agent::value_objects_new::AgentConfiguration;
///
/// let config = AgentConfiguration::new()
///     .with_setting("max_retries", 3)
///     .with_setting("timeout_seconds", 30);
/// ```
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AgentConfiguration {
    /// Configuration settings (key-value pairs)
    settings: HashMap<String, serde_json::Value>,

    /// Configuration version/revision
    version: u64,

    /// Last updated timestamp
    updated_at: chrono::DateTime<chrono::Utc>,
}

impl AgentConfiguration {
    /// Create a new empty configuration
    ///
    /// # Examples
    ///
    /// ```
    /// use cim_domain_agent::value_objects_new::AgentConfiguration;
    ///
    /// let config = AgentConfiguration::new();
    /// assert_eq!(config.version(), 0);
    /// ```
    pub fn new() -> Self {
        Self {
            settings: HashMap::new(),
            version: 0,
            updated_at: chrono::Utc::now(),
        }
    }

    /// Set a configuration value
    ///
    /// # Examples
    ///
    /// ```
    /// use cim_domain_agent::value_objects_new::AgentConfiguration;
    ///
    /// let mut config = AgentConfiguration::new();
    /// config.set("max_retries", 5);
    /// assert_eq!(config.get("max_retries"), Some(&serde_json::json!(5)));
    /// ```
    pub fn set<T: Serialize>(&mut self, key: impl Into<String>, value: T) {
        self.settings.insert(
            key.into(),
            serde_json::to_value(value).unwrap()
        );
        self.version += 1;
        self.updated_at = chrono::Utc::now();
    }

    /// Builder-style setting method
    ///
    /// # Examples
    ///
    /// ```
    /// use cim_domain_agent::value_objects_new::AgentConfiguration;
    ///
    /// let config = AgentConfiguration::new()
    ///     .with_setting("timeout", 30)
    ///     .with_setting("retries", 3);
    /// ```
    pub fn with_setting<T: Serialize>(mut self, key: impl Into<String>, value: T) -> Self {
        self.set(key, value);
        self
    }

    /// Get a configuration value
    ///
    /// # Examples
    ///
    /// ```
    /// use cim_domain_agent::value_objects_new::AgentConfiguration;
    ///
    /// let config = AgentConfiguration::new()
    ///     .with_setting("timeout", 30);
    ///
    /// assert_eq!(config.get("timeout"), Some(&serde_json::json!(30)));
    /// assert_eq!(config.get("missing"), None);
    /// ```
    pub fn get(&self, key: &str) -> Option<&serde_json::Value> {
        self.settings.get(key)
    }

    /// Get a typed configuration value
    ///
    /// # Examples
    ///
    /// ```
    /// use cim_domain_agent::value_objects_new::AgentConfiguration;
    ///
    /// let config = AgentConfiguration::new()
    ///     .with_setting("timeout", 30);
    ///
    /// let timeout: Option<i64> = config.get_typed("timeout").ok().flatten();
    /// assert_eq!(timeout, Some(30));
    /// ```
    pub fn get_typed<T: for<'de> Deserialize<'de>>(&self, key: &str) -> Result<Option<T>, serde_json::Error> {
        match self.settings.get(key) {
            Some(value) => serde_json::from_value(value.clone()).map(Some),
            None => Ok(None),
        }
    }

    /// Remove a configuration value
    pub fn remove(&mut self, key: &str) -> Option<serde_json::Value> {
        let result = self.settings.remove(key);
        if result.is_some() {
            self.version += 1;
            self.updated_at = chrono::Utc::now();
        }
        result
    }

    /// Check if a key exists
    pub fn contains(&self, key: &str) -> bool {
        self.settings.contains_key(key)
    }

    /// Get all settings
    pub fn settings(&self) -> &HashMap<String, serde_json::Value> {
        &self.settings
    }

    /// Get configuration version
    pub fn version(&self) -> u64 {
        self.version
    }

    /// Get last updated timestamp
    pub fn updated_at(&self) -> chrono::DateTime<chrono::Utc> {
        self.updated_at
    }

    /// Merge another configuration into this one
    ///
    /// Overwrites existing keys with values from other config.
    ///
    /// # Examples
    ///
    /// ```
    /// use cim_domain_agent::value_objects_new::AgentConfiguration;
    ///
    /// let mut config1 = AgentConfiguration::new()
    ///     .with_setting("a", 1)
    ///     .with_setting("b", 2);
    ///
    /// let config2 = AgentConfiguration::new()
    ///     .with_setting("b", 3)
    ///     .with_setting("c", 4);
    ///
    /// config1.merge(config2);
    ///
    /// assert_eq!(config1.get_typed::<i64>("a").ok().flatten(), Some(1));
    /// assert_eq!(config1.get_typed::<i64>("b").ok().flatten(), Some(3)); // overwritten
    /// assert_eq!(config1.get_typed::<i64>("c").ok().flatten(), Some(4));
    /// ```
    pub fn merge(&mut self, other: Self) {
        for (key, value) in other.settings {
            self.settings.insert(key, value);
        }
        self.version += 1;
        self.updated_at = chrono::Utc::now();
    }

    // ========================================
    // Common Configuration Presets
    // ========================================

    /// Default configuration for system agents
    pub fn system_defaults() -> Self {
        Self::new()
            .with_setting("max_retries", 3)
            .with_setting("timeout_seconds", 30)
            .with_setting("log_level", "info")
            .with_setting("health_check_interval", 60)
    }

    /// Default configuration for AI agents
    pub fn ai_defaults() -> Self {
        Self::new()
            .with_setting("max_retries", 5)
            .with_setting("timeout_seconds", 120)
            .with_setting("log_level", "debug")
            .with_setting("temperature", 0.7)
            .with_setting("max_tokens", 2048)
    }

    /// Default configuration for integration agents
    pub fn integration_defaults() -> Self {
        Self::new()
            .with_setting("max_retries", 10)
            .with_setting("timeout_seconds", 60)
            .with_setting("log_level", "info")
            .with_setting("backoff_multiplier", 2.0)
            .with_setting("max_backoff_seconds", 300)
    }
}

impl Default for AgentConfiguration {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_configuration() {
        let config = AgentConfiguration::new();
        assert_eq!(config.version(), 0);
        assert!(config.settings().is_empty());
    }

    #[test]
    fn test_set_and_get() {
        let mut config = AgentConfiguration::new();
        config.set("timeout", 30);

        assert_eq!(config.get("timeout"), Some(&serde_json::json!(30)));
        assert_eq!(config.version(), 1);
    }

    #[test]
    fn test_with_setting() {
        let config = AgentConfiguration::new()
            .with_setting("a", 1)
            .with_setting("b", "test")
            .with_setting("c", true);

        assert_eq!(config.get("a"), Some(&serde_json::json!(1)));
        assert_eq!(config.get("b"), Some(&serde_json::json!("test")));
        assert_eq!(config.get("c"), Some(&serde_json::json!(true)));
    }

    #[test]
    fn test_get_typed() {
        let config = AgentConfiguration::new()
            .with_setting("number", 42)
            .with_setting("text", "hello")
            .with_setting("flag", true);

        let number: Option<i64> = config.get_typed("number").ok().flatten();
        assert_eq!(number, Some(42));

        let text: Option<String> = config.get_typed("text").ok().flatten();
        assert_eq!(text, Some("hello".to_string()));

        let flag: Option<bool> = config.get_typed("flag").ok().flatten();
        assert_eq!(flag, Some(true));
    }

    #[test]
    fn test_remove() {
        let mut config = AgentConfiguration::new()
            .with_setting("key", "value");

        assert!(config.contains("key"));

        let removed = config.remove("key");
        assert_eq!(removed, Some(serde_json::json!("value")));
        assert!(!config.contains("key"));
    }

    #[test]
    fn test_contains() {
        let config = AgentConfiguration::new()
            .with_setting("exists", 1);

        assert!(config.contains("exists"));
        assert!(!config.contains("missing"));
    }

    #[test]
    fn test_merge() {
        let mut config1 = AgentConfiguration::new()
            .with_setting("a", 1)
            .with_setting("b", 2);

        let config2 = AgentConfiguration::new()
            .with_setting("b", 3)
            .with_setting("c", 4);

        let old_version = config1.version();
        config1.merge(config2);

        assert_eq!(config1.get_typed::<i64>("a").ok().flatten(), Some(1));
        assert_eq!(config1.get_typed::<i64>("b").ok().flatten(), Some(3));
        assert_eq!(config1.get_typed::<i64>("c").ok().flatten(), Some(4));
        assert!(config1.version() > old_version);
    }

    #[test]
    fn test_version_increment() {
        let mut config = AgentConfiguration::new();
        assert_eq!(config.version(), 0);

        config.set("key1", 1);
        assert_eq!(config.version(), 1);

        config.set("key2", 2);
        assert_eq!(config.version(), 2);

        config.remove("key1");
        assert_eq!(config.version(), 3);
    }

    #[test]
    fn test_system_defaults() {
        let config = AgentConfiguration::system_defaults();
        assert_eq!(config.get_typed::<i64>("max_retries").ok().flatten(), Some(3));
        assert_eq!(config.get_typed::<i64>("timeout_seconds").ok().flatten(), Some(30));
    }

    #[test]
    fn test_ai_defaults() {
        let config = AgentConfiguration::ai_defaults();
        assert_eq!(config.get_typed::<i64>("max_tokens").ok().flatten(), Some(2048));
        assert_eq!(config.get_typed::<f64>("temperature").ok().flatten(), Some(0.7));
    }

    #[test]
    fn test_integration_defaults() {
        let config = AgentConfiguration::integration_defaults();
        assert_eq!(config.get_typed::<i64>("max_retries").ok().flatten(), Some(10));
        assert_eq!(config.get_typed::<f64>("backoff_multiplier").ok().flatten(), Some(2.0));
    }

    #[test]
    fn test_serialization() {
        let config = AgentConfiguration::new()
            .with_setting("test", 123);

        let json = serde_json::to_string(&config).unwrap();
        let deserialized: AgentConfiguration = serde_json::from_str(&json).unwrap();

        assert_eq!(config, deserialized);
    }
}
