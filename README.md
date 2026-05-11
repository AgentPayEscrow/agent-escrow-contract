# Agent Escrow Contract

[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](LICENSE)
[![Rust](https://img.shields.io/badge/Rust-1.70%2B-orange)](https://www.rust-lang.org/)
[![Soroban](https://img.shields.io/badge/Soroban-25.1.0-purple)](https://soroban.stellar.org/)

## Overview

Soroban smart contract for AI agent spending limit escrow with human overseer.

## Features

| Feature | Description |
|---------|-------------|
| Agent Management | Create, pause, resume, terminate agents |
| Spending Limits | Daily, weekly, monthly caps |
| Human Oversight | Supervisors can pause and adjust limits |
| Multi-token | XLM, USDC, any Stellar asset |
| Dispute Resolution | Raise and resolve disputes |

## Contract Functions

| Function | Description |
|----------|-------------|
| `create_agent` | Create new AI agent |
| `deposit` | Fund an agent |
| `set_limits` | Set spending caps |
| `pay` | Agent makes payment |
| `pause_agent` | Pause agent |
| `resume_agent` | Resume agent |
| `raise_dispute` | Dispute a transaction |

## Deployment

```bash
cargo build --target wasm32-unknown-unknown --release
