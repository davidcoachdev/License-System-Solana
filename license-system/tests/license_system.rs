use anchor_lang::prelude::*;
use license_system::{License, ErrorCode};
use mollusk_svm::Mollusk;

fn init_mollusk() -> Mollusk {
    let program_id = license_system::ID;
    Mollusk::new(&program_id, "target/deploy/license_system.so")
}

#[test]
fn test_initialization() {
    let mollusk = init_mollusk();
    let payer = mollusk.gen_payer();
    mollusk.airdrop(&payer, 10_000_000_000).unwrap();
    mollusk.process_and_validate_instruction(
        &anchor_lang::solana_program::Instruction {
            program_id: license_system::ID,
            accounts: vec![],
            data: vec![],
        },
        &[&payer],
        &[mollusk_svm::Check::success()],
    );
}