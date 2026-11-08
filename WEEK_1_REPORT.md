# Nervos CKB Builder Track — Week 1 Report

**Personal Repository:** [TTT9954/CKBuilder](https://github.com/TTT9954/CKBuilder)  
**Focus:** Nervos CKB and Blockchain Development  
**Environment:** Windows, CKB Local Devnet (OffCKB)

## 1. Overview

During Week 1, I focused on understanding the Nervos CKB ecosystem, exploring its blockchain architecture, and gaining practical experience with decentralized application development.

I established a local CKB development environment and worked through three practical dApp examples: Simple Transfer, Create DOB, and Simple Hash Lock.

My personal project repository, **CKBuilder**, is the project repository for my Nervos CKB Builder Track learning and development activities. The official `docs.nervos.org` repository was used as a reference and source for practical dApp examples.

The main objective was to gain hands-on experience with CKB accounts, Cells, transactions, Scripts, smart contract deployment, and on-chain verification.

## 2. Personal Project Repository

**Purpose:**
- Organize my Nervos CKB Builder Track development work.
- Maintain project documentation and learning progress.
- Track practical exercises and development milestones.
- Provide a foundation for future independent CKB dApp and smart contract projects.

The official Nervos documentation repository, `docs.nervos.org`, was used as a reference and source for the example projects.

## 3. Learning Materials and Fundamentals

Studied the following materials and concepts:

- Nervos CKB Academy introductory learning materials.
- Nervos CKB protocol architecture and design concepts.
- RFC 0022.
- Bitcoin Whitepaper.
- CKB Cell Model and its UTXO-inspired transaction structure.
- CKB transactions, Inputs, Outputs, Capacity, and transaction fees.
- Lock Scripts, Type Scripts, witnesses, and cell dependencies.
- CKB addresses, Script code hashes, hash types, and Script arguments.

## 4. Local CKB Devnet Setup

**Status: Completed**

Established and configured a local CKB development environment using OffCKB.

Completed activities:

- Installed and configured OffCKB and the CKB node.
- Started the local CKB Devnet successfully.
- Configured the local RPC endpoints:
  - CKB RPC: `http://127.0.0.1:8114`
  - OffCKB RPC proxy: `http://127.0.0.1:28114`
- Generated and inspected 20 pre-funded Devnet accounts.
- Queried blockchain information using CKB RPC and `ckb-cli`.
- Practiced inspecting account balances, transaction information, block height, and live Cells.

## 5. Practical Development Progress

### 5.1 Simple Transfer dApp

**Status: Completed and verified**

Developed and executed the Simple Transfer example using the CCC SDK.

Completed activities:

- Installed project dependencies.
- Configured the application to connect to the local Devnet.
- Connected sender and receiver accounts.
- Constructed and submitted a CKB transfer transaction.
- Verified transaction commitment and output Cells.
- Inspected the change output and transaction fee.

**Verified transaction:**

`0x80f6485e3ce19f075420c460604eb17ecb8b2e439c96b5bb761d22791d419e2f`

Transaction results:

- Receiver output: 62 CKB.
- Sender change output: 41,999,937.99999535 CKB.
- Transaction fee: 0.00000465 CKB.

### 5.2 Create DOB dApp — Spore Protocol

**Status: Completed — transactions committed**

Explored the Create DOB example and successfully created two Spore assets on the local Devnet.

Completed activities:

- Installed project dependencies.
- Executed the project's test suite successfully.
- Configured the application for local Devnet use.
- Synchronized Devnet system scripts.
- Created and submitted two DOB transactions.
- Inspected transaction outputs and Spore Cell data.
- Verified that both Spore Cells were live on Devnet.

**Verified transactions:**

DOB transaction 1:

`0x57d86b1cf75de0babd1fc0ff4251b7bbc3fd8b8f6374055cc35332332310886b`

DOB transaction 2:

`0x1be0fcc562f3b10dae2ad15cb63d4862f704d4a5120a510735ccfeb34ae4ba03`

Each transaction created a Spore output containing 77,714 bytes of data.

### 5.3 Simple Hash Lock dApp

**Status: Completed — deployed and verified on Devnet**

Built, tested, and deployed the Simple Hash Lock example to the local CKB Devnet.

**Build and deployment**

- Installed project dependencies using pnpm.
- Resolved a Windows compatibility issue in the deployment script.
- Built the `hash-lock` contract successfully.
- Executed the local test suite: 2 test suites passed and 12 tests passed.
- Started the Next.js frontend successfully.
- Deployed the Hash Lock contract and synchronized deployment artifacts with the frontend.

Deployment transaction:

`0x767db0cea95d8e2090afd35f002f10544149137d35f523573b1f1ebc7ff5b0bd`

**Hash Lock experiment**

- Generated a Hash Lock address using the preimage `Hello World`.
- Calculated its Blake2b-256 hash.
- Deposited 600 CKB through two 300 CKB deposits.
- Tested an incorrect preimage and received Script error 11.
- Confirmed that the rejected transaction did not consume the original Cells.
- Submitted the correct preimage and successfully committed the transfer.

Successful transaction:

`0x31c6bcee184314c8d11ef5e9abf98f61b4b6beb66c4585ff8bea93e5aeb38c04`

Committed in block 13,632.

**Verified transaction outputs:**

| Output | Capacity | Description |
|---|---:|---|
| Output 0 | 99 CKB | Receiver Cell |
| Output 1 | 200.99999506 CKB | Change returned to Hash Lock |

Transaction fee: 0.00000494 CKB.

One original 300 CKB Cell remained untouched. The final live-cell balance at the original Hash Lock address was 500.99999506 CKB.

Both receiver and Hash Lock live Cells were independently verified using `ckb-cli wallet get-live-cells`.

## 6. Key Achievements

By the end of Week 1, I had:

- Established a working local CKB Devnet.
- Generated and inspected 20 pre-funded Devnet accounts.
- Successfully transferred CKB between accounts.
- Explored Store Data on Cell transaction and data-update workflows.
- Created and verified two Spore-based DOB assets.
- Issued a custom xUDT token and successfully transferred 2,000 xUDT from Account #0 to Account #1. 
- Verified the receiver's 2,000 xUDT balance and confirmed its live Cell.
- Built, tested, and deployed a Hash Lock contract.
- Completed an end-to-end Hash Lock authorization experiment.
- Tested invalid preimage handling and verified atomic transaction rejection.
- Verified successful transactions, outputs, and live Cells using CKB RPC and `ckb-cli`.
- Established CKBuilder as my personal project repository for the Builder Track.

## 7. Conclusion

Week 1 provided a strong practical foundation in Nervos CKB and blockchain development.

I successfully configured a local blockchain environment, interacted with CKB through its CLI and RPC interfaces, created on-chain digital assets, deployed a Script contract, and verified real transaction results.

With CKBuilder as my personal project repository, I now have a dedicated foundation for organizing my development work and progressing toward independent CKB smart contract and dApp development in Week 2.
