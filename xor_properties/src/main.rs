use hex;
use std::fmt;

fn main() {
    let key1 = "a6c8b6733c9b22de7bc0253266a3867df55acde8635e19c73313";
    let key2_xor_key1 = "37dcb292030faa90d07eec17e3b1c6d8daf94c35d4c9191a5e1e";

    let key1_bytes = hex::decode(key1).unwrap();
    println!("the bytes of key1 are: {:?}", key1_bytes);

    let key2_xor_key1_bytes = hex::decode(key2_xor_key1).unwrap();
    println!("the k2 and k1 bytes are: {:?}", key2_xor_key1_bytes);

    let key2_bytes: Vec<u8> = key1_bytes.iter().zip(key2_xor_key1_bytes.iter()).map(|(k1, k2)| k1 ^ k2).collect();

    

    //compare_bytes(vec![bin_vec, new_bin]);
}

