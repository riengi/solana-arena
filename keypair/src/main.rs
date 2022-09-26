use solana_sdk::signer::{keypair:: Keypair, Signer};

fn main() {

    let keypair = Keypair::new();
    println!("Keypair:\n{:?}", keypair);
    let pubkey = keypair.pubkey();
    println!("Pubkey: {}", pubkey.to_string());
    let base58keypair = keypair.to_base58_string();
    println!("Keypair b58:\n{}", base58keypair);
    let json_privatekey = keypair.to_bytes();
    println!("Keypair b58:\n{:?}", json_privatekey);
}
