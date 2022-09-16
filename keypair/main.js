// Generating keypair by various approaches
// Example keypair: DON'T USE IT ON MAINNET !!!
// Wrote new keypair to example-keypair.json
// ==============================================================================
// pubkey: 6HxevKoH5fXYEVnJR8cAtueSNBojdRMRPjt7tGDeyEqK
// ==============================================================================
// Save this seed phrase and your BIP39 passphrase to recover your new keypair:
// photo pulse armor must ticket present dynamic fortune game refuse naive street
// ==============================================================================


// Solana JS dependencies
const { Keypair } = require('@solana/web3.js');
const { Buffer } = require('buffer');
const { derivePath } = require('ed25519-hd-key');
const fs = require('fs');
const bip39 = require('bip39');

// generate new keypair
const myKeypair = Keypair.generate();

console.log("Public key:", myKeypair.publicKey);
console.log("B58  :", myKeypair.publicKey.toBase58());
console.log("Str  :", myKeypair.publicKey.toString());
console.log("Bytes:", myKeypair.publicKey.toBytes());
console.log("Len :" + myKeypair.publicKey.toString().length);

console.log('-'.repeat(70))
console.log("Secret key    :", myKeypair.secretKey);
console.log("Secret key len:" + myKeypair.secretKey.length);
    
console.log('-'.repeat(70))
// read generated key from file (sync) created with solana-keygen new -o example-keygen.json
// read file into a buffer
const buffer = fs.readFileSync('./example-keypair.json','utf-8')
console.log('Buffer:', buffer);

// parse JSON into object
const json = JSON.parse(buffer);
console.log('JSON:', json);

// convert to Uint8 array
const buffer2 = Buffer.from(json);
console.log('Buffer2:', json);

// create keypair form Uint8 array secret key
const keypairFromFile = Keypair.fromSecretKey(buffer2);
console.log(keypairFromFile.publicKey);
console.log(keypairFromFile.publicKey.toString());
console.log(keypairFromFile.secretKey);


// Generating keypair from derived seed
// import { derivePath } from 'ed25519-hd-key';

const mnemonic ="photo pulse armor must ticket present dynamic fortune game refuse naive street"
console.log('Mnemonic validation', bip39.validateMnemonic(mnemonic));


const seed = bip39.mnemonicToSeedSync(mnemonic)
const seedBuffer = Buffer.from(seed).toString('hex');

console.log('seed:', seed);

// const seed2 = Buffer.from(seed);
// let a = new Uint8Array(seedBuffer.toJSON().data.slice(0,32));

const path501Change = "m/501'/0'/0'"
// const path44Change = `m/44'/501'/0'/0'`;
// const derivedSeed = derivePath(path44Change, seedBuffer).key;
const derivedSeed = derivePath(path501Change, seedBuffer.toString('hex')).key;

const keypairFromSeed3 = Keypair.fromSeed(derivedSeed);
const keypairFromSeed2 = Keypair.fromSeed(derivedSeed);
const keypairFromSeed = Keypair.fromSeed(derivedSeed);

console.log('-'.repeat(70));
console.log('From seed:')
console.log(keypairFromSeed.publicKey);
console.log(keypairFromSeed.publicKey.toString());
console.log(keypairFromSeed.secretKey);


// Generating from seed 

console.log(bip39.validateMnemonic(mnemonic)) // true
bip39.mnemonicToSeed(mnemonic).then(buffer => {
    let a = new Uint8Array(buffer.toJSON().data.slice(0,32));
    const key = Keypair.fromSeed(a);
    console.log('Public key:', key.publicKey.toString());
}).catch(err => console.log(err))

