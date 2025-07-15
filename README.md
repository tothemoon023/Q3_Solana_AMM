<!-- PROJECT BANNER -->
<p align="center">
  <img src="https://raw.githubusercontent.com/solana-labs/solana/master/docs/src/assets/solana-logo.svg" alt="Solana Logo" width="120"/>
</p>

<h1 align="center">AMM (Automated Market Maker) ⚡️</h1>

<p align="center">
  <b>Solana-based AMM built with Anchor • Swap tokens • Provide liquidity • Earn fees</b>
</p>

<p align="center">
  <a href="https://solana.com/" target="_blank"><img src="https://img.shields.io/badge/Solana-Mainnet-blueviolet?logo=solana"/></a>
  <a href="https://project-serum.github.io/anchor/" target="_blank"><img src="https://img.shields.io/badge/Anchor-Framework-orange?logo=anchor"/></a>
  <img src="https://img.shields.io/badge/License-MIT-green.svg"/>
</p>

---

## 🚀 Overview
This project implements a simple Automated Market Maker (AMM) on <b>Solana</b> using <b>Anchor</b>. It allows users to:

- 💧 <b>Deposit</b> tokens into a liquidity pool
- 🔥 <b>Withdraw</b> tokens and burn LP tokens
- 🔄 <b>Swap</b> between two tokens using a constant product formula
- 💸 <b>Earn</b> trading fees as liquidity providers

---

## ✨ Features

> - ⚖️ Constant product curve (Uniswap-style)
> - 🪙 LP token minting and burning
> - 🛡️ Slippage protection
> - 🔒 Pool locking/unlocking by admin
> - ⛓️ Fully on-chain logic

---

## 📁 Directory Structure
- `programs/amm/` — Solana program (smart contract)
- `tests/` — Integration tests
- `migrations/` — Deployment scripts
- `app/` — (Optional) Frontend or client code

---

## ⚡️ Quick Start

1. **Install dependencies:**
   ```sh
   yarn install
   # or
   npm install
   ```
2. **Build the program:**
   ```sh
   anchor build
   ```
3. **Run tests:**
   ```sh
   anchor test
   ```

---

## 🛠 Usage

> - <b>Deposit:</b> Add liquidity to the pool and receive LP tokens.
> - <b>Withdraw:</b> Burn LP tokens to withdraw your share of the pool.
> - <b>Swap:</b> Trade between the two pool tokens at the current price.

---

## 🤔 What is an AMM?

An **Automated Market Maker (AMM)** is a smart contract that enables users to trade tokens and provide liquidity without relying on a traditional order book. Instead, it uses a mathematical formula (like the constant product formula: `x * y = k`) to determine prices and facilitate swaps. Liquidity providers deposit pairs of tokens into the pool and receive LP tokens representing their share. Traders can swap tokens instantly, and liquidity providers earn a portion of the trading fees.

This AMM is inspired by Uniswap and built for the Solana blockchain using the Anchor framework. It supports:
- Depositing tokens to earn fees
- Withdrawing tokens by burning LP tokens
- Swapping between two tokens
- Slippage protection and admin controls

---

## 🧩 Workflow Diagrams

### 1. High-Level Flow

```mermaid
flowchart TD
    A["User"] -- "Deposit/Withdraw" --> B["AMM Program"]
    B -- "Token Transfer" --> C["Pool Vaults"]
    B -- "Mint/Burn" --> D["LP Token Mint"]
    B -- "Update" --> E["Config Account"]
    A -- "Swap" --> B
    B -- "Token Transfer" --> C
    C -.->|"Holds"| E
    D -.->|"Represents Share"| E
    style A fill:#e3f6fc,stroke:#1e90ff,stroke-width:2px
    style B fill:#fffbe6,stroke:#f7b731,stroke-width:2px
    style C fill:#eafbe6,stroke:#20bf6b,stroke-width:2px
    style D fill:#fce6f6,stroke:#eb3b5a,stroke-width:2px
    style E fill:#f6e6fc,stroke:#8854d0,stroke-width:2px
```

> **How to read this:**
> - The user interacts with the AMM program to deposit, withdraw, or swap tokens.
> - The AMM program manages token transfers, LP token minting/burning, and updates the pool configuration.
> - Colorful boxes represent different on-chain accounts and their roles.

### 2. Sequence Diagram

```mermaid
sequenceDiagram
    participant U as User
    participant F as Frontend
    participant P as AMM Program
    participant V as Pool Vaults
    participant L as LP Token Mint
    participant C as Config Account

    U->>F: Initiate Deposit/Withdraw/Swap
    F->>P: Send Transaction
    P->>V: Transfer Tokens
    P->>L: Mint/Burn LP Tokens
    P->>C: Update Pool State
    P-->>F: Return Result
    F-->>U: Show Confirmation
    %% Styling for color
    %% Note: Mermaid sequence diagrams have limited color support, but we can use actor styling
    %% This is a visual hint for Mermaid live editors, but GitHub may not render it
    %% actor U fill:#e3f6fc,stroke:#1e90ff
    %% actor F fill:#fffbe6,stroke:#f7b731
    %% actor P fill:#fce6f6,stroke:#eb3b5a
    %% actor V fill:#eafbe6,stroke:#20bf6b
    %% actor L fill:#fce6f6,stroke:#eb3b5a
    %% actor C fill:#f6e6fc,stroke:#8854d0
```

> **How to read this:**
> - The user starts an action in the frontend, which sends a transaction to the AMM program.
> - The program processes the request, updates vaults, mints/burns LP tokens, and updates the config.
> - Results are returned to the frontend and shown to the user.

---

## 🤝 Contributing
Pull requests and issues are welcome!

---

## 📄 License
MIT

---

<p align="center">
  <sub>Powered by <a href="https://solana.com/">Solana</a> & <a href="https://project-serum.github.io/anchor/">Anchor</a></sub>
</p> 
