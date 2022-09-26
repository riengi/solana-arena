# Encoded keypair from Phantom Wallet to JSON Solana CLI keypair
import base58

MY_PRIVATE_KEY_IN_BASE58 = "yncwVqYkBPwoyfPMQmugpWwUpKnorzhb8dUEiczwNWA8k1wmKfUdhPgWb7Lkt2619ZXXkxN7YYCS1gRy1EQWTuo"
byte_array = base58.b58decode(MY_PRIVATE_KEY_IN_BASE58)
json_string = "[" + ",".join(map(lambda b: str(b), byte_array)) + "]"
print(json_string)