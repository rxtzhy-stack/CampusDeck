# CampusDeck
<img width="1919" height="933" alt="Screenshot 2026-05-26 124757" src="https://github.com/user-attachments/assets/23ff00dd-1bc6-48ec-bf88-cdd24b4042dd" />


Peer-to-Peer Textbook & Material Escrow for University Students.

## Problem & Solution
University students in Dhaka, Bangladesh, spend up to 40% of their monthly allowance on mandatory textbooks, yet often face financial fraud when purchasing used materials from unverified listings on standard social networks. CampusDeck addresses this through an elegant web app interface integrated with a Soroban escrow contract that locks a student's payment, releasing it to the seller immediately upon physical campus handover confirmed via a dynamic QR code scan.

## Timeline
* **Day 1:** Architecture design, local storage definitions, and contract compile logic testing.
* **Day 2:** Building web app frontend workflow logic, QR payload parameters, and embedding the Soroban Client SDK.

## Stellar Features Used
* USDC / XLM Transfers
* Soroban Smart Contracts
* Trustlines

## Vision and Purpose
To insulate student operational budgets from localized textbook marketplace scams using secure, trustless smart contract primitives that combine software tracking with physical student verification.

## Prerequisites
* Rust v1.75+
* Soroban CLI v20.0.0+
* Target `wasm32-unknown-unknown`

## Build, Test and Deploy

### How to Build
```bash
soroban contract build
