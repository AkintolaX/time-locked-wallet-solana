# Time-Locked Wallet on Solana

A Solana program that allows users to lock SOL for a specified time period using Program Derived Addresses (PDAs).

## Features

- Lock SOL until a specified unlock timestamp
- Withdraw funds after the unlock time expires
- PDA-based account management for security
- Time validation enforced on-chain
- Web frontend with wallet integration

## Program Details

**Program ID (Devnet):** `JCLD5JBByyLnmndKw6R4iua1Fer8yRfJqp3DGRcVRUd6`

### Instructions

1. `initialize_lock(amount, unlock_timestamp)` - Creates a time-locked wallet
2. `withdraw()` - Withdraws funds after unlock time

### Account Structure

```rust
pub struct TimeLockedWallet {
    pub owner: Pubkey,           // 32 bytes
    pub amount: u64,             // 8 bytes  
    pub unlock_timestamp: i64,   // 8 bytes
    pub bump: u8,                // 1 byte
    pub created_at: i64,         // 8 bytes
}
```

## Project Structure

```
time_locked_wallet/
├── programs/
│   └── time-locked-wallet/
│       └── src/
│           └── lib.rs          # Main program logic
├── tests/
│   └── anchor.ts               # Test suite
├── frontend/
│   └── index.html              # Web interface
├── Anchor.toml                 # Anchor configuration
└── README.md
```

## Setup Instructions

### Prerequisites
- Rust 1.75.0+
- Solana CLI 1.17.31+
- Anchor CLI 0.30.1+
- Node.js and Yarn

### Installation

1. Clone the repository:
```bash
git clone https://github.com/YOUR_USERNAME/time-locked-wallet-solana.git
cd time-locked-wallet-solana
```

2. Install dependencies:
```bash
yarn install
```

3. Build the program:
```bash
anchor build --no-idl
```

4. Deploy to devnet:
```bash
solana config set --url devnet
anchor deploy --provider.cluster devnet
```

### Running the Frontend

1. Start a local server:
```bash
cd frontend
python3 -m http.server 8080
```

2. Open `http://localhost:8080` in your browser

3. Connect your Phantom wallet (set to devnet)

4. Create time locks and test withdrawals

## Testing

```bash
anchor test --skip-build
```

## Known Issues

- IDL generation fails due to dependency conflicts between Anchor CLI versions
- Frontend currently uses demo mode due to instruction format issues
- Core program functionality is deployed and working on devnet

## Implementation Notes

The program uses PDA seeds `["time_locked_wallet", owner.key()]` to derive unique wallet addresses for each user. Time validation is enforced on-chain using Solana's Clock sysvar.

Error handling includes checks for:
- Invalid unlock timestamps (must be in future)
- Invalid amounts (must be > 0)
- Early withdrawal attempts

## License

MIT
