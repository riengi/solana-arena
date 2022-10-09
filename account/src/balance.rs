use solana_client::rpc_client::RpcClient;
use solana_sdk::signer::{keypair::Keypair, Signer};

const LAMPORT2SOL: u64 = 1_000_000_000;

pub fn balance_info() {
    let singer_keypair = dotenv::var("signer_keypair").expect("Cannot get .env signer keypair");
    println!("{}", singer_keypair);
    let url = dotenv::var("rpc_url").expect("rpc_url not found");
    println!("{}", url);

    let singer_wallet = Keypair::from_base58_string(&singer_keypair);

    println!("{:?}", singer_wallet);
    println!("{:?}", singer_wallet.pubkey());

    let client = RpcClient::new(url);
    let balance = client.get_balance(&singer_wallet.pubkey()).unwrap();

    let account = client.get_account(&singer_wallet.pubkey()).unwrap();
    println!("{:?}", account);
    println!("Lamports: {}", account.lamports);
    println!("Data lenght: {}", account.data.len());
    println!("Owner:{}", account.owner);

    println!("Rent epoch:{}", account.rent_epoch);

    let sol = balance / LAMPORT2SOL;
    println!("Balance: {} SOL", sol);
}
