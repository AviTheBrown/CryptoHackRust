use hex;
use std::fmt;

fn main() {
    // the hash values for key1
    let key1 = "a6c8b6733c9b22de7bc0253266a3867df55acde8635e19c73313";

    // hash value -> evaluated result of key1 ^ key2.
        // not knowing the hash of key2
    let key2_xor_key1 = "37dcb292030faa90d07eec17e3b1c6d8daf94c35d4c9191a5e1e";

    // key1_bytes -> we first decode the hex number and get ther result of the option.
        // pub fn decode<T: AsRef<[u8]>>(data: T) -> Result<Vec<u8>, FromHexError>
    let key1_bytes = hex::decode(key1).unwrap();
    println!("the bytes of key1 are: {:?}", key1_bytes);

    // this decodes key2 ^ key1 ( this HASH is the evaluated result of key1 ^ key2 not key2 itselt)
    let key2_xor_key1_bytes = hex::decode(key2_xor_key1).unwrap();
    println!("the k2 ^ k1 bytes are: {:?}", key2_xor_key1_bytes);

    // inorder to get the byte value for key2
        // we iterate over the bytes of key1 and key1&key2 bytes together using zip using map then collecting the bytes.
    let key2_bytes: Vec<u8> = key1_bytes.iter().zip(key2_xor_key1_bytes.iter()).map(|(k1, k2)| {k1 ^ k2}).collect();

    let key2_xor_key3 = "c1545756687e7573db23aa1c3452a098b71a7fbf0fddddde5fc1";
    let key2_xor_key3_bytes = hex::decode(key2_xor_key3).unwrap();

    //compare_bytes(vec![bin_vec, new_bin]);
}

