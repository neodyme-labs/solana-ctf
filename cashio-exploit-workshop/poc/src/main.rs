use anchor_lang::{prelude::*, system_program, InstructionData};
use arrow_sunny::{Arrow, ArrowMiner};
use bankman::CASH_DECIMALS;
use cashio_poc::{AccountConfig, LocalEnv, LocalEnvBuilder};
use solana_program::{
    instruction::Instruction, native_token::sol_to_lamports, program_pack::Pack, pubkey::Pubkey,
    system_instruction,
};
use solana_program_test::tokio;
use solana_sdk::{signature::Keypair, signer::Signer};
use stable_swap_client::{fees::Fees, state::SwapInfo};

type Result<T, E> = std::result::Result<T, E>;

struct FakeArrow {
    #[allow(dead_code)]
    dummy: Pubkey,
    arrow_addr: Keypair,
    arrow_data: Vec<u8>,
}

/// We provide an API to create a fake Arrow because faking it is technically
/// not difficult (only two trivial fields are checked in the struct) but can be
/// time consuming.
///
/// Feel free to create your own fake arrow account for an extra credit :)
fn create_fake_arrow(token_mint: Pubkey) -> FakeArrow {
    let dummy = Keypair::new();
    let arrow_addr = Keypair::new();

    let vendor_miner = ArrowMiner {
        mint: token_mint,
        rewarder: dummy.pubkey(),
        quarry: dummy.pubkey(),
        miner: dummy.pubkey(),
        miner_vault: dummy.pubkey(),
        rewards_mint: dummy.pubkey(),
        mint_wrapper: dummy.pubkey(),
        claim_fee_token_account: dummy.pubkey(),
        vault_staked_token_account: dummy.pubkey(),
        vault_rewards_token_account: dummy.pubkey(),
        sunny_pool_rewards_fee_account: dummy.pubkey(),
    };

    let internal_miner = ArrowMiner {
        mint: dummy.pubkey(),
        rewarder: dummy.pubkey(),
        quarry: dummy.pubkey(),
        miner: dummy.pubkey(),
        miner_vault: dummy.pubkey(),
        rewards_mint: dummy.pubkey(),
        mint_wrapper: dummy.pubkey(),
        claim_fee_token_account: dummy.pubkey(),
        vault_staked_token_account: dummy.pubkey(),
        vault_rewards_token_account: dummy.pubkey(),
        sunny_pool_rewards_fee_account: dummy.pubkey(),
    };

    let arrow = Arrow {
        mint: token_mint,
        bump: 0,
        beneficiary: dummy.pubkey(),
        pool: dummy.pubkey(),
        vault: dummy.pubkey(),
        vendor_miner: vendor_miner,
        internal_miner: internal_miner,
    };

    let mut arrow_data = Vec::new();
    arrow.try_serialize(&mut arrow_data).unwrap();

    FakeArrow {
        dummy: dummy.pubkey(),
        arrow_addr,
        arrow_data,
    }
}

const SOLVE_AMOUNT: u64 = 10_000_000_000;

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    // prepare mock environment
    let bankman_data = std::fs::read("cashio/target/deploy/bankman.so")?;
    let brrr_data = std::fs::read("cashio/target/deploy/brrr.so")?;
    let crate_token_data = std::fs::read("dep-programs/crate-token-0.6.0.so")?;
    let arrow_sunny_data = std::fs::read("dep-programs/arrow_sunny-0.3.1.so")?;
    let stable_swap_data = std::fs::read("dep-programs/stable-swap.so")?;

    let stable_token_authority = Keypair::new();
    let stable_a_mint = Keypair::new();
    let stable_b_mint = Keypair::new();

    let saber_authority = Keypair::new();
    let saber_lp_mint = Keypair::new();
    let saber_swap_info = Keypair::new();

    let cashio_authority = Keypair::new();
    let cashio_mint = Keypair::new();

    let fake_arrow = create_fake_arrow(saber_lp_mint.pubkey());

    let hacker = Keypair::new();
    let innocent = Keypair::new();

    // Fake Arrow pool is provided
    let hacker_saber_lp_mint = Keypair::new();
    let hacker_fake_arrow = create_fake_arrow(hacker_saber_lp_mint.pubkey());

    let mut env = LocalEnvBuilder::new()
        .add_program(bankman::ID, bankman_data)
        .add_program(brrr::ID, brrr_data)
        .add_program(crate_token::ID, crate_token_data)
        .add_program(arrow_sunny::ID, arrow_sunny_data)
        .add_program(stable_swap_client::ID, stable_swap_data)
        .add_account(
            fake_arrow.arrow_addr.pubkey(),
            AccountConfig {
                data: fake_arrow.arrow_data,
                owner: arrow_sunny::ID,
                ..Default::default()
            },
        )
        .add_account(
            hacker_fake_arrow.arrow_addr.pubkey(),
            AccountConfig {
                data: hacker_fake_arrow.arrow_data,
                owner: arrow_sunny::ID,
                ..Default::default()
            },
        )
        .add_account(
            cashio_authority.pubkey(),
            AccountConfig {
                lamports: Some(sol_to_lamports(1_000_000.0)),
                ..Default::default()
            },
        )
        .add_account(
            hacker.pubkey(),
            AccountConfig {
                lamports: Some(sol_to_lamports(1.0)),
                ..Default::default()
            },
        )
        .add_account(
            innocent.pubkey(),
            AccountConfig {
                lamports: Some(sol_to_lamports(100.0)),
                ..Default::default()
            },
        )
        .build()
        .await;

    println!("Step 1: Prepare Saber Swap and LP token");

    env.create_token_mint(
        &stable_a_mint,
        stable_token_authority.pubkey(),
        None,
        CASH_DECIMALS,
    )
    .await?;
    env.create_token_mint(
        &stable_b_mint,
        stable_token_authority.pubkey(),
        None,
        CASH_DECIMALS,
    )
    .await?;

    let (swap_authority, swap_authority_nonce) = Pubkey::find_program_address(
        &[&saber_swap_info.pubkey().to_bytes()[..32]],
        &stable_swap_client::ID,
    );

    env.create_token_mint(&saber_lp_mint, swap_authority, None, CASH_DECIMALS)
        .await?;

    let swap_stable_a_wallet = env
        .create_associated_token_account(swap_authority, stable_a_mint.pubkey())
        .await?;
    let swap_stable_b_wallet = env
        .create_associated_token_account(swap_authority, stable_b_mint.pubkey())
        .await?;

    env.mint_tokens(
        stable_a_mint.pubkey(),
        &stable_token_authority,
        swap_stable_a_wallet,
        100_000_000_000_000,
    )
    .await?;
    env.mint_tokens(
        stable_b_mint.pubkey(),
        &stable_token_authority,
        swap_stable_b_wallet,
        100_000_000_000_000,
    )
    .await?;

    // The innocent receives initial LP tokens.
    // In more realistic situation, the innocent would deposit their stable token to receive it.
    let innocent_saber_wallet = env
        .create_associated_token_account(innocent.pubkey(), saber_lp_mint.pubkey())
        .await?;

    let swap_info_exemption_amount = env.rent_exemption_amount(SwapInfo::LEN).await?;
    env.run_instructions(
        &[
            system_instruction::create_account(
                &env.payer().pubkey(),
                &saber_swap_info.pubkey(),
                swap_info_exemption_amount,
                SwapInfo::LEN as u64,
                &stable_swap_client::ID,
            ),
            stable_swap_client::instruction::initialize(
                &spl_token::ID,
                &saber_swap_info.pubkey(),
                &swap_authority,
                &saber_authority.pubkey(),
                &swap_stable_a_wallet,
                &swap_stable_b_wallet,
                &stable_a_mint.pubkey(),
                &swap_stable_a_wallet,
                &stable_b_mint.pubkey(),
                &swap_stable_b_wallet,
                &saber_lp_mint.pubkey(),
                &innocent_saber_wallet,
                swap_authority_nonce,
                100,
                Fees {
                    admin_trade_fee_numerator: 0,
                    admin_trade_fee_denominator: 1,
                    admin_withdraw_fee_numerator: 0,
                    admin_withdraw_fee_denominator: 1,
                    trade_fee_numerator: 0,
                    trade_fee_denominator: 1,
                    withdraw_fee_numerator: 0,
                    withdraw_fee_denominator: 1,
                },
            )?,
        ],
        &[&saber_swap_info],
    )
    .await?;

    println!("Step 2: Prepare Cash token");
    let (cashio_crate_token, cashio_crate_token_bump) = Pubkey::find_program_address(
        &[b"CrateToken".as_ref(), &cashio_mint.pubkey().to_bytes()],
        &crate_token::ID,
    );

    let (bank, bank_bump) = Pubkey::find_program_address(
        &[b"Bank".as_ref(), &cashio_crate_token.to_bytes()],
        &bankman::ID,
    );

    env.create_token_mint(
        &cashio_mint,
        cashio_crate_token,
        Some(cashio_crate_token),
        bankman::CASH_DECIMALS,
    )
    .await?;

    println!("Step 3: Initialize a new bank");
    env.run_instruction(
        Instruction {
            program_id: bankman::ID,
            accounts: bankman::accounts::NewBank {
                bank,
                crate_mint: cashio_mint.pubkey(),
                crate_token: cashio_crate_token,
                brrr_issue_authority: brrr::ISSUE_AUTHORITY_ADDRESS,
                burn_withdraw_authority: brrr::WITHDRAW_AUTHORITY_ADDRESS,
                payer: cashio_authority.pubkey(),
                admin: cashio_authority.pubkey(),
                system_program: system_program::ID,
                crate_token_program: crate_token::ID,
            }
            .to_account_metas(None),
            data: bankman::instruction::NewBank {
                _bank_bump: bank_bump,
                crate_bump: cashio_crate_token_bump,
            }
            .data(),
        },
        &[&cashio_authority],
    )
    .await?;

    println!("Step 4: Add the saber LP token as a collateral");
    let (cashio_collateral, cashio_collateral_bump) = Pubkey::find_program_address(
        &[
            b"Collateral".as_ref(),
            &bank.to_bytes(),
            &saber_lp_mint.pubkey().to_bytes(),
        ],
        &bankman::ID,
    );

    env.run_instruction(
        Instruction {
            program_id: bankman::ID,
            accounts: bankman::accounts::AuthorizeCollateral {
                bank,
                collateral: cashio_collateral,
                mint: saber_lp_mint.pubkey(),
                curator: cashio_authority.pubkey(),
                payer: cashio_authority.pubkey(),
                system_program: system_program::ID,
            }
            .to_account_metas(None),
            data: bankman::instruction::AuthorizeCollateral {
                _bump: cashio_collateral_bump,
            }
            .data(),
        },
        &[&cashio_authority],
    )
    .await?;

    println!("Step 5: Set Collateral hard cap");
    env.run_instruction(
        Instruction {
            program_id: bankman::ID,
            accounts: bankman::accounts::SetCollateralHardCap {
                bank,
                collateral: cashio_collateral,
                curator: cashio_authority.pubkey(),
            }
            .to_account_metas(None),
            data: bankman::instruction::SetCollateralHardCap { hard_cap: u64::MAX }.data(),
        },
        &[&cashio_authority],
    )
    .await?;

    println!("Step 6: The innocent prints $CASH");
    let cashio_collateral_wallet = env
        .create_associated_token_account(cashio_crate_token, saber_lp_mint.pubkey())
        .await?;

    let innocent_cash_wallet = env
        .create_associated_token_account(innocent.pubkey(), cashio_mint.pubkey())
        .await?;

    env.run_instruction(
        Instruction {
            program_id: brrr::ID,
            accounts: brrr_shim::accounts::PrintCash {
                common: brrr_shim::accounts::BrrrCommon {
                    bank,
                    collateral: cashio_collateral,
                    crate_token: cashio_crate_token,
                    crate_mint: cashio_mint.pubkey(),
                    crate_collateral_tokens: cashio_collateral_wallet,
                    saber_swap: brrr_shim::accounts::SaberSwapAccounts {
                        arrow: fake_arrow.arrow_addr.pubkey(),
                        saber_swap: saber_swap_info.pubkey(),
                        pool_mint: saber_lp_mint.pubkey(),
                        reserve_a: swap_stable_a_wallet,
                        reserve_b: swap_stable_b_wallet,
                    },
                    token_program: spl_token::ID,
                    crate_token_program: crate_token::ID,
                },
                depositor: innocent.pubkey(),
                depositor_source: innocent_saber_wallet,
                mint_destination: innocent_cash_wallet,
                issue_authority: brrr::ISSUE_AUTHORITY_ADDRESS,
            }
            .to_account_metas(None),
            data: brrr::instruction::PrintCash {
                deposit_amount: 100_000_000_000,
            }
            .data(),
        },
        &[&innocent],
    )
    .await?;

    println!("Step 7: prepare hackers' wallets");
    let hacker_saber_wallet = env
        .create_associated_token_account(hacker.pubkey(), saber_lp_mint.pubkey())
        .await?;

    let hacker_cash_wallet = env
        .create_associated_token_account(hacker.pubkey(), cashio_mint.pubkey())
        .await?;

    println!("Successfully initialized!");

    println!("==============================");
    println!("[Before Hacking]");
    println!(
        "Innocent's $CASH: {}",
        env.token_balance(innocent_cash_wallet).await?
    );
    println!(
        "Saber LP in bank: {}",
        env.token_balance(cashio_collateral_wallet).await?
    );

    println!(
        "hacker's $CASH: {}",
        env.token_balance(hacker_cash_wallet).await?
    );
    println!(
        "hacker's Saber LP: {}",
        env.token_balance(hacker_saber_wallet).await?
    );
    println!("==============================");

    // execute PoC
    let hacker_info = HackerInfo {
        hacker,

        hacker_saber_lp_mint,
        hacker_fake_arrow_addr: hacker_fake_arrow.arrow_addr.pubkey(),

        hacker_saber_wallet,
        hacker_cash_wallet,

        stable_token_authority: stable_token_authority.pubkey(),
        stable_a_mint: stable_a_mint.pubkey(),
        stable_b_mint: stable_b_mint.pubkey(),

        saber_authority: saber_authority.pubkey(),
        saber_lp_mint: saber_lp_mint.pubkey(),
        saber_swap_info: saber_swap_info.pubkey(),
        saber_swap_stable_a_wallet: swap_stable_a_wallet,
        saber_swap_stable_b_wallet: swap_stable_b_wallet,

        cashio_authority: cashio_authority.pubkey(),
        cashio_mint: cashio_mint.pubkey(),

        cashio_crate_token,
        cashio_bank: bank,

        cashio_collateral,
        cashio_collateral_wallet,

        innocent_cash_wallet,

        fake_arrow_addr: fake_arrow.arrow_addr.pubkey(),
    };

    execute_poc(&mut env, &hacker_info).await?;

    // validate PoC
    println!("==============================");
    println!("[After Hacking]");
    println!(
        "Innocent's $CASH: {}",
        env.token_balance(innocent_cash_wallet).await?
    );
    println!(
        "Saber LP in bank: {}",
        env.token_balance(cashio_collateral_wallet).await?
    );

    let hacker_cash_tokens = env.token_balance(hacker_cash_wallet).await?;
    let hacker_lp_tokens = env.token_balance(hacker_saber_wallet).await?;

    print!(
        "Hacker's $CASH: {} >= {}? - ",
        hacker_cash_tokens, SOLVE_AMOUNT
    );
    if hacker_cash_tokens >= SOLVE_AMOUNT {
        println!("PASS");
    } else {
        println!("FAIL");
    }

    print!(
        "Hacker's Saber LP: {} >= {}? - ",
        hacker_lp_tokens, SOLVE_AMOUNT,
    );
    if hacker_lp_tokens >= SOLVE_AMOUNT {
        println!("PASS");
    } else {
        println!("FAIL");
    }
    println!("==============================");

    Ok(())
}

#[allow(dead_code)]
struct HackerInfo {
    hacker: Keypair,

    // We provide an already-faked Arrow account
    hacker_saber_lp_mint: Keypair,
    hacker_fake_arrow_addr: Pubkey,

    hacker_cash_wallet: Pubkey,
    hacker_saber_wallet: Pubkey,

    stable_token_authority: Pubkey,
    stable_a_mint: Pubkey,
    stable_b_mint: Pubkey,

    saber_authority: Pubkey,
    saber_lp_mint: Pubkey,
    saber_swap_info: Pubkey,
    saber_swap_stable_a_wallet: Pubkey,
    saber_swap_stable_b_wallet: Pubkey,

    cashio_authority: Pubkey,
    cashio_mint: Pubkey,

    cashio_crate_token: Pubkey,
    cashio_bank: Pubkey,

    cashio_collateral: Pubkey,
    cashio_collateral_wallet: Pubkey,

    innocent_cash_wallet: Pubkey,

    fake_arrow_addr: Pubkey,
}

/// Write a PoC code so that hacker's cash wallet and saber LP wallet
/// both hold at least SOLVE_AMOUNT of tokens.
async fn execute_poc(env: &mut LocalEnv, hacker_info: &HackerInfo) -> Result<(), anyhow::Error> {
    // Write your exploit here
    Ok(())
}
