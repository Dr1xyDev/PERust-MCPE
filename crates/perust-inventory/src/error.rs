use thiserror::Error;

/// Errors that can occur during inventory operations.
#[derive(Debug, Error)]
pub enum InventoryError {
    /// The requested slot index is out of the inventory's valid range.
    #[error("Slot {0} is out of range (size: {1})")]
    SlotOutOfRange(usize, usize),

    /// The item operation is invalid (e.g., negative count, mismatched item).
    #[error("Invalid item: {0}")]
    InvalidItem(String),

    /// A transaction could not be completed (e.g., validation failed).
    #[error("Transaction failed: {0}")]
    TransactionFailed(String),
}
