
use {
    anchor_lang::{solana_program::instruction::Instruction, InstructionData, ToAccountMetas},
    litesvm::LiteSVM,
    solana_message::{Message, VersionedMessage},
    solana_signer::Signer,
    solana_keypair::Keypair,
    solana_transaction::versioned::VersionedTransaction,
};

#[test]
fn test_initialize() {
    let program_id = walkmore::id();
    let payer = Keypair::new();
    let mut svm = LiteSVM::new();
    // let bytes = include_bytes!("../../../target/deploy/walkmore.so");
    // svm.add_program(program_id, bytes).unwrap();
    // svm.airdrop(&payer.pubkey(), 1_000_000_000).unwrap();
}
