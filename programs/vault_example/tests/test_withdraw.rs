use {
    anchor_lang::{
        prelude::*, solana_program::instruction::Instruction, InstructionData, ToAccountMetas,
    },
    litesvm::LiteSVM,
    solana_keypair::Keypair,
    solana_message::{Message, VersionedMessage},
    solana_signer::Signer,
    solana_transaction::versioned::VersionedTransaction,
};

#[test]
fn test_withdraw() {
    let program_id = vault_example::id();
    let payer = Keypair::new();
    let payer_pubkey = &payer.pubkey();

    let mut svm = LiteSVM::new();
    let bytes = include_bytes!("../../../target/deploy/vault_example.so");
    svm.add_program(program_id, bytes).unwrap();
    svm.airdrop(&payer_pubkey, 5_000_000_000).unwrap();

    let (vault_state_pda, _) =
        Pubkey::find_program_address(&[b"state", payer_pubkey.as_ref()], &program_id);
    let (vault_pda, _) =
        Pubkey::find_program_address(&[b"vault", payer_pubkey.as_ref()], &program_id);

    let init_accounts = vault_example::accounts::Initialize {
        user: *payer_pubkey,
        vault_state: vault_state_pda,
        vault: vault_pda,
        system_program: anchor_lang::system_program::ID,
    };

    let init_instruction = Instruction::new_with_bytes(
        program_id,
        &vault_example::instruction::Initialize.data(),
        init_accounts.to_account_metas(None),
    );

    let blockhash1 = svm.latest_blockhash();
    let msg_init =
        Message::new_with_blockhash(&[init_instruction], Some(&payer_pubkey), &blockhash1);
    let tx_init =
        VersionedTransaction::try_new(VersionedMessage::Legacy(msg_init), &[&payer]).unwrap();

    let init_res = svm.send_transaction(tx_init);
    assert!(init_res.is_ok());

    let deposit_accounts = vault_example::accounts::Deposit {
        user: *payer_pubkey,
        vault_state: vault_state_pda,
        vault: vault_pda,
        system_program: anchor_lang::system_program::ID,
    };

    let amount: u64 = 300_000_000;

    let deposit_instruction = Instruction::new_with_bytes(
        program_id,
        &vault_example::instruction::Deposit { amount }.data(),
        deposit_accounts.to_account_metas(None),
    );

    let blockhash2 = svm.latest_blockhash();
    let msg_deposit =
        Message::new_with_blockhash(&[deposit_instruction], Some(&payer_pubkey), &blockhash2);
    let tx_deposit =
        VersionedTransaction::try_new(VersionedMessage::Legacy(msg_deposit), &[&payer]).unwrap();

    let res = svm.send_transaction(tx_deposit);
    assert!(res.is_ok());
    let vault_account_balance_after_deposit = svm.get_balance(&vault_pda).unwrap();

    let withdraw_accounts = vault_example::accounts::Withdraw {
        user: *payer_pubkey,
        vault_state: vault_state_pda,
        vault: vault_pda,
        system_program: anchor_lang::system_program::ID,
    };

    let amount = 200_000_000;

    let withdraw_instructions = Instruction::new_with_bytes(
        program_id,
        &vault_example::instruction::Withdraw { amount }.data(),
        withdraw_accounts.to_account_metas(None),
    );

    let blockhash3 = svm.latest_blockhash();
    let msg_withdraw =
        Message::new_with_blockhash(&[withdraw_instructions], Some(&payer_pubkey), &blockhash3);
    let tx_withdraw =
        VersionedTransaction::try_new(VersionedMessage::Legacy(msg_withdraw), &[&payer]).unwrap();
    let withdraw_res = svm.send_transaction(tx_withdraw);
    println!("================= 📜 SOLANA TRANSACTION LOGS =================");
    match &withdraw_res {
        Ok(meta) => {
            for log in &meta.logs {
                println!("{}", log);
            }
        }
        Err(failed_meta) => {
            for log in &failed_meta.meta.logs {
                println!("{}", log);
            }
            println!("-------------------------------------------------------------");
            println!("Transaction Failed:\n{:#?}", failed_meta.err);
        }
    }
    println!("==============================================================");

    assert!(withdraw_res.is_ok());
    let vault_account_balance = svm.get_balance(&vault_pda).unwrap();
    println!("Total asset: {} lamports", vault_account_balance);
    assert!(vault_account_balance <= vault_account_balance_after_deposit);
}
