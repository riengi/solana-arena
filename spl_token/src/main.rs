use solana_client::rpc_client::RpcClient;
use solana_sdk::pubkey::Pubkey;

// const LAMPORT2SOL: u64 = 1_000_000_000;

fn main() {
    // .env (get account_pk and token_id)
    let account = dotenv::var("account_pk").unwrap();
    let token_id = dotenv::var("token_id").unwrap();
    println!("Account address:\t {}", account);
    println!("Token ID:\t\t {}", token_id);

    // convert it to Pubkey
    let account_pk: Pubkey = account.parse().unwrap();
    let token_id_pk: Pubkey = token_id.parse().unwrap();

    // Initialize RPC client
    let url = dotenv::var("rpc_url").expect("rpc_url not found");
    println!("Network RPC url:\t {}", url);
    let client = RpcClient::new(url);

    // Get associated token address
    let associated_token_address =
        spl_associated_token_account::get_associated_token_address(&account_pk, &token_id_pk);
    println!("Token account address:\t {:?}", associated_token_address);

    // Get balance of associated token account
    let balance = client
        .get_token_account_balance(&associated_token_address)
        .unwrap();
    println!("Token amount:\t\t {:?}", balance);
    if let Some(amount) = balance.ui_amount {
        println!("SPL Token balance:\t {} tokens", amount)
    }
    println!("Token balance decimals:\t {}", balance.decimals);
}
