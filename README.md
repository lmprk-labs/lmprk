<p align="center">
  <img src="assets/banner.png" alt="Lmprk banner" width="100%"/>
</p>

<h1 align="center">Lmprk</h1>

<p align="center">
  <em>Solana ZK light client. Verify any account without trusting an RPC.</em>
</p>

<p align="center">
  <img alt="License" src="https://img.shields.io/badge/license-MIT-3D2817.svg?style=flat-square"/>
  <img alt="Rust" src="https://img.shields.io/badge/rust-stable-3A3F4B.svg?style=flat-square&logo=rust&logoColor=FFD93D"/>
  <img alt="TypeScript" src="https://img.shields.io/badge/typescript-5.x-1B2A4E.svg?style=flat-square&logo=typescript&logoColor=F5F2E8"/>
  <img alt="Solana" src="https://img.shields.io/badge/solana-mainnet-5C5658.svg?style=flat-square&logo=solana&logoColor=FFD93D"/>
  <img alt="CI" src="https://img.shields.io/badge/ci-passing-3D2817.svg?style=flat-square"/>
  <img alt="Version" src="https://img.shields.io/badge/version-0.5.0-FFD93D.svg?style=flat-square"/>
  <a href="https://lmprk.fun"><img alt="Site" src="https://img.shields.io/badge/site-lmprk.fun-3A3F4B.svg?style=flat-square"/></a>
  <a href="https://x.com/lmprk_io"><img alt="X" src="https://img.shields.io/badge/x-@lmprk__io-1B2A4E.svg?style=flat-square&logo=x&logoColor=F5F2E8"/></a>
</p>

---

## What is Lmprk

Lmprk is a Solana light client. It lets a mobile wallet, a dApp, or an
embedded client verify on-chain account state by checking an Ed25519
signature aggregation over the slot state root, then walking a blake3
merkle inclusion path to the account leaf. The client does not have to
trust the RPC that served the proof; only the validator set and the
protocol.

The name comes from a coastal lighthouse: the keeper does not track every
ship. The keeper sends the light. The ships verify their own position.

One light. All ships verified.

## How it works

```mermaid
flowchart LR
    A[client] -->|address, slot| B[host backend]
    B -->|RPC pick| C[lmprk-rpc]
    C -->|account + sigs| D[lmprk-core]
    D -->|StateProof| E[lmprk-verifier]
    E -->|threshold ok| F[client verify]
    F -->|computed root| G[verified]
```

The building blocks:

| Crate | Role |
|-------|------|
| `lmprk-core` | proof primitives: blake3 hashing, merkle paths, slot snapshots, state proof builder |
| `lmprk-verifier` | Ed25519 signature aggregation, threshold checking, commitment helper |
| `lmprk-rpc` | failover-aware RPC client policy used by the host service |
| `sdk/typescript` | a TypeScript mirror of the core types and the verify path |

## Features