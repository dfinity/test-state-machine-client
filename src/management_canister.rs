// `ic-cdk` use this crate as a dependency in its tests.
// To break the circular dependency, we define the management canister types directly
// instead of using them from `ic-cdk`.
// There is a plan to move management canister types out of `ic-cdk`.
// Once that happens, we can use the management canister types in a consistent way.

use candid::{CandidType, Nat, Principal};
use serde::{Deserialize, Serialize};

pub(crate) type CanisterId = Principal;

#[derive(
    CandidType, Serialize, Deserialize, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Default,
)]
pub struct CanisterSettings {
    /// A list of principals. Must be between 0 and 10 in size.
    pub controllers: Option<Vec<Principal>>,
    /// Must be a number between 0 and 100, inclusively.
    pub compute_allocation: Option<Nat>,
    /// Must be a number between 0 and 2^48^ (i.e 256TB), inclusively.
    pub memory_allocation: Option<Nat>,
    /// Must be a number between 0 and 2^64^-1, inclusively, and indicates a length of time in seconds.
    pub freezing_threshold: Option<Nat>,
}

/// Argument type of [create_canister](super::create_canister).
#[derive(
    CandidType, Serialize, Deserialize, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Default,
)]
pub(crate) struct CreateCanisterArgument {
    /// See [CanisterSettings].
    pub settings: Option<CanisterSettings>,
}

#[derive(
    CandidType, Serialize, Deserialize, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy,
)]
// #[serde(rename_all = "lowercase")]
pub(crate) enum CanisterInstallMode {
    /// A fresh install of a new canister.
    #[serde(rename = "install")]
    Install,
    /// Reinstalling a canister that was already installed.
    #[serde(rename = "reinstall")]
    Reinstall,
    /// Upgrade an existing canister.
    #[serde(rename = "upgrade")]
    Upgrade,
}

pub(crate) type WasmModule = Vec<u8>;

#[derive(
    CandidType, Serialize, Deserialize, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone,
)]
pub(crate) struct InstallCodeArgument {
    /// See [CanisterInstallMode].
    pub mode: CanisterInstallMode,
    /// Principle of the canister.
    pub canister_id: CanisterId,
    /// Code to be installed.
    pub wasm_module: WasmModule,
    /// The argument to be passed to `canister_init` or `canister_post_upgrade`.
    pub arg: Vec<u8>,
}

/// A wrapper of canister id.
#[derive(
    CandidType, Serialize, Deserialize, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy,
)]
pub(crate) struct CanisterIdRecord {
    /// Principle of the canister.
    pub canister_id: CanisterId,
}
