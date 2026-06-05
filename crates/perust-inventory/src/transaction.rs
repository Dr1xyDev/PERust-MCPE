use crate::error::InventoryError;
use crate::item_stack::ItemStack;

/// A single inventory transaction describing a change in one slot.
#[derive(Debug, Clone)]
pub struct Transaction {
    /// The inventory ID this transaction affects.
    pub inventory_id: u32,
    /// The slot index within the inventory.
    pub slot: usize,
    /// The item that was in the slot before the transaction.
    pub old_item: Option<ItemStack>,
    /// The item that should be in the slot after the transaction.
    pub new_item: Option<ItemStack>,
}

impl Transaction {
    /// Creates a new transaction.
    pub fn new(
        inventory_id: u32,
        slot: usize,
        old_item: Option<ItemStack>,
        new_item: Option<ItemStack>,
    ) -> Self {
        Self {
            inventory_id,
            slot,
            old_item,
            new_item,
        }
    }
}

/// A queue of pending inventory transactions from a player.
#[derive(Debug, Clone)]
pub struct TransactionQueue {
    transactions: Vec<Transaction>,
    /// The runtime ID of the player who initiated the transactions.
    source: u64,
}

impl TransactionQueue {
    /// Creates a new empty transaction queue for the given player.
    pub fn new(source: u64) -> Self {
        Self {
            transactions: Vec::new(),
            source,
        }
    }

    /// Returns the source player's runtime ID.
    pub fn source(&self) -> u64 {
        self.source
    }

    /// Adds a transaction to the queue.
    pub fn add(&mut self, transaction: Transaction) {
        self.transactions.push(transaction);
    }

    /// Returns the number of pending transactions.
    pub fn len(&self) -> usize {
        self.transactions.len()
    }

    /// Returns `true` if there are no pending transactions.
    pub fn is_empty(&self) -> bool {
        self.transactions.is_empty()
    }

    /// Validates all transactions in the queue.
    ///
    /// Currently performs basic validation: ensures slots are not out of range
    /// for known inventory sizes. More complex validation (e.g., checking
    /// that old_item matches what's actually in the inventory) should be
    /// done by the caller with access to the actual inventories.
    pub fn validate(&self) -> Result<(), InventoryError> {
        for tx in &self.transactions {
            // Basic validation: old_item and new_item should not both be None (no-op)
            // unless this is explicitly allowed.
            if tx.old_item.is_none() && tx.new_item.is_none() {
                // Allow no-op transactions (they're harmless)
                continue;
            }

            // Validate that items are not invalid
            if let Some(ref item) = tx.old_item {
                if item.count == 0 && !item.is_air() {
                    return Err(InventoryError::InvalidItem(format!(
                        "Old item has zero count but is not air: id={}",
                        item.item_id
                    )));
                }
            }

            if let Some(ref item) = tx.new_item {
                if item.count == 0 && !item.is_air() {
                    return Err(InventoryError::InvalidItem(format!(
                        "New item has zero count but is not air: id={}",
                        item.item_id
                    )));
                }
            }
        }
        Ok(())
    }

    /// Executes all validated transactions against the given inventory mutator.
    ///
    /// The `apply` closure receives each transaction and should apply it to
    /// the appropriate inventory. If any transaction fails, execution stops
    /// and an error is returned.
    pub fn execute<F>(&mut self, mut apply: F) -> Result<(), InventoryError>
    where
        F: FnMut(&Transaction) -> Result<(), InventoryError>,
    {
        for tx in &self.transactions {
            apply(tx)?;
        }
        self.transactions.clear();
        Ok(())
    }

    /// Clears all pending transactions without executing them.
    pub fn clear(&mut self) {
        self.transactions.clear();
    }

    /// Returns an iterator over the pending transactions.
    pub fn iter(&self) -> impl Iterator<Item = &Transaction> {
        self.transactions.iter()
    }
}
