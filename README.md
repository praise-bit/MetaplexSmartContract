# Metaplex Smart Contract (Codigo DevQuest #3)

## Overview

This project is a **Solana smart contract** built with **Anchor** and **Metaplex mpl-token-metadata**, demonstrating NFT minting and metadata creation.

The submission was built for the **Codigo DevQuest #3**, a community-powered initiative showcasing Solana developer talent in Nigeria. The bounty was focused on implementing one or more Metaplex features with clean code and unit tests.

---

## Challenge / Bounty Description

**Mission (Metaplex Track):**

- Build a smart contract using the latest Metaplex `mpl-core` + Anchor dependencies.
- Demonstrate at least one Metaplex feature:
  - NFT minting (basic or advanced)
  - Metadata creation & updates
  - Collection creation
  - Asset plugins (optional)

**Requirements:**

- Program must compile successfully.
- Include unit tests in TypeScript or Rust.
- No frontend app required.

**Judging Criteria:**

- Completeness – addresses challenge prompt
- Code Quality – clean, organized, functional
- Imaginativity – novel or creative solutions
- Reusability – can be used or remixed by others

---

## Features Implemented

- Standalone NFT minting simulation
- Metadata PDA creation
- Full TypeScript test demonstrating mint logic
- Clean project structure for easy verification

---

## Project Structure
metaplex_smart_contract/
├─ Anchor.toml
├─ Cargo.toml
├─ programs/
│   └─ metaplex_smart_contract/
│       └─ src/lib.rs
├─ tests/
│   └─ mint_nft_standalone.ts
├─ tsconfig.json
├─ package.json
└─ README.md

---

## Setup & Run

1. **Install dependencies**
```bash
npm install

Build the Program
anchor build

Run the test
npx ts-node tests/mint_nft_standalone.ts

Notes
	•	The test simulates NFT minting and metadata creation on Solana devnet.
	•	No frontend application is required — the focus is on program logic and unit tests.
	•	Fully compatible with CodeSandbox + TypeScript + Anchor.

⸻

References
	•	Anchor Framework
	•	Metaplex Token Metadata
	•	Codigo AI Development Platform

⸻

Author

Mark 
Email: retrovocoded@gmail.com
Location: Port Harcourt, Nigeria

