use axon_abi::builder::CkbHeaderBuilder;
use axon_abi::HeaderUpdateCall;
use ckb_jsonrpc_types::BlockView;
use ethers::types::transaction::eip2718::TypedTransaction;
use ethers::{signers::Wallet, types::H160};

fn load_ckb_block() -> BlockView {
    let buf = std::fs::read_to_string("examples/block.json").unwrap();
    serde_json::from_str(buf.as_str()).unwrap()
}

fn build_header_update_call() -> HeaderUpdateCall {
    let block = load_ckb_block();
    let header = CkbHeaderBuilder::default()
        .version(block.header.inner.version.into())
        .compact_target(block.header.inner.compact_target.into())
        .timestamp(block.header.inner.timestamp.into())
        .number(block.header.inner.number.into())
        .epoch(block.header.inner.epoch.into())
        .parent_hash(block.header.inner.parent_hash.0)
        .transactions_root(block.header.inner.transactions_root.0)
        .proposals_hash(block.header.inner.proposals_hash.0)
        .extra_hash(block.header.inner.extra_hash.0)
        .dao(block.header.inner.dao.0)
        .nonce(block.header.inner.nonce.into())
        .extension(block.extension.unwrap_or_default().into_bytes().into())
        .block_hash(block.header.hash.0)
        .build();

    HeaderUpdateCall::new(vec![header])
}

fn get_nonce() -> i32 {
    rand::random()
}

fn main() {
    let call = build_header_update_call();

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
