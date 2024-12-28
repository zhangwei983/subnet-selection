use candid::{CandidType, Principal};
use ic_cdk::api::call::call_with_payment128;
use ic_cdk::api::management_canister::main::CanisterSettings;
use serde::{Deserialize, Serialize};

const CYCLES_MINTING_CANISTER: &str = "rkp4c-7iaaa-aaaaa-aaaca-cai";

#[derive(Clone, Eq, PartialEq, Debug, CandidType, Deserialize)]
pub struct SubnetId {
    pub principal_id: String,
}

#[derive(Clone, Eq, PartialEq, Debug, CandidType, Deserialize)]
pub enum SubnetSelection {
    /// Choose a specific subnet
    Subnet { subnet: SubnetId },
}

#[derive(Clone, Eq, PartialEq, Debug, CandidType, Deserialize)]
struct CreateCanister {
    pub settings: Option<CanisterSettings>,
    pub subnet_selection: Option<SubnetSelection>,
    pub subnet_type: Option<String>,
}

/// Error for create_canister endpoint
#[derive(Clone, Eq, PartialEq, Debug, CandidType, Deserialize, Serialize)]
pub enum CreateCanisterError {
    Refunded {
        refund_amount: u128,
        create_error: String,
    },
}

#[ic_cdk::update]
async fn create_canister(subnet_id: Principal) -> Result<Principal, CreateCanisterError> {
    let subnet_selection = SubnetSelection::Subnet {
        subnet: SubnetId {
            principal_id: subnet_id.to_text(),
        },
    };

    let arg = CreateCanister {
        settings: None,
        subnet_type: None,
        subnet_selection: Some(subnet_selection),
    };

    let cycles: u128 = 3_000_000_000_000; // 3T cycles.

    let canister_id = Principal::from_text(CYCLES_MINTING_CANISTER).unwrap();

    call_with_payment128::<(CreateCanister,), (Result<Principal, CreateCanisterError>,)>(
        canister_id,
        "create_canister",
        (arg,),
        cycles,
    )
    .await
    .unwrap()
    .0
}

ic_cdk::export_candid!();
