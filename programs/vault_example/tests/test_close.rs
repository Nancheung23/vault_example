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
fn test_close() {
    let program_id = vault_example::id();
    let payer = Keypair::new();
    let payer_pubkey = &payer.pubkey();
    let mut svm = LiteSVM::new();
    let bytes = include_bytes!("../../../target/deploy/vault_example.so");
    svm.add_program(program_id, bytes).unwrap();
    svm.airdrop(&payer.pubkey(), 1_000_000_000).unwrap();
    let (vault_state_pda, _) =
        Pubkey::find_program_address(&[b"state", payer.pubkey().as_ref()], &program_id);
    let (vault_pda, _) =
        Pubkey::find_program_address(&[b"vault", payer.pubkey().as_ref()], &program_id);
    let accounts = vault_example::accounts::Initialize {
        user: *payer_pubkey,
        vault_state: vault_state_pda,
        vault: vault_pda,
        system_program: anchor_lang::system_program::ID,
    };
    let instruction = Instruction::new_with_bytes(
        program_id,
        &vault_example::instruction::Initialize.data(),
        accounts.to_account_metas(None),
    );

    let blockhash = svm.latest_blockhash();
    let msg = Message::new_with_blockhash(&[instruction], Some(&payer.pubkey()), &blockhash);
    let tx = VersionedTransaction::try_new(VersionedMessage::Legacy(msg), &[&payer]).unwrap();
    // print whole log of TX
    let res = svm.send_transaction(tx);
    assert!(res.is_ok());
    let close_instruction = Instruction::new_with_bytes(
        program_id,
        &vault_example::instruction::Close.data(),
        accounts.to_account_metas(None),
    );
    let blockhash1 = svm.latest_blockhash();
    let msg = Message::new_with_blockhash(&[close_instruction], Some(&payer.pubkey()), &blockhash1);
    let tx = VersionedTransaction::try_new(VersionedMessage::Legacy(msg), &[payer]).unwrap();
    // print whole log of TX
    let close_res = svm.send_transaction(tx);
    // if let Err(err) = &res {
    //     println!("====== The reason of failure ======");
    //     println!("{:#?}", err);
    //     println!("===============================");
    // }
    // assert!(res.is_ok());
    println!("================= 📜 SOLANA TRANSACTION LOGS =================");
    match &close_res {
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
            println!("❌ Close Failed Reason:\n{:#?}", failed_meta.err);
        }
    }
    println!("==============================================================");
    assert!(close_res.is_ok());
}
