use candid::{CandidType, Principal};
use ic_cdk::api::management_canister::main::CanisterSettings;
use ic_cdk::api::{call::call_with_payment128, is_controller};
use ic_cdk::{caller, id};
use serde::{Deserialize, Serialize};

const CYCLES_MINTING_CANISTER: &str = "rkp4c-7iaaa-aaaaa-aaaca-cai";
const CYCLES: u128 = 1_000_000_000_000; // 1T cycles.

#[derive(Clone, Eq, PartialEq, Debug, CandidType, Deserialize)]
pub struct SubnetId {
    pub principal_id: String,
}

#[derive(Clone, Eq, PartialEq, Debug, CandidType, Deserialize)]
pub enum SubnetSelection {
    /// Choose a specific subnet
    Subnet { subnet: SubnetId },
    // Skip the SubnetFilter on the CMC SubnetSelection for simplification.
    // https://github.com/dfinity/ic/blob/master/rs/nns/cmc/cmc.did#L35
}

#[derive(Clone, Eq, PartialEq, Debug, CandidType, Deserialize)]
struct CreateCanister {
    pub settings: Option<CanisterSettings>,
    pub subnet_selection: Option<SubnetSelection>,
    pub subnet_type: Option<String>,
}

/// Error for create_canister.
#[derive(Clone, Eq, PartialEq, Debug, CandidType, Deserialize, Serialize)]
pub enum CreateCanisterError {
    Refunded {
        refund_amount: u128,
        create_error: String,
    },
}

#[ic_cdk::update]
async fn create_canister(subnet_id: Principal) -> Result<Principal, CreateCanisterError> {
    if subnet_id == Principal::anonymous() {
        panic!("Cannot use an anonymous subnet id.");
    }

    if !is_controller(&caller()) {
        panic!("Only the controllers can call 'create_canister' function.");
    }

    let arg = CreateCanister {
        settings: Some(CanisterSettings {
            controllers: Some(vec![caller(), id()]),
            compute_allocation: None,
            memory_allocation: None,
            freezing_threshold: None,
            reserved_cycles_limit: None,
            log_visibility: None,
            wasm_memory_limit: None,
        }),
        subnet_type: None,
        subnet_selection: Some(SubnetSelection::Subnet {
            subnet: SubnetId {
                principal_id: subnet_id.to_text(),
            },
        }),
    };

    call_with_payment128::<(CreateCanister,), (Result<Principal, CreateCanisterError>,)>(
        Principal::from_text(CYCLES_MINTING_CANISTER).unwrap(),
        "create_canister",
        (arg,),
        CYCLES,
    )
    .await
    .unwrap()
    .0
}

ic_cdk::export_candid!();
