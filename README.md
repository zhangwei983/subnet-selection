# Subnet Selection
Demonstrate how to create a canister on a given subnet.

## How to interact

The below steps are for deploying locally.

### Install the system canisters

In this sample, cycles minting canister(CMC) is going to be used to create canisters on a given subnet. Please refer [this document](https://github.com/zhangwei983/deploy_icp_ledger/blob/main/by_dfx_nns/README.md) to install the system canisters locally.

Here are the two canisters that we're going to use in this example.

```
nns-registry          rwlgt-iiaaa-aaaaa-aaaaa-cai
nns-cycles-minting    rkp4c-7iaaa-aaaaa-aaaca-cai
```

### Create canister

1. Deploy the subnet_backend canister
   ```bash
   dfx deploy
   ```
1. Get the subnet id
   ```bash
   dfx canister call rwlgt-iiaaa-aaaaa-aaaaa-cai get_subnet_for_canister '(record {"principal"= opt principal "rkp4c-7iaaa-aaaaa-aaaca-cai"})'
   ```
   We're going to use the `nns-registry` canister to get the subnet id from a canister id (e.g. the nns-cycles-minting canister).  
   Since we're testing in the local environment, we only have one subnet available.

1. create canister
   ```bash
   dfx canister call subnet_backend create_canister '(principal "4rnji-lwrly-r3v6p-y5vby-jrtkt-4efzj-r4i46-yt3vj-rkbiu-43muf-yqe")'
   ```
   Please replace `4rnji-lwrly-r3v6p-y5vby-jrtkt-4efzj-r4i46-yt3vj-rkbiu-43muf-yqe` with the subnet id returned in the previous step.

### Implementation

In the [source code](./src/subnet_backend/src/lib.rs), `CreateCanister` and `CreateCanisterError` structs are added as the input and return types of the [create_canister](https://github.com/dfinity/ic/blob/master/rs/nns/cmc/cmc.did#L220) method on the CMC. They're similar to the [original implementation](https://github.com/dfinity/ic/blob/master/rs/nns/cmc/cmc.did).
