use std::collections::HashMap;
use clap::Parser;
use ethers::prelude::{
    rand::{prelude::IteratorRandom, thread_rng, Rng},
    *,
};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// The pattern to look for
    #[arg(short, long)]
    private_key: String,
    /// Start nonce
    #[arg(short, long, default_value_t = 0)]
    start_nonce: u32,
    // End nonce
    #[arg(short, long, default_value_t = 10000)]
    end_nonce: u32,
}

// From a random account, generate a random transaction to a random user, with anywhere from 0.1 to
// 1 eth
fn main() {
    let args = Cli::parse();
    let wallets = [
        args.private_key,
    ]
    .iter()
    .map(|pk| {
        pk.strip_prefix("0x")
            .unwrap_or(pk)
            .parse::<LocalWallet>()
            .unwrap()
            .with_chain_id(Chain::AnvilHardhat)
    })
    .collect::<Vec<_>>();

    let rng = &mut thread_rng();

    let mut nonces = HashMap::<_, u64>::new();

    let wallet = &wallets[0];
    let addr = wallet.address();
    for i in args.start_nonce..args.end_nonce {
        let amt = rng.gen_range(1..10);

        let nonce = nonces.entry(addr).or_default();

        let tx = TransactionRequest::new()
            .from(addr)
            .to(Address::random())
            .nonce(*nonce)
            .gas(21000)
            .gas_price(100e9 as u64)
            .value(amt)
            .into();

        *nonce += 1;

        let sig = wallet.sign_transaction_sync(&tx);

        let encoded = tx.rlp_signed(&sig);
        println!("{}", hex::encode(encoded));
    }
}
