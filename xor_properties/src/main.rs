use hex;
use itertools::multiunzip;
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
    let key2_bytes: Vec<u8> = key1_bytes
        .iter()
        .zip(key2_xor_key1_bytes.iter())
        .map(|(k1, k2)| k1 ^ k2)
        .collect();

    let key2_xor_key3 = "c1545756687e7573db23aa1c3452a098b71a7fbf0fddddde5fc1";
    let key2_xor_key3_bytes = hex::decode(key2_xor_key3).unwrap();

    // the same process as previous
    let key3_bytes: Vec<u8> = key2_bytes
        .iter()
        .zip(key2_xor_key3_bytes.iter())
        .map(|(k2, k3)| k2 ^ k3)
        .collect();

    let flag_k1_k2_k3 = "04ee9855208a2cd59091d04767ae47963170d1660df7f56f5faf";
    let flag_k1_k2_k3_bytes = hex::decode(flag_k1_k2_k3).unwrap();

    println!("the flag bytes are:{:?}", flag_k1_k2_k3_bytes);

    // this on is a bit differnent as it uses multiple zips to xor all the bytes using the associate propertie
    let flag_bytes: Vec<u8> = flag_k1_k2_k3_bytes
        // clones the byte as it is used on line 41
        .clone()
        // into_iter() take posession of the object
        // i cpuld have used iter() as well
        .into_iter()
        // the zips tie each byte, so that they can be used together for whatever reason.
        .zip(key1_bytes.iter())
        .zip(key2_bytes.iter())
        .zip(key3_bytes.iter())
        // closure using lamda function to xor all the btyes
        .map(|(((fg, k1), k2), k3)| fg ^ k1 ^ k2 ^ k3)
        // collects all result of the inital collection (Vec<u8> from hash::decode()) into the new collection (Vec<u8> type annotation)
        .collect();

    println!("the flag bytes are: {:?}", flag_bytes);

    println!("the lenght is : {:?}", key3_bytes.len());

    println!("the string is: {:?}", ascii_to_chars(flag_bytes));
}

// used to obtain the flag
fn ascii_to_chars(ascii_vec: Vec<u8>) -> String {
    let mut flag = "".to_string();

    ascii_vec
        .into_iter()
        .for_each(|ch| flag.push(char::from_u32(ch.into()).unwrap()));
    flag
}
