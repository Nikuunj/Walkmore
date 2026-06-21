use anchor_lang::system_program::ID as SYSTEM_PROGRAM_ID;
use litesvm_token::TOKEN_ID;
use solana_signer::Signer;

mod utils;
use utils::*;

#[test]
fn test_create_pool() {
    let (mut svm, maker) = setup();
    let mint = mint_token(&mut svm, &maker);
    let (pool_pda, _pool_bump) = get_pool_pda(&maker.pubkey(), 100);

    let create_pool_ix = create_pool(
        &maker,
        &pool_pda,
        100,
        0,
        1000,
        100_000,
        5000,
        &mint,
        &SYSTEM_PROGRAM_ID,
    );

    send(&mut svm, &maker, &[create_pool_ix]);
}

#[test]
fn test_create_pool_and_join() {
    let (mut svm, maker) = setup();
    let user = solana_keypair::Keypair::new();
    svm.airdrop(&user.pubkey(), 10_000_000_000).unwrap();

    let mint = mint_token(&mut svm, &maker);
    let (pool_pda, _pool_bump) = get_pool_pda(&maker.pubkey(), 200);

    // Create pool
    let create_pool_ix = create_pool(
        &maker,
        &pool_pda,
        200,
        0,
        1000,
        100_000,
        5000,
        &mint,
        &SYSTEM_PROGRAM_ID,
    );
    send(&mut svm, &maker, &[create_pool_ix]);

    // Mint tokens to user
    let user_ata = create_ata(&mut svm, &maker, &mint, &user.pubkey());
    mint_to(&mut svm, &maker, &mint, &user_ata, 1_000_000_000);

    // Create pool vault
    let pool_vault = create_ata(&mut svm, &maker, &mint, &pool_pda);

    // User joins pool
    let join_pool_ix = join_pool(
        &user,
        &mint,
        &pool_pda,
        &user_ata,
        &pool_vault,
        &anchor_spl::associated_token::ID,
        &TOKEN_ID,
        &SYSTEM_PROGRAM_ID,
    );
    send(&mut svm, &user, &[join_pool_ix]);
}

#[test]
fn test_create_user_account_for_pool() {
    let (mut svm, maker) = setup();
    let user = solana_keypair::Keypair::new();
    svm.airdrop(&user.pubkey(), 10_000_000_000).unwrap();

    let mint = mint_token(&mut svm, &maker);
    let (pool_pda, _pool_bump) = get_pool_pda(&maker.pubkey(), 300);

    // Create pool
    let create_pool_ix = create_pool(
        &maker,
        &pool_pda,
        300,
        0,
        1000,
        100_000,
        5000,
        &mint,
        &SYSTEM_PROGRAM_ID,
    );
    send(&mut svm, &maker, &[create_pool_ix]);

    // Create user account for pool
    let (user_data_pda, _user_data_bump) = get_user_data_pda(&user.pubkey(), &pool_pda);

    let create_user_account_ix =
        create_user_account(&user, &pool_pda, &user_data_pda, &SYSTEM_PROGRAM_ID);
    send(&mut svm, &user, &[create_user_account_ix]);
}

#[test]
fn test_update_steps_pool() {
    let (mut svm, maker) = setup();
    let user = solana_keypair::Keypair::new();
    svm.airdrop(&user.pubkey(), 10_000_000_000).unwrap();

    let mint = mint_token(&mut svm, &maker);
    let (pool_pda, _pool_bump) = get_pool_pda(&maker.pubkey(), 400);

    // Create pool
    let create_pool_ix = create_pool(
        &maker,
        &pool_pda,
        400,
        0,
        1000,
        100_000,
        5000,
        &mint,
        &SYSTEM_PROGRAM_ID,
    );
    send(&mut svm, &maker, &[create_pool_ix]);

    // Create user account
    let (user_data_pda, _user_data_bump) = get_user_data_pda(&user.pubkey(), &pool_pda);
    let create_user_account_ix =
        create_user_account(&user, &pool_pda, &user_data_pda, &SYSTEM_PROGRAM_ID);
    send(&mut svm, &user, &[create_user_account_ix]);

    // Update steps
    let update_steps_ix = update_steps_pool(&user, &pool_pda, &user_data_pda, 1000);
    send(&mut svm, &user, &[update_steps_ix]);
}
