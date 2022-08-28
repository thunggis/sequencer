use serde::{Deserialize, Serialize};

use super::{BlockNumber, ClassHash, ContractAddress, ContractClass, Nonce, StarkFelt};

/// The sequential numbering of the states between blocks in StarkNet.
// Example:
// States: S0       S1       S2
// Blocks      B0->     B1->
#[derive(
    Debug, Default, Copy, Clone, Eq, PartialEq, Hash, Deserialize, Serialize, PartialOrd, Ord,
)]
pub struct StateNumber(u64);
impl StateNumber {
    // The state at the beginning of the block.
    pub fn right_before_block(block_number: BlockNumber) -> StateNumber {
        StateNumber(block_number.0)
    }
    // The state at the end of the block.
    pub fn right_after_block(block_number: BlockNumber) -> StateNumber {
        StateNumber(block_number.next().0)
    }
    pub fn is_before(&self, block_number: BlockNumber) -> bool {
        self.0 <= block_number.0
    }
    pub fn is_after(&self, block_number: BlockNumber) -> bool {
        self.0 > block_number.0
    }
    pub fn block_after(&self) -> BlockNumber {
        BlockNumber(self.0)
    }
}

// Invariant: Addresses are strictly increasing.
/// The differences between two states in StarkNet.
#[derive(Debug, Default, Clone, Eq, PartialEq, Deserialize, Serialize)]
pub struct StateDiff {
    deployed_contracts: Vec<DeployedContract>,
    storage_diffs: Vec<StorageDiff>,
    declared_classes: Vec<(ClassHash, ContractClass)>,
    nonces: Vec<(ContractAddress, Nonce)>,
}

type StateDiffAsTuple = (
    Vec<DeployedContract>,
    Vec<StorageDiff>,
    Vec<(ClassHash, ContractClass)>,
    Vec<(ContractAddress, Nonce)>,
);

impl StateDiff {
    pub fn new(
        mut deployed_contracts: Vec<DeployedContract>,
        mut storage_diffs: Vec<StorageDiff>,
        mut declared_classes: Vec<(ClassHash, ContractClass)>,
        mut nonces: Vec<(ContractAddress, Nonce)>,
    ) -> Self {
        deployed_contracts.sort_by_key(|dc| dc.address);
        storage_diffs.sort_by_key(|sd| sd.address);
        declared_classes.sort_by_key(|dc| dc.0);
        nonces.sort_by_key(|n| n.0);
        Self { deployed_contracts, storage_diffs, declared_classes, nonces }
    }

    pub fn destruct(self) -> StateDiffAsTuple {
        (self.deployed_contracts, self.storage_diffs, self.declared_classes, self.nonces)
    }
}

/// A deployed contract in StarkNet.
#[derive(Debug, Default, Clone, Eq, PartialEq, Hash, Deserialize, Serialize, PartialOrd, Ord)]
pub struct DeployedContract {
    pub address: ContractAddress,
    pub class_hash: ClassHash,
}

/// A declared contract in StarkNet.
#[derive(Debug, Default, Clone, Eq, PartialEq, Deserialize, Serialize)]
pub struct DeclaredContract {
    pub class_hash: ClassHash,
    pub contract_class: ContractClass,
}

// Invariant: Addresses are strictly increasing. In particular, no address appears twice.
// TODO(spapini): Enforce the invariant.
/// Storage differences in StarkNet.
#[derive(Debug, Default, Clone, Eq, PartialEq, Hash, Deserialize, Serialize, PartialOrd, Ord)]
pub struct StorageDiff {
    pub address: ContractAddress,
    pub diff: Vec<StorageEntry>,
}

// TODO: Invariant: this is in range.
// TODO(spapini): Enforce the invariant.
/// A storage key in a StarkNet contract.
#[derive(Debug, Default, Clone, Eq, PartialEq, Hash, Deserialize, Serialize, PartialOrd, Ord)]
pub struct StorageKey(pub StarkFelt);

/// A storage entry in a StarkNet contract.
#[derive(Debug, Default, Clone, Eq, PartialEq, Hash, Deserialize, Serialize, PartialOrd, Ord)]
pub struct StorageEntry {
    pub key: StorageKey,
    pub value: StarkFelt,
}
