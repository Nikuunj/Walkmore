mod utils;
use anchor_lang::solana_program::system_program::ID as SYSTEM_PROGRAM_ID;
use litesvm_token::TOKEN_ID;
use solana_signer::Signer;
use utils::*;

#[test]
fn test_winner_withdraw_pool() {
    let (mut svm, maker) = setup();
    let user = solana_keypair::Keypair::new();
    svm.airdrop(&user.pubkey(), 10_000_000_000).unwrap();

    let mint = mint_token(&mut svm, &maker);
    let (pool_pda, _pool_bump) = get_pool_pda(&maker.pubkey(), 600);

    // Create pool
    let create_pool_ix = create_pool(
        &maker,
        &pool_pda,
        600,
        0,
        1_000_000,
        100_000,
        5000,
        &mint,
        &SYSTEM_PROGRAM_ID,
    );
    send(&mut svm, &maker, &[create_pool_ix]);

    // Setup user
    let user_ata = create_ata(&mut svm, &maker, &mint, &user.pubkey());
    mint_to(&mut svm, &maker, &mint, &user_ata, 1_000_000_000);

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

    // Create user data account
    let (user_data_pda, _user_data_bump) = get_user_data_pda(&user.pubkey(), &pool_pda);
    let create_user_account_ix =
        create_user_account(&user, &pool_pda, &user_data_pda, &SYSTEM_PROGRAM_ID);
    send(&mut svm, &user, &[create_user_account_ix]);

    // Update steps to reach target
    let update_steps_ix = update_steps_pool(&user, &pool_pda, &user_data_pda, 6000);
    send(&mut svm, &user, &[update_steps_ix]);

    // Withdraw as winner
    let withdraw_ix = winner_withdraw_pool(
        &user,
        &mint,
        &pool_pda,
        &user_data_pda,
        &user_ata,
        &pool_vault,
        &anchor_spl::associated_token::ID,
        &TOKEN_ID,
        &SYSTEM_PROGRAM_ID,
    );
    send(&mut svm, &user, &[withdraw_ix]);
}

#[test]
fn test_winner_withdraw_challenge() {
    let (mut svm, player1) = setup();
    let player2 = solana_keypair::Keypair::new();
    svm.airdrop(&player2.pubkey(), 10_000_000_000).unwrap();

    let mint = mint_token(&mut svm, &player1);
    let (challenge_pda, _challenge_bump) = get_challenge_pda(601);

    // Setup tokens
    let player1_ata = create_ata(&mut svm, &player1, &mint, &player1.pubkey());
    mint_to(&mut svm, &player1, &mint, &player1_ata, 1_000_000_000);

    let player2_ata = create_ata(&mut svm, &player1, &mint, &player2.pubkey());
    mint_to(&mut svm, &player1, &mint, &player2_ata, 1_000_000_000);

    let vault_ata = get_ata(&challenge_pda, &mint);

    let (player1_data_pda, _player1_data_bump) =
        get_challenge_user_data_pda(&player1.pubkey(), &challenge_pda);
    let (player2_data_pda, _player2_data_bump) =
        get_challenge_user_data_pda(&player2.pubkey(), &challenge_pda);

    // Create challenge
    let create_challenge_ix = create_challenge_onevone(
        &player1,
        &mint,
        &challenge_pda,
        &vault_ata,
        &player1_ata,
        &player1_data_pda,
        &player2_data_pda,
        &anchor_spl::associated_token::ID,
        &TOKEN_ID,
        &SYSTEM_PROGRAM_ID,
        601,
        &player2.pubkey(),
        0,
        100_000,
    );
    send(&mut svm, &player1, &[create_challenge_ix]);

    // Accept challenge
    let accept_challenge_ix = accept_challenge(
        &player2,
        &mint,
        &challenge_pda,
        &vault_ata,
        &player2_ata,
        &anchor_spl::associated_token::ID,
        &TOKEN_ID,
        &SYSTEM_PROGRAM_ID,
    );
    send(&mut svm, &player2, &[accept_challenge_ix]);

    // Players update steps - player1 wins
    let update_p1_ix = update_steps_challenge(&player1, &challenge_pda, &player1_data_pda, 5000);
    send(&mut svm, &player1, &[update_p1_ix]);

    let update_p2_ix = update_steps_challenge(&player2, &challenge_pda, &player2_data_pda, 3000);
    send(&mut svm, &player2, &[update_p2_ix]);

    // Player1 (winner) withdraws
    let withdraw_ix = winner_withdraw_onevone(
        &player1,
        &mint,
        &challenge_pda,
        &vault_ata,
        &player1_data_pda,
        &player2_data_pda,
        &player1_ata,
        &anchor_spl::associated_token::ID,
        &TOKEN_ID,
        &player1.pubkey(),
        &player2.pubkey(),
    );
    send(&mut svm, &player1, &[withdraw_ix]);
}

// #[test]
// fn test_close_challenge() {
//     let (mut svm, player1) = setup();
//     let player2 = solana_keypair::Keypair::new();
//     svm.airdrop(&player2.pubkey(), 10_000_000_000).unwrap();

//     let mint = mint_token(&mut svm, &player1);
//     let (challenge_pda, _challenge_bump) = get_challenge_pda(602);

//     let player1_ata = create_ata(&mut svm, &player1, &mint, &player1.pubkey());
//     mint_to(&mut svm, &player1, &mint, &player1_ata, 1_000_000_000);

//     let player2_ata = create_ata(&mut svm, &player1, &mint, &player2.pubkey());
//     mint_to(&mut svm, &player1, &mint, &player2_ata, 1_000_000_000);

//     let vault_ata = get_ata(&challenge_pda, &mint);

//     let (player1_data_pda, _player1_data_bump) =
//         get_challenge_user_data_pda(&player1.pubkey(), &challenge_pda);
//     let (player2_data_pda, _player2_data_bump) =
//         get_challenge_user_data_pda(&player2.pubkey(), &challenge_pda);

//     // Create challenge
//     let create_challenge_ix = create_challenge_onevone(
//         &player1,
//         &mint,
//         &challenge_pda,
//         &vault_ata,
//         &player1_ata,
//         &player1_data_pda,
//         &player2_data_pda,
//         &anchor_spl::associated_token::ID,
//         &TOKEN_ID,
//         &SYSTEM_PROGRAM_ID,
//         602,
//         &player2.pubkey(),
//         0,
//         100_000,
//     );
//     send(&mut svm, &player1, &[create_challenge_ix]);

//     // Accept challenge
//     let accept_challenge_ix = accept_challenge(
//         &player2,
//         &mint,
//         &challenge_pda,
//         &vault_ata,
//         &player2_ata,
//         &anchor_spl::associated_token::ID,
//         &TOKEN_ID,
//         &SYSTEM_PROGRAM_ID,
//     );
//     send(&mut svm, &player2, &[accept_challenge_ix]);

//     // Players update steps
//     let update_p1_ix = update_steps_challenge(&player1, &challenge_pda, &player1_data_pda, 5000);
//     send(&mut svm, &player1, &[update_p1_ix]);

//     let update_p2_ix = update_steps_challenge(&player2, &challenge_pda, &player2_data_pda, 5000);
//     send(&mut svm, &player2, &[update_p2_ix]);

//     // Close challenge
//     let close_challenge_ix = close_challenge(
//         &player1,
//         &player2.pubkey(),
//         &mint,
//         &challenge_pda,
//         &vault_ata,
//         &player1_data_pda,
//         &player2_data_pda,
//         &anchor_spl::associated_token::ID,
//         &TOKEN_ID,
//     );
//     send(&mut svm, &player1, &[close_challenge_ix]);
// }
