from base58 import b58encode, b58decode

encoded_keypair = "Encoded_keypair_from_phantom_wallet:"

encoded_keypair = input(encoded_keypair)

keypair = b58decode(encoded_keypair)

# private key is first 32 bytes of the keypair
private_key = keypair[:32]

# public key is the last 32 bytes of the keypair
public_key = keypair[32:]

# wallet address is b52 encoded public key
wallet_address = b58encode(public_key).decode()

print("Wallet address:", wallet_address)

# reverse the process
_keypair = private_key + public_key

_encoded_keypair = b58encode(_keypair).decode()

print("Encoded keypair:", _encoded_keypair)
