# Subnet Selection
Demonstrate how to create a canister on a given subnet.

## How to interact

### Install the system canisters

In this sample, cycles minting canister is going to be used to create canisters on a given subnet. Please refer [this document](https://github.com/zhangwei983/deploy_icp_ledger/blob/main/by_dfx_nns/README.md) to install the necessary canisters locally.

Here are the two canisters that we're going to use in this example.

```
nns-registry          rwlgt-iiaaa-aaaaa-aaaaa-cai
nns-cycles-minting    rkp4c-7iaaa-aaaaa-aaaca-cai
```

### Create canister

1. deploy the subnet_backend canister
   ```
   dfx deploy
   ```
1. get the subnet id
   ```
   // dfx canister call rwlgt-iiaaa-aaaaa-aaaaa-cai get_subnet_for_canister '(record {principal= principal "rkp4c-7iaaa-aaaaa-aaaca-cai"})'  
   ```
   Since we're testing in the local environment, we only have one subnet available.
1. create canister
   ```
   dfx canister call subnet_backend create_canister '(principal "4rnji-lwrly-r3v6p-y5vby-jrtkt-4efzj-r4i46-yt3vj-rkbiu-43muf-yqe")'
   ```
