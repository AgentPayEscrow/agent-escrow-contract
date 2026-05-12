# Agent Escrow Contract
Rust Soroban Stellar Mainnet License: MIT

The Agent Escrow Contract is a production-ready Soroban smart contract that enables AI agents to make autonomous payments with spending limits and human oversight.

## 📚 Table of Contents
- [Overview](#overview)
- [Features](#features)
- [Tech Stack](#tech-stack)
- [Project Structure](#project-structure)
- [Quick Start](#quick-start)
- [Contract Functions](#contract-functions)
- [Contributing](#contributing)

## 🎯 Overview
Agent Escrow Contract enables:

- AI agent creation with spending limits
- Autonomous payments within limits
- Human overseer supervision
- Complete on-chain transaction history

## ✨ Features
- **Agent Management** - Create, pause, resume, terminate agents
- **Spending Limits** - Daily, weekly, monthly caps
- **Human Oversight** - Supervisors can pause and adjust limits
- **Multi-token Support** - XLM, USDC, any Stellar asset
- **Dispute Resolution** - Raise and resolve payment disputes

## 🛠️ Tech Stack
- Rust 1.70+
- Soroban SDK 25.1.0
- WASM32 target

## 📁 Project Structure
src/
├── lib.rs # Main contract entry
├── agent/ # Agent management module
│ └── mod.rs
├── limits/ # Spending limits module
│ └── mod.rs
├── payment/ # Payment execution module
│ └── mod.rs
├── dispute/ # Dispute resolution module
│ └── mod.rs
├── overseer/ # Human overseer module
│ └── mod.rs
└── storage/ # Storage keys module
└── mod.rs

## ⚡ Quick Start
```bash
# Build the contract
cargo build --target wasm32-unknown-unknown --release

# Run tests
cargo test

# Check formatting
cargo fmt

# Run linter
cargo clippy -- -D warnings
📜 Contract Functions
Function	Description
create_agent	Create new AI agent
deposit	Fund an agent
set_limits	Set spending caps
pay	Agent makes payment
pause_agent	Pause agent
resume_agent	Resume agent
raise_dispute	Dispute a transaction
resolve_dispute	Resolve dispute
add_overseer	Add new overseer
get_agent	Get agent details
get_limits	Get spending limits
get_transaction	Get transaction details
🤝 Contributing
Pull requests welcome! See CONTRIBUTING.md for guidelines.

Part of the AgentPay Escrow Platform | Built for Stellar

---

## Backend Repo README (using YOUR actual structure)

Replace `agent-escrow-indexer/README.md` with this:

```markdown
# Agent Escrow Indexer
NestJS TypeScript PostgreSQL Redis Stellar Mainnet License: MIT

The Agent Escrow Indexer is the backend API for the Agent Escrow platform. It provides agent management, payment processing, spending limits, and real-time WebSocket updates.

## 📚 Table of Contents
- [Overview](#overview)
- [Features](#features)
- [Tech Stack](#tech-stack)
- [Project Structure](#project-structure)
- [Quick Start](#quick-start)
- [API Endpoints](#api-endpoints)
- [Contributing](#contributing)

## 🎯 Overview
Agent Escrow Indexer enables:

- Agent creation and management
- Payment execution with spending limits
- Real-time WebSocket notifications
- Transaction history and audit trails

## ✨ Features
- **Agent Management** - Create, pause, resume, terminate agents
- **Payment Processing** - Execute and track payments
- **Spending Limits** - Daily, weekly, monthly caps
- **WebSocket Updates** - Real-time payment status
- **JWT Authentication** - Secure API access

## 🛠️ Tech Stack
- NestJS 10
- TypeScript
- PostgreSQL + TypeORM
- Redis + BullMQ
- Socket.IO

## 📁 Project Structure
src/
├── main.ts # Application entry
├── app.module.ts # Root module
├── modules/
│ ├── agents/ # Agent management
│ │ ├── agents.module.ts
│ │ ├── agents.controller.ts
│ │ ├── agents.service.ts
│ │ ├── dto/
│ │ │ └── create-agent.dto.ts
│ │ └── entities/
│ │ └── agent.entity.ts
│ └── payments/ # Payment processing
│ ├── payments.module.ts
│ ├── payments.controller.ts
│ └── payments.service.ts  

 
## ⚡ Quick Start
```bash
# Install dependencies
npm install

# Copy environment variables
cp .env.example .env

# Start development server
npm run start:dev
📡 API Endpoints
Method	Endpoint	Description
GET	/api/v1/agents	List all agents
GET	/api/v1/agents/:id	Get agent by ID
POST	/api/v1/agents	Create agent
POST	/api/v1/agents/:id/pause	Pause agent
POST	/api/v1/agents/:id/resume	Resume agent
POST	/api/v1/payments	Execute payment
GET	/api/v1/transactions/:id	Get transaction
🤝 Contributing
Pull requests welcome! See CONTRIBUTING.md for guidelines.
