use solana_client::rpc_client::RpcClient;
use solana_sdk::pubkey::Pubkey;
use mpl_token_metadata::state::Metadata;
use borsh::de::BorshDeserialize;

// const LAMPORT2SOL: u64 = 1_000_000_000;

pub fn print_metadata() {
    // .env (get account_pk and token_id)
    let token_id = dotenv::var("degods").unwrap();
    println!("Token ID:\t\t {}", token_id);

    // convert it to Pubkey
    let token_id_pk: Pubkey = token_id.parse().unwrap();

    // Initialize RPC client
    let url = dotenv::var("rpc_url_mainnet").expect("rpc_url not found");
    println!("Network RPC url:\t {}", url);
    let client = RpcClient::new(url);

    let metadata = get_metadata(&client, &token_id_pk);

    println!("{} metadata:\n{:#?}", token_id_pk.to_string(), metadata);

    let data = metadata.data;
    println!("Collection: {}",data.name);
    println!("Symbol: {}",data.symbol);
    println!("URI: {}",data.uri);
    println!("Fee basis points: {}",data.seller_fee_basis_points);



}

fn get_metadata(rpc: &RpcClient, mint_address: &Pubkey) -> Metadata {
    let (meta_addr, _) = mpl_token_metadata::pda::find_metadata_account(&mint_address);
    let metadata_account = rpc.get_account(&meta_addr).unwrap();
    let acct = &mut &metadata_account.data[..];
    Metadata::deserialize(acct).unwrap()    
}