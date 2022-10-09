// List accounts and qualify if NFT or NOT
use solana_client::rpc_client::RpcClient;
use solana_sdk::signer::{keypair::Keypair, Signer};

#[derive(serde::Deserialize)]
struct Parsed {
    info: SplToken,
}

#[derive(serde::Deserialize)]
struct SplToken {
    mint: String,
    #[serde(rename(deserialize = "tokenAmount"))]
    token_amount: Amount,
}
#[derive(serde::Deserialize)]
struct Amount {
    amount: String,
    decimals: u8,
}

pub fn list_nfts() {
    let singer_keypair = dotenv::var("signer_keypair").expect("Cannot get .env signer keypair");
    println!("{}", singer_keypair);
    let url = dotenv::var("rpc_url").expect("rpc_url not found");
    println!("{}", url);

    let singer_wallet = Keypair::from_base58_string(&singer_keypair);

    let client = RpcClient::new(url);

    let accounts = client
        .get_token_accounts_by_owner(
            &singer_wallet.pubkey(),
            solana_client::rpc_request::TokenAccountsFilter::ProgramId(spl_token::ID),
        )
        .unwrap();

    for a in accounts {
        println!("{:?}", a);

        println!("{:?}", a.account.owner);
        println!("{:?}", a.account.data);

        let json_data = a.account.data;
        let a = if let solana_account_decoder::UiAccountData::Json(d) = &json_data {
            Some(d)
        } else {
            None
        };

        if let Some(x) = a {
            let parsed = serde_json::from_value::<Parsed>(x.parsed.clone()).unwrap();
            println!("parsed.info.mint: {:?}", parsed.info.mint);

            let decimals = u32::from(parsed.info.token_amount.decimals);
            println!("decimals:{}", decimals);
            let amount_raw = parsed.info.token_amount.amount.parse::<u64>().unwrap();
            println!("amount_raw:{}", amount_raw);
            let num = amount_raw / u64::pow(10, decimals);
            println!(
                "{}",
                if decimals == 0 && amount_raw == 1 {
                    "NFT"
                } else {
                    "NON-NFT"
                }
            );

            println!("parsed.info.token_amount.amount: {:?}", num);
        }
    }
}
