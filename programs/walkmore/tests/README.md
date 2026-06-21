# Walkmore Program Tests

This directory contains comprehensive test cases for the Walkmore Solana program. The tests are organized using a modular structure to avoid code repetition and make maintenance easier.

## Directory Structure

```
tests/
├── utils.rs                 # Shared utility functions and helpers
├── test_initialize.rs       # Program initialization tests
├── test_pool.rs             # Pool creation and lifecycle tests
├── test_challenge.rs        # 1v1 challenge tests
└── README.md               # This file
```

## Test Organization

### `utils.rs`

Contains all shared functionality to avoid repetition:

- **Setup**: Program initialization and account creation
- **Token Helpers**: Mint creation, ATA management, token transfers
- **PDA Helpers**: Program Derived Address calculations for all account types
- **Transaction Execution**: Helper functions to send and manage transactions
- **Instruction Builders**: Functions that construct each program instruction with proper accounts and data

### `test_initialize.rs`

Tests basic program initialization:

- `test_initialize` - Verifies program can be initialized successfully

### `test_pool.rs`

Tests pool-related functionality:

- `test_create_pool` - Creates a new pool
- `test_create_pool_and_join` - Creates pool and user joins
- `test_create_user_account_for_pool` - Creates user data account for pool participation
- `test_update_steps_pool` - Updates user steps in a pool

### `test_challenge.rs`

Tests 1v1 challenge functionality:

- `test_create_challenge_onevone` - Creates a new 1v1 challenge
- `test_accept_challenge` - Player 2 accepts a challenge
- `test_update_steps_challenge` - Players update their steps during challenge
- `test_reject_challenge` - Player 2 rejects a challenge

## Running Tests

### Run all tests:

```bash
cargo test --test test_*
```

### Run specific test file:

```bash
cargo test --test test_pool
cargo test --test test_challenge
```

### Run specific test:

```bash
cargo test --test test_pool test_create_pool
```

### Run with output:

```bash
cargo test -- --nocapture --test-threads=1
```

## Key Design Patterns

### 1. Avoid Code Repetition

All common setup, token management, and instruction creation logic is in `utils.rs`. Each test file imports these utilities via `mod utils; use utils::*;`.

### 2. Helper Functions Pattern

Each instruction has a corresponding helper function:

```rust
pub fn create_pool(
    maker: &Keypair,
    pool: &Pubkey,
    seed: u128,
    // ... parameters
) -> Instruction {
    // Build and return instruction
}
```

### 3. PDA Calculation Helpers

PDAs are calculated using helper functions to ensure consistency:

```rust
let (pool_pda, pool_bump) = get_pool_pda(&maker.pubkey(), 100);
let (user_data_pda, user_data_bump) = get_user_data_pda(&user.pubkey(), &pool_pda);
```

### 4. Setup Isolation

Each test independently sets up its required state, but uses shared `setup()` function:

```rust
let (mut svm, payer) = setup();  // Creates fresh SVM and payer with airdrop
```

## Adding New Tests

To add a new test:

1. **Create a new test file** (or add to existing):

   ```rust
   mod utils;
   use utils::*;

   #[test]
   fn test_new_feature() {
       let (mut svm, payer) = setup();
       // Your test code
   }
   ```

2. **Reuse instruction builders**:

   ```rust
   let instruction = create_pool(...);
   send(&mut svm, &payer, &[instruction]);
   ```

3. **Use PDA helpers**:
   ```rust
   let (pda, bump) = get_pool_pda(&maker.pubkey(), seed);
   ```

## Troubleshooting

### Test Compilation Issues

- Ensure `Cargo.toml` has `litesvm` and related dependencies
- Check that instruction data types match your program's definitions

### Account Not Found

- Verify PDA calculations use correct seeds
- Ensure all required accounts are created before sending instructions

### Token Decimal Issues

- All tests use 6-decimal tokens (standard for SOL)
- Adjust `mint_token()` if different decimals needed

## Best Practices

1. ✅ **Use shared utilities** - Don't duplicate setup/helper code
2. ✅ **Clear test names** - Name tests to describe what they verify
3. ✅ **Independent tests** - Each test should work standalone
4. ✅ **Proper cleanup** - Tests using PDA helpers should use unique seeds
5. ✅ **Document complex scenarios** - Add comments for multi-step tests

## Future Test Coverage

Consider adding tests for:

- Error conditions (insufficient balance, invalid permissions)
- Edge cases (zero amounts, max values)
- Complex scenarios (multi-user interactions)
- Permissioning and authorization
- Token standard validation
