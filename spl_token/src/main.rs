use solana_sdk::signer::{keypair::Keypair, Signer};
use solana_client::{rpc_client::RpcClient};

const LAMPORT2SOL: u64 = 1_000_000_000;

fn main() {
    let singer_keypair = dotenv::var("signer_keypair").unwrap();
    println!("{}", singer_keypair);
    let url = dotenv::var("rpc_url").unwrap();
    println!("{}", url);

    let singer_wallet = Keypair::from_base58_string(&singer_keypair);

    println!("{:?}", singer_wallet);
    println!("{:?}", singer_wallet.pubkey());

    let client = RpcClient::new(url);
    let balance = client.get_balance(&singer_wallet.pubkey()).unwrap();
    let sol = balance / LAMPORT2SOL;
    println!("Balance: {} SOL", sol ); 
}