//! Permission value objects
//!
//! Fine-grained access control for agent capabilities.

use serde::{Deserialize, Serialize};
use std::fmt;

/// Permission identifier
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct PermissionId(String);

impl PermissionId {
    /// Create a new permission ID
    ///
    /// Uses hierarchical naming: `resource.action` or `domain.resource.action`
    ///
    /// # Examples
    ///
    /// ```
    /// use cim_domain_agent::value_objects_new::PermissionId;
    ///
    /// let id = PermissionId::new("graph.read");
    /// assert_eq!(id.as_str(), "graph.read");
    /// ```
    pub fn new(id: impl Into<String>) -> Self {
        Self(id.into())
    }

    /// Get the permission ID as a string
    pub fn as_str(&self) -> &str {
        &self.0
    }

    /// Parse a permission from hierarchical notation
    ///
    /// # Examples
    ///
    /// ```
    /// use cim_domain_agent::value_objects_new::PermissionId;
    ///
    /// let id = PermissionId::from_notation("workflow", "execute");
    /// assert_eq!(id.as_str(), "workflow.execute");
    /// ```
    pub fn from_notation(resource: &str, action: &str) -> Self {
        Self::new(format!("{}.{}", resource, action))
    }
}

impl fmt::Display for PermissionId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<String> for PermissionId {
    fn from(s: String) -> Self {
        Self::new(s)
    }
}

impl From<&str> for PermissionId {
    fn from(s: &str) -> Self {
        Self::new(s)
    }
}

/// Permission - Granular access control
///
/// Permissions follow hierarchical naming:
/// - `resource.action` - Simple permission (e.g., `graph.read`)
/// - `domain.resource.action` - Namespaced (e.g., `workflow.graph.modify`)
///
/// # Examples
///
/// ```
/// use cim_domain_agent::value_objects_new::Permission;
///
/// let perm = Permission::new(
///     "graph.read",
///     "Read access to graph data"
/// );
/// assert!(perm.matches("graph.read"));
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Permission {
    /// Permission identifier
    id: PermissionId,

    /// Human-readable description
    description: String,

    /// Scope/context for this permission (optional)
    scope: Option<String>,
}

impl Permission {
    /// Create a new permission
    ///
    /// # Arguments
    ///
    /// * `id` - Permission identifier (hierarchical)
    /// * `description` - What this permission allows
    ///
    /// # Examples
    ///
    /// ```
    /// use cim_domain_agent::value_objects_new::Permission;
    ///
    /// let perm = Permission::new("workflow.execute", "Execute workflows");
    /// ```
    pub fn new(
        id: impl Into<PermissionId>,
        description: impl Into<String>,
    ) -> Self {
        Self {
            id: id.into(),
            description: description.into(),
            scope: None,
        }
    }

    /// Add a scope to the permission
    pub fn with_scope(mut self, scope: impl Into<String>) -> Self {
        self.scope = Some(scope.into());
        self
    }

    /// Get the permission ID
    pub fn id(&self) -> &PermissionId {
        &self.id
    }

    /// Get the description
    pub fn description(&self) -> &str {
        &self.description
    }

    /// Get the scope
    pub fn scope(&self) -> Option<&str> {
        self.scope.as_deref()
    }

    /// Check if this permission matches a required permission
    ///
    /// Supports wildcard matching:
    /// - `graph.*` matches `graph.read`, `graph.write`, etc.
    /// - `*` matches everything
    ///
    /// # Examples
    ///
    /// ```
    /// use cim_domain_agent::value_objects_new::Permission;
    ///
    /// let perm = Permission::new("graph.read", "Read graphs");
    /// assert!(perm.matches("graph.read"));
    /// assert!(!perm.matches("graph.write"));
    ///
    /// let wildcard = Permission::new("graph.*", "All graph permissions");
    /// assert!(wildcard.matches("graph.read"));
    /// assert!(wildcard.matches("graph.write"));
    /// ```
    pub fn matches(&self, required: &str) -> bool {
        let perm_str = self.id.as_str();

        // Exact match
        if perm_str == required {
            return true;
        }

        // Wildcard match
        if perm_str.ends_with(".*") {
            let prefix = &perm_str[..perm_str.len() - 2];
            return required.starts_with(prefix);
        }

        // Global wildcard
        if perm_str == "*" {
            return true;
        }

        false
    }

    // ========================================
    // Standard Permissions
    // ========================================

    /// Read access to graphs
    pub fn graph_read() -> Self {
        Self::new("graph.read", "Read access to graph data")
    }

    /// Write access to graphs
    pub fn graph_write() -> Self {
        Self::new("graph.write", "Write access to graph data")
    }

    /// Modify graph structure
    pub fn graph_modify() -> Self {
        Self::new("graph.modify", "Modify graph structure")
    }

    /// Execute workflows
    pub fn workflow_execute() -> Self {
        Self::new("workflow.execute", "Execute workflows")
    }

    /// Read workflow definitions
    pub fn workflow_read() -> Self {
        Self::new("workflow.read", "Read workflow definitions")
    }

    /// Modify workflow definitions
    pub fn workflow_modify() -> Self {
        Self::new("workflow.modify", "Modify workflow definitions")
    }

    /// Access to AI capabilities
    pub fn ai_capabilities() -> Self {
        Self::new("ai.capabilities", "Access to AI capabilities")
    }

    /// System administration
    pub fn system_admin() -> Self {
        Self::new("system.admin", "System administration access")
    }

    /// All permissions (wildcard)
    pub fn all() -> Self {
        Self::new("*", "Full access to all resources")
    }
}

impl fmt::Display for Permission {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}: {}", self.id, self.description)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_permission_id() {
        let id = PermissionId::new("graph.read");
        assert_eq!(id.as_str(), "graph.read");
    }

    #[test]
    fn test_permission_id_from_notation() {
        let id = PermissionId::from_notation("workflow", "execute");
        assert_eq!(id.as_str(), "workflow.execute");
    }

    #[test]
    fn test_permission() {
        let perm = Permission::new("graph.read", "Read graphs");
        assert_eq!(perm.id().as_str(), "graph.read");
        assert_eq!(perm.description(), "Read graphs");
        assert!(perm.scope().is_none());
    }

    #[test]
    fn test_permission_with_scope() {
        let perm = Permission::new("graph.read", "Read graphs")
            .with_scope("tenant:123");

        assert_eq!(perm.scope(), Some("tenant:123"));
    }

    #[test]
    fn test_permission_exact_match() {
        let perm = Permission::new("graph.read", "Read");
        assert!(perm.matches("graph.read"));
        assert!(!perm.matches("graph.write"));
    }

    #[test]
    fn test_permission_wildcard_match() {
        let perm = Permission::new("graph.*", "All graph permissions");
        assert!(perm.matches("graph.read"));
        assert!(perm.matches("graph.write"));
        assert!(perm.matches("graph.modify"));
        assert!(!perm.matches("workflow.execute"));
    }

    #[test]
    fn test_permission_global_wildcard() {
        let perm = Permission::new("*", "All permissions");
        assert!(perm.matches("graph.read"));
        assert!(perm.matches("workflow.execute"));
        assert!(perm.matches("anything.at.all"));
    }

    #[test]
    fn test_standard_permissions() {
        let graph_read = Permission::graph_read();
        assert_eq!(graph_read.id().as_str(), "graph.read");

        let workflow_exec = Permission::workflow_execute();
        assert_eq!(workflow_exec.id().as_str(), "workflow.execute");

        let all = Permission::all();
        assert_eq!(all.id().as_str(), "*");
    }

    #[test]
    fn test_permission_serialization() {
        let perm = Permission::new("test.permission", "Test");
        let json = serde_json::to_string(&perm).unwrap();
        let deserialized: Permission = serde_json::from_str(&json).unwrap();
        assert_eq!(perm, deserialized);
    }

    #[test]
    fn test_permission_display() {
        let perm = Permission::new("graph.read", "Read graphs");
        let display = format!("{}", perm);
        assert!(display.contains("graph.read"));
        assert!(display.contains("Read graphs"));
    }
}
