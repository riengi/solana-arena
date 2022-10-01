from base58 import b58encode, b58decode

# byte keypair (from json) to encoded b58 keypair (phantom)
byte_keypair = [216,178,76,47,10,184,154,84,181,234,244,107,121,235,19,137,189,84,131,127,135,202,100,9,230,42,173,248,121,28,192,48,162,215,40,97,172,165,180,78,252,156,92,34,114,211,142,48,110,50,11,199,205,245,16,75,254,122,180,124,42,59,35,215]
expected_b58_public_key = "BxfLMLvn6Y4f19NuTXhVax1U3TXzy3KjpNbJX5FhEwup"

byte_public_key = byte_keypair[32:]
byte_private_key = byte_keypair[:32]

b58_encoded_public_key = b58encode(bytes(byte_public_key)).decode();
b58_encoded_private_key = b58encode(bytes(byte_private_key)).decode();

# encode separately 
print('B58 public key/address:', b58_encoded_public_key);
print('B58 private key:', b58_encoded_private_key);

b58_keypair = (byte_private_key + byte_public_key)
b58_encoded_keypair = b58encode(bytes(b58_keypair));
print('B58 keypair:', b58_encoded_keypair);


print('Test OK:', b58_encoded_public_key == expected_b58_public_key);

