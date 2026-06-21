use anchor_lang::{InstructionData, ToAccountMetas, solana_program::{instruction::Instruction, pubkey::Pubkey}};
use anchor_spl::associated_token;
use litesvm::LiteSVM;
use litesvm_token::{CreateAssociatedTokenAccount, CreateMint, MintTo};
use solana_keypair::Keypair;
use solana_message::Message;
use solana_signer::Signer;
use solana_transaction::Transaction;

// ============================================================================
// Setup
// ============================================================================

pub fn setup() -> (LiteSVM, Keypair) {
    let program_id = walkmore::id();
    let payer = Keypair::new();
    let mut svm = LiteSVM::new();

    let bytes = include_bytes!("../../../target/deploy/walkmore.so");
    svm.add_program(program_id, bytes).unwrap();
    svm.airdrop(&payer.pubkey(), 10_000_000_000).unwrap();
    (svm, payer)
}

// ============================================================================
// Token Helpers
// ============================================================================

pub fn mint_token(svm: &mut LiteSVM, payer: &Keypair) -> Pubkey {
    CreateMint::new(svm, payer)
        .decimals(6)
        .authority(&payer.pubkey())
        .send()
        .unwrap()
}

pub fn get_ata(owner: &Pubkey, mint: &Pubkey) -> Pubkey {
    associated_token::get_associated_token_address(owner, mint)
}

pub fn create_ata(svm: &mut LiteSVM, payer: &Keypair, mint: &Pubkey, owner: &Pubkey) -> Pubkey {
    CreateAssociatedTokenAccount::new(svm, payer, mint)
        .owner(owner)
        .send()
        .unwrap()
}

pub fn mint_to(svm: &mut LiteSVM, payer: &Keypair, mint: &Pubkey, ata: &Pubkey, amount: u64) {
    MintTo::new(svm, payer, mint, ata, amount).send().unwrap();
}

// ============================================================================
// PDA Helpers
// ============================================================================

pub fn get_pool_pda(maker: &Pubkey, seed: u128) -> (Pubkey, u8) {
    Pubkey::find_program_address(
        &[b"pool", maker.as_ref(), seed.to_le_bytes().as_ref()],
        &walkmore::id(),
    )
}

pub fn get_user_data_pda(user: &Pubkey, pool: &Pubkey) -> (Pubkey, u8) {
    Pubkey::find_program_address(
        &[b"user_data", user.as_ref(), pool.as_ref()],
        &walkmore::id(),
    )
}

pub fn get_challenge_pda(seed: u64) -> (Pubkey, u8) {
    Pubkey::find_program_address(
        &[b"challenge", seed.to_le_bytes().as_ref()],
        &walkmore::id(),
    )
}

pub fn get_challenge_user_data_pda(user: &Pubkey, challenge: &Pubkey) -> (Pubkey, u8) {
    Pubkey::find_program_address(
        &[b"user_data", user.as_ref(), challenge.as_ref()],
        &walkmore::id(),
    )
}

// ============================================================================
// Transaction Execution
// ============================================================================

pub fn send(svm: &mut LiteSVM, payer: &Keypair, instructions: &[Instruction]) {
    let msg = Message::new(instructions, Some(&payer.pubkey()));
    let recent_blockhash = svm.latest_blockhash();
    let tx = Transaction::new(&[payer], msg, recent_blockhash);
    svm.send_transaction(tx).unwrap();
}

pub fn send_expect_error(svm: &mut LiteSVM, payer: &Keypair, instructions: &[Instruction]) -> bool {
    let msg = Message::new(instructions, Some(&payer.pubkey()));
    let recent_blockhash = svm.latest_blockhash();
    let tx = Transaction::new(&[payer], msg, recent_blockhash);
    svm.send_transaction(tx).is_err()
}

// ============================================================================
// Instruction Builders
// ============================================================================

pub fn initialize() -> Instruction {
    Instruction {
        program_id: walkmore::id(),
        accounts: walkmore::accounts::Initialize {}.to_account_metas(None),
        data: walkmore::instruction::Initialize {}.data(),
    }
}

pub fn create_pool(
    maker: &Keypair,
    pool: &Pubkey,
    seed: u128,
    start_time: u64,
    end_time: u64,
    entry_fee: u64,
    target: u32,
    mint: &Pubkey,
    system_program: &Pubkey,
) -> Instruction {
    Instruction {
        program_id: walkmore::id(),
        accounts: walkmore::accounts::CreatePool {
            maker: maker.pubkey(),
            pool: *pool,
            system_program: *system_program,
        }
        .to_account_metas(None),
        data: walkmore::instruction::CreatePool {
            seed,
            start_time,
            end_time,
            entry_fee,
            target,
            mint: *mint
        }
        .data(),
    }
}

pub fn create_user_account(
    user: &Keypair,
    pool: &Pubkey,
    user_data: &Pubkey,
    system_program: &Pubkey,
) -> Instruction {
    Instruction {
        program_id: walkmore::id(),
        accounts: walkmore::accounts::CreateUserDataAcc {
            user: user.pubkey(),
            pool: *pool,
            user_data: *user_data,
            system_program: *system_program,
        }
        .to_account_metas(None),
        data: walkmore::instruction::CreateUserAccount {}.data(),
    }
}

pub fn join_pool(
    user: &Keypair,
    mint: &Pubkey,
    pool: &Pubkey,
    user_ata: &Pubkey,
    pool_vault: &Pubkey,
    associated_token_program: &Pubkey,
    token_program: &Pubkey,
    system_program: &Pubkey,
) -> Instruction {
    Instruction {
        program_id: walkmore::id(),
        accounts: walkmore::accounts::JoinPool {
            user: user.pubkey(),
            mint: *mint,
            pool: *pool,
            user_ata: *user_ata,
            pool_vault: *pool_vault,
            associated_token_program: *associated_token_program,
            token_program: *token_program,
            system_program: *system_program,
        }
        .to_account_metas(None),
        data: walkmore::instruction::JoinPool {}.data(),
    }
}

pub fn create_challenge_onevone(
    creator: &Keypair,
    mint: &Pubkey,
    challenge: &Pubkey,
    vault_ata: &Pubkey,
    player1_ata: &Pubkey,
    player1_data: &Pubkey,
    player2_data: &Pubkey,
    associated_token_program: &Pubkey,
    token_program: &Pubkey,
    system_program: &Pubkey,
    seed: u64,
    player2: &Pubkey,
    duration: u64,
    fee: u64,
) -> Instruction {
    Instruction {
        program_id: walkmore::id(),
        accounts: walkmore::accounts::CreateChallengeOneVOne {
            creator: creator.pubkey(),
            mint: *mint,
            challenge: *challenge,
            vault_ata: *vault_ata,
            player1_ata: *player1_ata,
            player1_data: *player1_data,
            player2_data: *player2_data,
            associated_token_program: *associated_token_program,
            token_program: *token_program,
            system_program: *system_program,
        }
        .to_account_metas(None),
        data: walkmore::instruction::CreateChallengeOnevone {
            seed,
            player2: *player2,
            duration,
            fee,
        }
        .data(),
    }
}

pub fn create_challenge_deposit(
    creator: &Keypair,
    mint: &Pubkey,
    challenge: &Pubkey,
    vault_ata: &Pubkey,
    player1_ata: &Pubkey,
    player1_data: &Pubkey,
    player2_data: &Pubkey,
    associated_token_program: &Pubkey,
    token_program: &Pubkey,
    system_program: &Pubkey,
    seed: u64,
    player2: &Pubkey,
    fee: u64,
) -> Instruction {
    Instruction {
        program_id: walkmore::id(),
        accounts: walkmore::accounts::CreateChallengeOneVOne {
            creator: creator.pubkey(),
            mint: *mint,
            challenge: *challenge,
            vault_ata: *vault_ata,
            player1_ata: *player1_ata,
            player1_data: *player1_data,
            player2_data: *player2_data,
            associated_token_program: *associated_token_program,
            token_program: *token_program,
            system_program: *system_program,
        }
        .to_account_metas(None),
        data: walkmore::instruction::CreateChallengeOnevone {
            seed,
            player2: *player2,
            duration: (10000 * 1000) / 400,
            fee,
        }
        .data(),
    }
}

pub fn accept_challenge(
    player2: &Keypair,
    mint: &Pubkey,
    challenge: &Pubkey,
    vault_ata: &Pubkey,
    player2_ata: &Pubkey,
    associated_token_program: &Pubkey,
    token_program: &Pubkey,
    system_program: &Pubkey,
) -> Instruction {
    Instruction {
        program_id: walkmore::id(),
        accounts: walkmore::accounts::AcceptChallengeOneVOne {
            player2: player2.pubkey(),
            mint: *mint,
            challenge: *challenge,
            vault_ata: *vault_ata,
            player2_ata: *player2_ata,
            associated_token_program: *associated_token_program,
            token_program: *token_program,
            system_program: *system_program,
        }
        .to_account_metas(None),
        data: walkmore::instruction::AcceptChallenge {}.data(),
    }
}

pub fn update_steps_challenge(
    user: &Keypair,
    challenge: &Pubkey,
    user_data: &Pubkey,
    steps: u32,
) -> Instruction {
    Instruction {
        program_id: walkmore::id(),
        accounts: walkmore::accounts::UpdateStepsChallenege {
            user: user.pubkey(),
            challenge: *challenge,
            user_data: *user_data,
        }
        .to_account_metas(None),
        data: walkmore::instruction::UpdateStepsChallenge { steps }.data(),
    }
}

pub fn update_steps_pool(
    user: &Keypair,
    pool: &Pubkey,
    user_data: &Pubkey,
    steps: u32,
) -> Instruction {
    Instruction {
        program_id: walkmore::id(),
        accounts: walkmore::accounts::UpdateStepsPool {
            user: user.pubkey(),
            challenge: *pool,
            user_data: *user_data,
        }
        .to_account_metas(None),
        data: walkmore::instruction::UpdateStepsPool { steps }.data(),
    }
}

pub fn winner_withdraw_pool(
    user: &Keypair,
    mint: &Pubkey,
    pool: &Pubkey,
    user_data: &Pubkey,
    user_ata: &Pubkey,
    pool_vault: &Pubkey,
    associated_token_program: &Pubkey,
    token_program: &Pubkey,
    system_program: &Pubkey,
) -> Instruction {
    Instruction {
        program_id: walkmore::id(),
        accounts: walkmore::accounts::WinnerWithdrawPool {
            user: user.pubkey(),
            mint: *mint,
            pool: *pool,
            user_data: *user_data,
            user_ata: *user_ata,
            pool_vault: *pool_vault,
            associated_token_program: *associated_token_program,
            token_program: *token_program,
            system_program: *system_program,
        }
        .to_account_metas(None),
        data: walkmore::instruction::WinnerWithdrawPool {}.data(),
    }
}

pub fn winner_withdraw_onevone(
    winner: &Keypair,
    mint: &Pubkey,
    challenge: &Pubkey,
    vault_ata: &Pubkey,
    player1_data: &Pubkey,
    player2_data: &Pubkey,
    winner_ata: &Pubkey,
    associated_token_program: &Pubkey,
    token_program: &Pubkey,
    player1: &Pubkey,
    player2: &Pubkey,
) -> Instruction {
    Instruction {
        program_id: walkmore::id(),
        accounts: walkmore::accounts::WinnerWithdrawOneVOne {
            winner: winner.pubkey(),
            mint: *mint,
            challenge: *challenge,
            vault_ata: *vault_ata,
            player1_data: *player1_data,
            player2_data: *player2_data,
            winner_ata: *winner_ata,
            associated_token_program: *associated_token_program,
            token_program: *token_program,
        }
        .to_account_metas(None),
        data: walkmore::instruction::WinnerWithdrawOnevone {
            player1: *player1,
            player2: *player2,
        }
        .data(),
    }
}

pub fn reject_challenge(
    player2: &Keypair,
    player1: &Keypair,
    mint: &Pubkey,
    challenge: &Pubkey,
    vault_ata: &Pubkey,
    player1_ata: &Pubkey,
    associated_token_program: &Pubkey,
    token_program: &Pubkey,
    system_program: &Pubkey,
) -> Instruction {
    Instruction {
        program_id: walkmore::id(),
        accounts: walkmore::accounts::RejectChallenge {
            player2: player2.pubkey(),
            player1: player1.pubkey(),
            mint: *mint,
            challenge: *challenge,
            vault_ata: *vault_ata,
            player1_ata: *player1_ata,
            associated_token_program: *associated_token_program,
            token_program: *token_program,
            system_program: *system_program,
        }
        .to_account_metas(None),
        data: walkmore::instruction::RejectChallenge {}.data(),
    }
}

pub fn close_challenge(
    player1: &Keypair,
    player2: &Pubkey,
    mint: &Pubkey,
    challenge: &Pubkey,
    vault_ata: &Pubkey,
    player1_data: &Pubkey,
    player2_data: &Pubkey,
    associated_token_program: &Pubkey,
    token_program: &Pubkey,
) -> Instruction {
    Instruction {
        program_id: walkmore::id(),
        accounts: walkmore::accounts::CloseChallenge {
            player1: player1.pubkey(),
            player2: *player2,
            mint: *mint,
            challenge: *challenge,
            vault_ata: *vault_ata,
            player1_data: *player1_data,
            player2_data: *player2_data,
            associated_token_program: *associated_token_program,
            token_program: *token_program,
        }
        .to_account_metas(None),
        data: walkmore::instruction::CloseChallenge {}.data(),
    }
}
