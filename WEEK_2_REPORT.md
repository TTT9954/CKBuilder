# Week 2 Progress Report – CCC Fundamentals

## Objective

Learn the CCC (Common Knowledge Base) SDK and build a CKB dApp using Next.js, TypeScript, and JoyID wallet integration. Implement a CKB transfer flow through the dApp UI and verify the transaction on-chain.

## 1. Development Environment

| Item | Configuration |
|---|---|
| Framework | Next.js 14.2.18 |
| Language | TypeScript |
| CCC React connector | `@ckb-ccc/connector-react` 1.1.9 |
| CCC Core | 1.19.1 |
| Wallet | JoyID |
| Local development network | OffCKB Devnet |
| dApp transaction network | CKB Testnet |
| Development RPC (initial local setup) | `http://127.0.0.1:28114` |
| CKB Testnet RPC | `wss://testnet.ckb.dev/ws` |

## 2. Completed Tasks

| No. | Task | Status |
|---:|---|---|
| 1 | Created a Next.js and TypeScript CCC dApp | Completed |
| 2 | Integrated the CCC Provider and wallet connector | Completed |
| 3 | Connected JoyID wallet | Completed |
| 4 | Configured the app to connect to local OffCKB Devnet | Completed |
| 5 | Implemented live block-height monitoring | Completed |
| 6 | Displayed the connected wallet's CKB balance | Completed |
| 7 | Implemented a CKB transfer form in the dApp UI | Completed |
| 8 | Integrated transaction signing through JoyID | Completed |
| 9 | Displayed the transaction hash and confirmation status | Completed |
| 10 | Switched the dApp to public CKB Testnet for JoyID transactions | Completed |
| 11 | Tested a CKB transfer through the dApp UI | Completed |
| 12 | Independently verified the transaction using public CKB Testnet RPC | Completed |

## 3. CKB Transfer Test

A 62 CKB transfer was submitted through the CCC dApp using JoyID on public CKB Testnet.

| Field | Result |
|---|---|
| Network | CKB Testnet |
| Wallet | JoyID |
| Testnet funding before test | 10,000 CKB (faucet funding, as reported) |
| Sender | JoyID wallet |
| Recipient | OffCKB Account #1 address used as the recipient address in the dApp |
| Transfer amount | **62 CKB** |
| Submission method | CCC dApp UI |
| Wallet approval | Approved through JoyID |
| Transaction status in dApp | **Committed** |

### Transaction Details

| Field | Result |
|---|---|
| Transaction hash | `0x748edfcd45a01d5d8e5df588c977ea512f625d1652c2063000c18c79239b6d6c` |
| Block number (decimal) | **23,074,412** |
| Block number (hexadecimal) | `0x160166c` |
| Transaction index | `0x1` |
| Block hash | `0x6265da8e85c46d2d1010a03ac18cce961984c3e825c7284f3dd6c04e4fea1955` |
| Independent verification | Public CKB Testnet `get_transaction` RPC response returned `status: committed` |

### Output Verification

The public Testnet RPC response included two outputs:

| Output index | Lock script args | Capacity | Description |
|---:|---|---:|---|
| 0 | `0x758d311c8483e0602dfad7b69d9053e3f917457d` | **62 CKB** | Recipient output; matches Account #1 lock args |
| 1 | `0x000162b9826777593a0eedd0a7a9e3069f81d58357e5` | Remaining capacity | Change output to the JoyID wallet |

Explorer link: [View transaction on CKB Testnet Explorer](https://testnet.explorer.nervos.org/transaction/0x748edfcd45a01d5d8e5df588c977ea512f625d1652c2063000c18c79239b6d6c)

## 4. Technical Issues and Resolution

- The initial attempt to spend JoyID funds through the local OffCKB Devnet failed with `TransactionFailedToResolve: Unknown(OutPoint(...))`.
- Investigation showed that the referenced outpoint existed on public CKB Testnet (`live`) but was unknown on the local OffCKB Devnet.
- The CCC provider and JoyID signer initially used the local Devnet RPC, while JoyID's transaction preparation referenced Testnet-related cells and dependencies.
- The dApp provider was changed to use the public CKB Testnet client (`new ccc.ClientPublicTestnet()`), and the network label was updated to **CKB Testnet**.
- JoyID was funded with 10,000 CKB on Testnet using the Nervos faucet, as reported during testing.
- The CKB transfer then succeeded. The Testnet RPC independently confirmed the transaction as committed.

## 5. Local Devnet and Testnet Separation

The local OffCKB Devnet was retained for local development and earlier labs. Its balances are separate from public CKB Testnet balances and were not moved by changing the CCC dApp's default network.

The dApp's active configuration for the successful JoyID transfer uses public CKB Testnet. Local OffCKB assets remain on the local Devnet.

## 6. Final Results

- Built a CCC-based CKB dApp with Next.js and TypeScript.
- Integrated JoyID wallet connection and CKB balance display.
- Added a CKB transfer form and JoyID transaction approval flow.
- Added transaction hash and committed-status display.
- Successfully transferred 62 CKB through the dApp on public CKB Testnet.
- Independently verified the transaction hash, outputs, and committed block using the public Testnet RPC.
- Kept the xUDT balance hidden from the dApp UI.


