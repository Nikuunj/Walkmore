mod utils;
use anchor_lang::solana_program::system_program::ID as SYSTEM_PROGRAM_ID;
use litesvm_token::TOKEN_ID;
use solana_signer::Signer;
use utils::*;

#[test]
fn test_create_challenge_onevone() {
    let (mut svm, player1) = setup();
    let player2 = solana_keypair::Keypair::new();
    svm.airdrop(&player2.pubkey(), 10_000_000_000).unwrap();

    let mint = mint_token(&mut svm, &player1);
    let (challenge_pda, _challenge_bump) = get_challenge_pda(500);

    // Create ATA for player1
    let player1_ata = create_ata(&mut svm, &player1, &mint, &player1.pubkey());
    mint_to(&mut svm, &player1, &mint, &player1_ata, 1_000_000_000);

    // Derive vault ATA for the challenge PDA; it will be created by the program
    let vault_ata = get_ata(&challenge_pda, &mint);

    // Create user data PDAs
    let (player1_data_pda, _player1_data_bump) =
        get_challenge_user_data_pda(&player1.pubkey(), &challenge_pda);
    let (player2_data_pda, _player2_data_bump) =
        get_challenge_user_data_pda(&player2.pubkey(), &challenge_pda);

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
        500,
        &player2.pubkey(),
        10000,
        100_000,
    );

    send(&mut svm, &player1, &[create_challenge_ix]);
}

#[test]
fn test_accept_challenge() {
    let (mut svm, player1) = setup();
    let player2 = solana_keypair::Keypair::new();
    svm.airdrop(&player2.pubkey(), 10_000_000_000).unwrap();

    let mint = mint_token(&mut svm, &player1);
    let (challenge_pda, __challenge_bump) = get_challenge_pda(501);

    // Setup player1
    let player1_ata = create_ata(&mut svm, &player1, &mint, &player1.pubkey());
    mint_to(&mut svm, &player1, &mint, &player1_ata, 1_000_000_000);

    // Setup player2
    let player2_ata = create_ata(&mut svm, &player1, &mint, &player2.pubkey());
    mint_to(&mut svm, &player1, &mint, &player2_ata, 1_000_000_000);

    let vault_ata = get_ata(&challenge_pda, &mint);

    let (player1_data_pda, _player1_data_bump) =
        get_challenge_user_data_pda(&player1.pubkey(), &challenge_pda);
    let (player2_data_pda, __player2_data_bump) =
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
        501,
        &player2.pubkey(),
        10000,
        100_000,
    );
    send(&mut svm, &player1, &[create_challenge_ix]);

    // Player2 accepts challenge
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
}

#[test]
fn test_update_steps_challenge() {
    let (mut svm, player1) = setup();
    let player2 = solana_keypair::Keypair::new();
    svm.airdrop(&player2.pubkey(), 10_000_000_000).unwrap();

    let mint = mint_token(&mut svm, &player1);
    let (challenge_pda, __challenge_bump) = get_challenge_pda(502);

    // Setup tokens
    let player1_ata = create_ata(&mut svm, &player1, &mint, &player1.pubkey());
    mint_to(&mut svm, &player1, &mint, &player1_ata, 1_000_000_000);

    let player2_ata = create_ata(&mut svm, &player1, &mint, &player2.pubkey());
    mint_to(&mut svm, &player1, &mint, &player2_ata, 1_000_000_000);

    let vault_ata = get_ata(&challenge_pda, &mint);

    let (player1_data_pda, _player1_data_bump) =
        get_challenge_user_data_pda(&player1.pubkey(), &challenge_pda);
    let (player2_data_pda, __player2_data_bump) =
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
        502,
        &player2.pubkey(),
        10000,
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

    // Player1 updates steps
    let update_steps_ix = update_steps_challenge(&player1, &challenge_pda, &player1_data_pda, 2000);
    send(&mut svm, &player1, &[update_steps_ix]);
}

#[test]
fn test_reject_challenge() {
    let (mut svm, player1) = setup();
    let player2 = solana_keypair::Keypair::new();
    svm.airdrop(&player2.pubkey(), 10_000_000_000).unwrap();

    let mint = mint_token(&mut svm, &player1);
    let (challenge_pda, __challenge_bump) = get_challenge_pda(503);

    let player1_ata = create_ata(&mut svm, &player1, &mint, &player1.pubkey());
    mint_to(&mut svm, &player1, &mint, &player1_ata, 1_000_000_000);

    let player2_ata = create_ata(&mut svm, &player1, &mint, &player2.pubkey());
    mint_to(&mut svm, &player1, &mint, &player2_ata, 1_000_000_000);

    let vault_ata = get_ata(&challenge_pda, &mint);

    let (player1_data_pda, _player1_data_bump) =
        get_challenge_user_data_pda(&player1.pubkey(), &challenge_pda);
    let (player2_data_pda, __player2_data_bump) =
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
        503,
        &player2.pubkey(),
        10000,
        100_000,
    );
    send(&mut svm, &player1, &[create_challenge_ix]);

    // Player2 rejects challenge
    let reject_challenge_ix = reject_challenge(
        &player2,
        &player1,
        &mint,
        &challenge_pda,
        &vault_ata,
        &player1_ata,
        &anchor_spl::associated_token::ID,
        &TOKEN_ID,
        &SYSTEM_PROGRAM_ID,
    );
    send(&mut svm, &player2, &[reject_challenge_ix]);
}
