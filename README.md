# Agent Escrow Contract

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Rust](https://img.shields.io/badge/rust-1.70%2B-orange)](https://www.rust-lang.org/)
[![Soroban](https://img.shields.io/badge/Soroban-25.1.0-blue)](https://soroban.stellar.org/)
[![CI](https://github.com/AgentPayEscrow/agent-escrow-contract/actions/workflows/ci.yml/badge.svg)](https://github.com/AgentPayEscrow/agent-escrow-contract/actions/workflows/ci.yml)

## Overview

Agent Escrow is a production-ready Soroban smart contract that enables AI agents to make autonomous payments within predefined spending limits, supervised by human overseers.

## Features

| Feature | Description |
|---------|-------------|
| **Agent Management** | Create, pause, resume, and terminate AI agents |
| **Spending Limits** | Per-transaction, daily, weekly, and monthly caps |
| **Multi-token Support** | XLM, USDC, and any Stellar asset |
| **Human Oversight** | Supervisors can pause agents and adjust limits |
| **Transaction History** | Complete on-chain record of all payments |
| **Fee Collection** | Platform fee configured by treasury |

## Architecture
