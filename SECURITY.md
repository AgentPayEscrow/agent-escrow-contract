# Security Policy

## Reporting a Vulnerability

**Email:** security@agentpay.io

Please include:
- Description of the issue
- Steps to reproduce
- Potential impact

## Smart Contract Security

This contract handles user funds. Security is critical.

- All functions use `require_auth()`
- Spending limits enforced
- Emergency pause available
- Multi-signature for admin functions

## Supported Versions

| Version | Supported |
|---------|-----------|
| 1.x.x | ✅ |
