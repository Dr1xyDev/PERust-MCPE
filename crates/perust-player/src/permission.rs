//! Permission system for players.

use std::collections::HashMap;

/// A permission attachment that groups related permissions together.
///
/// Attachments can be added or removed from a player to dynamically
/// change their effective permissions. For example, a plugin might
/// add an attachment for the duration of a minigame.
#[derive(Debug, Clone)]
pub struct PermissionAttachment {
    /// Map of permission name to granted/denied value.
    /// `true` means the permission is granted, `false` means explicitly denied.
    pub permissions: HashMap<String, bool>,
}

impl PermissionAttachment {
    /// Creates a new empty permission attachment.
    pub fn new() -> Self {
        Self {
            permissions: HashMap::new(),
        }
    }

    /// Creates a permission attachment with the given permissions.
    pub fn with_permissions(permissions: HashMap<String, bool>) -> Self {
        Self { permissions }
    }

    /// Sets a permission in this attachment.
    pub fn set_permission(&mut self, permission: String, value: bool) {
        self.permissions.insert(permission, value);
    }

    /// Removes a permission from this attachment.
    pub fn remove_permission(&mut self, permission: &str) {
        self.permissions.remove(permission);
    }

    /// Checks if this attachment has a specific permission set.
    pub fn has_permission(&self, permission: &str) -> Option<bool> {
        self.permissions.get(permission).copied()
    }
}

impl Default for PermissionAttachment {
    fn default() -> Self {
        Self::new()
    }
}

/// Player permissions system with operator status and permission attachments.
///
/// The permission resolution order is:
/// 1. If the player is an operator (OP), they have all permissions unless
///    explicitly denied.
/// 2. Check direct permissions set on the player.
/// 3. Check permissions from attachments (in reverse order, most recent first).
/// 4. Default: permission denied.
#[derive(Debug, Clone)]
pub struct PlayerPermissions {
    /// Whether the player is a server operator.
    pub is_op: bool,
    /// Directly set permissions on this player.
    pub permissions: HashMap<String, bool>,
    /// Permission attachments (e.g., from plugins).
    pub attachments: Vec<PermissionAttachment>,
}

impl PlayerPermissions {
    /// Creates a new permission system.
    ///
    /// If `is_op` is true, the player has all permissions by default.
    pub fn new(is_op: bool) -> Self {
        Self {
            is_op,
            permissions: HashMap::new(),
            attachments: Vec::new(),
        }
    }

    /// Checks if the player has the specified permission.
    ///
    /// Resolution order:
    /// 1. Direct permissions on the player
    /// 2. Attachment permissions (most recent attachment first)
    /// 3. If OP, grant by default
    /// 4. Deny by default
    pub fn has_permission(&self, permission: &str) -> bool {
        // Check direct permissions first
        if let Some(&value) = self.permissions.get(permission) {
            return value;
        }

        // Check attachments (most recent first)
        for attachment in self.attachments.iter().rev() {
            if let Some(&value) = attachment.permissions.get(permission) {
                return value;
            }
        }

        // OP players have all permissions by default
        if self.is_op {
            return true;
        }

        // Default: denied
        false
    }

    /// Sets a permission directly on the player.
    pub fn set_permission(&mut self, permission: String, value: bool) {
        self.permissions.insert(permission, value);
    }

    /// Removes a directly set permission from the player.
    pub fn remove_permission(&mut self, permission: &str) {
        self.permissions.remove(permission);
    }

    /// Adds a permission attachment.
    pub fn add_attachment(&mut self, attachment: PermissionAttachment) {
        self.attachments.push(attachment);
    }

    /// Removes the most recently added attachment.
    pub fn remove_last_attachment(&mut self) -> Option<PermissionAttachment> {
        self.attachments.pop()
    }

    /// Removes all permission attachments.
    pub fn clear_attachments(&mut self) {
        self.attachments.clear();
    }

    /// Recalculates effective permissions.
    ///
    /// This is a no-op in the current implementation since permissions
    /// are resolved on-demand. It exists for compatibility with the
    /// Bukkit-style permission API and for future caching optimizations.
    pub fn recalculate(&mut self) {
        // In the current implementation, permissions are resolved on-demand
        // via has_permission(), so there's nothing to recalculate.
        // Future implementations could cache the resolved permissions here.
    }

    /// Returns all effective permissions for this player.
    ///
    /// This resolves all permissions from direct settings and attachments.
    pub fn get_effective_permissions(&self) -> HashMap<String, bool> {
        let mut effective = HashMap::new();

        // Start with attachment permissions (oldest first so newer ones override)
        for attachment in &self.attachments {
            for (perm, value) in &attachment.permissions {
                effective.insert(perm.clone(), *value);
            }
        }

        // Direct permissions override attachment permissions
        for (perm, value) in &self.permissions {
            effective.insert(perm.clone(), *value);
        }

        effective
    }
}

impl Default for PlayerPermissions {
    fn default() -> Self {
        Self::new(false)
    }
}
