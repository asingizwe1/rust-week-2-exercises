use hex::{decode, encode};

pub fn decode_hex(hex_str: &str) -> Result<Vec<u8>, String> {
    // TODO: Decode hex string into Vec<u8>, return error string on failure
    //2 hex characters are 1 byte
    //map_err -> helps to transfor Err variant into another type F ,
    //Basically it ignores Ok(T)
    hex::decode(hex_str).map_err(|e| e.to_string())
}

pub fn to_big_endian(bytes: &[u8]) -> Vec<u8> {
    // TODO: Reverse the byte order of input slice and return as Vec<u8>
    //slices use .iter()[It creates an iterator that goes through each element in the slice,one by one], .rev() for reversing and .cloned() which turns the &[u8] to owned values, then .collect() gathers the values into a Vec<> basically into a type you define for it
    //instead of dereferencing we can use cloned method
    bytes.iter().rev().cloned().collect::<Vec<u8>>() //Collect into a Vec<u8>
}

pub fn bytes_to_hex(bytes: &[u8]) -> String {
    // TODO: Implement conversion of bytes slice to hex string
    //encode will convert a byte to 2 hex characters
    encode(bytes)
}

pub fn hex_to_bytes(hex: &str) -> Result<Vec<u8>, hex::FromHexError> {
    // TODO: Implement conversion of hex string to bytes vector
    decode(hex)
}

pub fn swap_endian_u32(num: u32) -> [u8; 4] {
    // TODO: Implement little-endian byte swap for u32
    num.to_le_bytes()
}

pub fn parse_satoshis(input: &str) -> Result<u64, String> {
    // TODO: Parse input string to u64, return error string if invalid
    //we use turbofish ::<> for the inference algorithm to know the type we are parsing to
    //map_err is used to produce our specific Error string
    //uding |_| to ignore the actual error instead of (|e| e.to_string())
    input
        .parse::<u64>()
        .map_err(|_| "Invalid satoshi amount".to_string())
}

pub enum ScriptType {
    P2PKH,
    P2WPKH,
    Unknown,
} //slice patterns

  /*[a, b, c, ..] → matches slices whose first three elements are a, b, c, and ignores the rest.

  [x, ..] → matches slices with at least one element, binding the first element to x.

  [x, y] → matches slices with exactly two elements.

  [] → matches an empty slice.

  [first, .., last] → matches slices with at least two elements, binding the first and last. */
  
pub fn classify_script(script: &[u8]) -> ScriptType {
    // TODO: Match script pattern and return corresponding ScriptType
    match script {
        [0x76, 0xa9, 0x14, ..] => ScriptType::P2PKH,
        [0x00, 0x14, ..] => ScriptType::P2WPKH,
        _ => ScriptType::Unknown,
    }
}

// TODO: complete Outpoint tuple struct
//tuple structs dont have field names
pub struct Outpoint(String, u32); 
                                  your wallet does not have a single "account balance". Instead,
                                  your balance is a collection of discrete chunks of Bitcoin that were sent to you. */

pub fn read_pushdata(script: &[u8]) -> &[u8] {
    // TODO: Return the pushdata portion of the script slice (assumes pushdata starts at index 2)
    &script[2..]
}

pub trait Wallet {
    fn balance(&self) -> u64;
}

pub struct TestWallet {
    pub confirmed: u64,
}

impl Wallet for TestWallet {
    fn balance(&self) -> u64 {
        //&self is shorthand for self: &TestWallet
        // TODO: Return the wallet's confirmed balance
        self.confirmed
    } //automatic dereferencing of self(&self)
}
//balance isnt owned hence it must be dereferenced
pub fn apply_fee(balance: &mut u64, fee: u64) {
    // TODO: Subtract fee from mutable balance reference
    //*balance-fee -  computes a value but doesn't assign it back
    *balance -= fee;
}

pub fn move_txid(txid: String) -> String {
    // TODO: Return formatted string including the txid for display or logging
    format!("txid: {}", txid)
} //format always returns a formatted string

// TODO: Add necessary derive traits
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Opcode {
    OpChecksig,
    OpDup,
    OpInvalid,
}

impl Opcode {
    pub fn from_byte(byte: u8) -> Result<Self, String> {
        // TODO: Implement mapping from byte to Opcode variant
        match byte {
            0xac => Ok(Opcode::OpChecksig),
            0x76 => Ok(Opcode::OpDup),
            _ => Err(format!("Invalid opcode: 0x{:02x}", byte)),
        }
    }
}

// TODO: Add necessary derive traits
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UTXO {
    pub txid: Vec<u8>,
    pub vout: u32,
    pub value: u64,
}

pub fn consume_utxo(utxo: UTXO) -> UTXO {
    // TODO: Implement UTXO consumption logic (if any)
    utxo
}


//It serves as an exact "map" to locate the source funds you are attempting to spend
//in btc every transaction input points to a previous output - pointer has 2 pieces of info which are txid(String) and vout(output index)(u32)-[because you can have multiple transactions] - output index in that transaction *Because Bitcoin uses the UTXO (Unspent Transaction Output) model,