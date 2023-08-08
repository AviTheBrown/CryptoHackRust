use hex;
use std::fmt;

fn main() {
    let key = "a6c8b6733c9b22de7bc0253266a3867df55acde8635e19c73313";
    let bin_vec = key_to_bin(key);
    println!("the binary values are : \n{:?}\n", key_to_bin(key));
    println!("there ar: {} elements in the vec", bin_vec.len() );
    let new_bin = bin_vec.clone();

    compare_bytes(vec![bin_vec, new_bin]);
}

fn key_to_bin(key: &str) -> Vec<String> {
    let val = hex::decode(key).unwrap();
    val.iter().map(|byte| format!("{:08b}", byte)).collect()
}

fn compare_bytes(bin_vecs: Vec<Vec<String>>) {

    
}
