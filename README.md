Treasury Vault Program

A comprehensive Solana smart contract for managing decentralized treasury operations with programmable payout logic, streaming payments, and multi-role access control.

## 🎯 Bounty Submission - Codigo DevQuest

**Track:** DAO/Governance - Onchain Treasury

**Challenge Addressed:** Treasury vault (SOL/SPL Token) with programmable payout logic

This project implements a feature-rich treasury vault system that goes beyond the basic requirements by providing:
- ✅ Treasury vault for SOL and SPL tokens
- ✅ Programmable payout logic with scheduled payments
- ✅ Role-based withdrawal permissions (Admin/Treasurer/Recipients)
- ✅ Spending limits per epoch (daily, weekly, monthly)
- ✅ Recipient whitelisting system
- ✅ Token-gated withdrawal rights
- ✅ **Bonus:** Streaming payment schedules for continuous payouts
- ✅ **Bonus:** Emergency withdrawal capabilities
- ✅ Comprehensive unit test coverage

## 🏗️ Architecture Overview

### Core Components

1. **Treasury Configuration** - Main vault with spending controls
2. **Recipients Management** - Whitelisted payment recipients  
3. **Payout Schedules** - Recurring payment automation
4. **Streaming Schedules** - Continuous payment streams
5. **Token Vaults** - SPL token management
6. **Access Control** - Multi-role permission system

### Key Features

- **Multi-Asset Support**: Handle both SOL and SPL tokens
- **Flexible Payment Models**: 
  - One-time payments
  - Recurring scheduled payouts
  - Streaming payments (vesting-like functionality)
- **Spending Controls**: Daily, weekly, and monthly limits with automatic resets
- **Role-Based Access**: Admin, Treasurer, and Recipient roles
- **Token Gating**: Optional token-based access control
- **Emergency Features**: Admin-only emergency withdrawal functions

## 🚀 Getting Started

### Prerequisites

- [Rust](https://www.rust-lang.org/tools/install) 1.70+
- [Solana CLI](https://docs.solana.com/cli/install-solana-cli-tools) 1.18+
- [Anchor Framework](https://www.anchor-lang.com/docs/installation) 0.31+
- [Node.js](https://nodejs.org/) 16+

### Installation

1. **Clone the repository:**
```bash
git clone https://github.com/ussyalfaks/treasury_vault_program.git
cd treasury_vault_program
```

2. **Install dependencies:**
```bash
# Install Anchor dependencies
anchor build

# Install Node.js dependencies
npm install
# or
yarn install
```

3. **Build the program:**
```bash
anchor build
```

4. **Run tests:**
```bash
anchor test
```

## 📖 Usage Guide

### Basic Treasury Operations

#### 1. Initialize Treasury
```typescript
import { Program, AnchorProvider } from "@coral-xyz/anchor";
import { TreasuryVault } from "./target/types/treasury_vault";

// Initialize a new treasury
await program.methods
  .initializeTreasury(
    "DAO Treasury",                    // name
    "Main DAO treasury for operations", // description
    treasurerPublicKey,                // treasurer authority
    1000000000,                        // daily limit (1 SOL in lamports)
    5000000000,                        // weekly limit (5 SOL)
    20000000000,                       // monthly limit (20 SOL)
    false,                             // require_token_gate
    null,                              // token_gate_mint
    0                                  // token_gate_amount
  )
  .accounts({
    admin: adminPublicKey,
    treasury: treasuryPDA,
    systemProgram: SystemProgram.programId,
  })
  .rpc();
```

#### 2. Deposit Funds
```typescript
// Deposit SOL
await program.methods
  .depositSol(
    new BN(1000000000), // 1 SOL in lamports
    "DAO Treasury"      // treasury name for PDA
  )
  .accounts({
    treasury: treasuryPDA,
    depositor: depositorPublicKey,
  })
  .rpc();

// Deposit SPL Tokens (after initializing token vault)
await program.methods
  .depositToken(
    new BN(1000000000), // token amount
    "DAO Treasury"
  )
  .accounts({
    treasury: treasuryPDA,
    tokenVault: tokenVaultPDA,
    tokenMint: tokenMintPublicKey,
    depositor: depositorPublicKey,
    source: sourceTokenAccount,
    destination: vaultTokenAccount,
    authority: depositorPublicKey,
  })
  .rpc();
```

#### 3. Add Recipients
```typescript
await program.methods
  .addRecipient(
    recipientPublicKey,
    "Development Team",
    1, // role: 1 = Privileged, 0 = Regular
    "DAO Treasury"
  )
  .accounts({
    treasury: treasuryPDA,
    recipient: recipientPDA,
    authority: adminPublicKey,
    systemProgram: SystemProgram.programId,
  })
  .rpc();
```

#### 4. Create Payment Schedules
```typescript
// Create recurring payout schedule
await program.methods
  .createPayoutSchedule(
    recipientPublicKey,
    1, // schedule_id
    new BN(500000000), // 0.5 SOL per payout
    null, // token_mint (null for SOL)
    new BN(Date.now() / 1000), // start_time
    86400, // interval_seconds (daily)
    30, // max_executions (30 days)
    "DAO Treasury"
  )
  .accounts({
    treasury: treasuryPDA,
    recipient: recipientPDA,
    payoutSchedule: payoutSchedulePDA,
    authority: treasurerPublicKey,
    systemProgram: SystemProgram.programId,
  })
  .rpc();
```

#### 5. Create Streaming Schedules
```typescript
// Create streaming payment (like vesting)
await program.methods
  .createStreamingSchedule(
    recipientPublicKey,
    1, // stream_id
    new BN(10000000000), // total_amount (10 SOL)
    new BN(115740), // amount_per_second (~0.01 SOL per day)
    new BN(Date.now() / 1000), // start_time
    new BN(Date.now() / 1000 + 86400 * 30), // cliff_time (30 days)
    86400 * 365, // duration_seconds (1 year)
    null, // token_mint (null for SOL)
    "DAO Treasury"
  )
  .accounts({
    treasury: treasuryPDA,
    recipient: recipientPDA,
    streamingSchedule: streamingSchedulePDA,
    authority: treasurerPublicKey,
    systemProgram: SystemProgram.programId,
  })
  .rpc();
```

### Advanced Features

#### Token Gating
```typescript
// Initialize treasury with token gating
await program.methods
  .initializeTreasury(
    "Token Gated Treasury",
    "Treasury requiring token ownership",
    treasurerPublicKey,
    1000000000, 5000000000, 20000000000,
    true, // require_token_gate
    governanceTokenMint, // token_gate_mint
    new BN(100000000) // token_gate_amount (100 tokens)
  )
  // ... accounts
  .rpc();
```

#### Emergency Operations
```typescript
// Emergency SOL withdrawal (Admin only)
await program.methods
  .emergencyWithdrawSol(
    new BN(1000000000), // amount
    "DAO Treasury"
  )
  .accounts({
    treasury: treasuryPDA,
    admin: adminPublicKey,
  })
  .rpc();
```

## 🧪 Testing

The program includes comprehensive unit tests covering all functionality:

```bash
# Run all tests
anchor test

# Run specific test file
anchor test -- --grep "initialize_treasury"

# Run tests with detailed output
anchor test -- --verbose
```

### Test Coverage
- ✅ Treasury initialization and configuration
- ✅ SOL and token deposits/withdrawals
- ✅ Recipient management
- ✅ Payout schedule creation and execution
- ✅ Streaming schedule functionality
- ✅ Access control and permissions
- ✅ Spending limits and resets
- ✅ Emergency operations
- ✅ Error conditions and edge cases

## 🏛️ Use Cases

### 1. DAO Treasury Management
- Manage community funds with spending limits
- Automate recurring payments to contributors
- Token-gate access to treasury functions

### 2. Payroll & Compensation
- Set up recurring salary payments
- Create vesting schedules for team members
- Manage contractor payments with automation

### 3. Grant Programs
- Distribute grants on scheduled intervals
- Stream payments over project milestones
- Control spending with epoch-based limits

### 4. Investment Management
- Time-locked funding releases
- Automated dividend distributions
- Multi-signature-like controls with roles

## 🔧 Program Instructions

| Instruction | Description |
|-------------|-------------|
| `initialize_treasury` | Create new treasury with configuration |
| `update_treasury_config` | Modify treasury settings |
| `deposit_sol` | Deposit SOL to treasury |
| `initialize_token_vault` | Setup SPL token support |
| `deposit_token` | Deposit SPL tokens |
| `add_recipient` | Add payment recipient |
| `update_recipient` | Modify recipient settings |
| `create_payout_schedule` | Setup recurring payments |
| `update_payout_schedule` | Modify payment schedule |
| `execute_sol_payout` | Process SOL payment |
| `execute_token_payout` | Process token payment |
| `create_streaming_schedule` | Setup streaming payments |
| `withdraw_from_stream` | Withdraw available stream funds |
| `cancel_stream` | Cancel active stream |
| `emergency_withdraw_sol` | Emergency SOL withdrawal |
| `emergency_withdraw_token` | Emergency token withdrawal |

## 📊 Account Structure

### Treasury Config
- Admin and treasurer authorities
- Spending limits (daily/weekly/monthly)
- Token gating configuration
- Balance tracking

### Recipients
- Wallet address and metadata
- Role-based permissions
- Active status management

### Payout Schedules  
- Recurring payment automation
- Execution tracking
- Flexible intervals

### Streaming Schedules
- Continuous payment streams
- Cliff and vesting periods
- Real-time calculations

## 🚀 Deployment

### Local Development
```bash
# Start local validator
solana-test-validator

# Deploy program
anchor deploy

# Get program ID
solana address -k target/deploy/treasury_vault-keypair.json
```

### Devnet Deployment
```bash
# Configure Solana CLI for devnet
solana config set --url https://api.devnet.solana.com

# Build and deploy
anchor build
anchor deploy --provider.cluster devnet
```

### Mainnet Deployment
```bash
# Configure for mainnet
solana config set --url https://api.mainnet-beta.solana.com

# Deploy with production settings
anchor deploy --provider.cluster mainnet
```

## 🔐 Security Considerations

- **Access Control**: Multi-role system with admin/treasurer separation
- **Spending Limits**: Automatic epoch-based spending controls
- **Emergency Functions**: Admin-only emergency withdrawal capabilities
- **Input Validation**: Comprehensive parameter validation
- **Reentrancy Protection**: Safe state management patterns
- **Token Safety**: Proper SPL token handling

## 🛠️ Built With

- **Anchor Framework 0.31.1** - Solana development framework
- **Rust** - Systems programming language
- **Solana Program Library** - SPL token integration
- **TypeScript** - Testing and client development

## 🤝 Contributing

This project was built for the Codigo DevQuest bounty. Feel free to:

1. Fork the repository
2. Create feature branches
3. Add comprehensive tests
4. Submit pull requests

### Development Setup
```bash
# Fork and clone
git clone https://github.com/your-username/treasury_vault_program.git

# Create feature branch
git checkout -b feature/your-feature

# Make changes and test
anchor test

# Commit and push
git commit -m "Add your feature"
git push origin feature/your-feature
```

## 📝 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## 👏 Acknowledgments

- **Codigo Platform** - AI-powered Solana development platform
- **Superteam Nigeria** - Community and bounty organization
- **Anchor Framework** - Solana development tools
- **Solana Foundation** - Blockchain infrastructure

## 📞 Contact

- **Developer**: @ussyalfaks
- **GitHub**: [ussyalfaks](https://github.com/ussyalfaks)
- **Repository**: [treasury_vault_program](https://github.com/ussyalfaks/treasury_vault_program)

---
<img width="1440" height="900" alt="Screenshot 2025-08-11 at 12 02 46 PM" src="https://github.com/user-attachments/assets/1d16d3b3-010f-43cd-84bc-73d7d5f28bc2" />

**Program ID**: `FZF2W7peTaeeAYkL5sz81drHMNW5qQemerM1Cx8FViHC`

Built with ❤️ using [Codigo](https://codigo.ai) for the Superteam Nigeria DevQuest bounty.
