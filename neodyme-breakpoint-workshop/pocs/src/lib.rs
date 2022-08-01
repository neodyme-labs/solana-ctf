use poc_framework::{solana_transaction_status::EncodedConfirmedTransaction, PrintableTransaction};

pub fn assert_tx_success(tx: EncodedConfirmedTransaction) -> EncodedConfirmedTransaction {
    match &tx.transaction.meta {
        Some(meta) if meta.err.is_some() => {
            tx.print();
            panic!("tx failed!")
        }
        _ => tx,
    }
}
