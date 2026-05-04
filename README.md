# Lend A Hand

A Soroban-based micro-lending protocol for gig workers in Manila, enabling collateral-backed instant credit using stablecoins.

## Problem
A freelance motorcycle delivery rider in Manila struggles to get small emergency cash advances because banks reject him due to lack of credit history, forcing him to borrow from informal lenders with high daily interest rates.

## Solution
The rider deposits small savings into a Soroban smart contract lending pool and instantly borrows stablecoin credit against it, with all deposits, loans, and repayments executed transparently on-chain.

## Timeline
- Deposit collateral
- Borrow up to 50% LTV
- Repay loan to unlock funds

## Stellar Features Used
- USDC / XLM transfers
- Soroban smart contracts
- Trustless collateral tracking

## Vision
Replace informal lending systems with transparent, programmable credit access for gig workers in Southeast Asia.

## Prerequisites
- Rust
- Soroban CLI

## Build
```
bash
soroban contract build 
```
## Test
``` 
cargo test 
```

## Deploy
```
soroban contract deploy \
  --wasm target/wasm32-unknown-unknown/release/lend_a_hand.wasm \
  --network testnet \
  --source alice
```

Example Invocation
```
stellar contract invoke \
  --id <CONTRACT_ID> \
  --source alice \
  --network testnet \
  -- \
  deposit \
  --user alice \
  --token <TOKEN> \
  --amount 100
```

## License


## Contract

CC54GEVRNMQEL2RONIWKR2FCQBDBRC475OASTQKBL54JHVAILV5YTQ3H
https://stellar.expert/explorer/testnet/tx/08973b70572f3a281b8c9f40731695fead88b0ba8071f72b31d75aedca33d480
https://lab.stellar.org/r/testnet/contract/CC54GEVRNMQEL2RONIWKR2FCQBDBRC475OASTQKBL54JHVAILV5YTQ3H?$=network$id=testnet&label=Testnet&horizonUrl=https:////horizon-testnet.stellar.org&rpcUrl=https:////soroban-testnet.stellar.org&passphrase=Test%20SDF%20Network%20/;%20September%202015;&smartContracts$explorer$contractId=CC54GEVRNMQEL2RONIWKR2FCQBDBRC475OASTQKBL54JHVAILV5YTQ3H;;
