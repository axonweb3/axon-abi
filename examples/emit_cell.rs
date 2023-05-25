use axon_abi::builder::{CellInfoBuilder, CellOutputBuilder, OutPointBuilder};
use axon_abi::{CellBlockUpdate, CellUpdateCall};
use ckb_jsonrpc_types::TransactionView;
use ethers::types::transaction::eip2718::TypedTransaction;
use ethers::{signers::Wallet, types::H160};

fn load_ckb_tx() -> TransactionView {
    let buf = std::fs::read_to_string("examples/tx.json").unwrap();
    serde_json::from_str(buf.as_str()).unwrap()
}

fn build_cell_update_call() -> CellUpdateCall {
    let block_number = 9416528;
    let tx = load_ckb_tx();
    let tx_hash = tx.hash.0;
    let tx = tx.inner;

    let inputs = tx
        .inputs
        .iter()
        .map(|i| {
            OutPointBuilder::default()
                .tx_hash(i.previous_output.tx_hash.0)
                .index(i.previous_output.index.into())
                .build()
        })
        .collect::<Vec<_>>();
    let outputs = tx
        .outputs
        .iter()
        .zip(tx.outputs_data)
        .enumerate()
        .map(|(i, (c, data))| {
            CellInfoBuilder::default()
                .out_point(
                    OutPointBuilder::default()
                        .tx_hash(tx_hash)
                        .index(i as u32)
                        .build(),
                )
                .output(
                    CellOutputBuilder::default()
                        .type_(c.type_.clone().map(Into::into))
                        .lock(c.lock.clone().into())
                        .build(),
                )
                .data(data.as_bytes().into())
                .build()
        })
        .collect::<Vec<_>>();

    CellUpdateCall::new(vec![CellBlockUpdate::new(block_number, inputs, outputs)])
}

fn get_nonce() -> i32 {
    rand::random()
}

fn main() {
    let call = build_cell_update_call();

    let mut tx_req = TypedTransaction::default();
    tx_req.set_from(H160::random());
    tx_req.set_to(H160::random());
    tx_req.set_gas(21000u32);
    tx_req.set_gas_price(1u32);
    tx_req.set_data(call.abi_encode().into());
    tx_req.set_nonce(get_nonce());

    let wallet = Wallet::new(&mut rand::thread_rng());
    let sig = wallet.sign_transaction_sync(&tx_req).unwrap();
    let raw = tx_req.rlp_signed(&sig);

    println!("{:?}", raw);
}
