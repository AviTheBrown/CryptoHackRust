use hex;
use itertools::multiunzip;
use std::fmt;

struct Bytes(Vec<u8>);

impl Bytes {
    fn iter(&self) -> BytesIterator {
        BytesIterator {
            inner: self.0.iter(),
        }
    }
}

struct BytesIterator<'a> {
    inner: std::slice::Iter<'a, u8>,
}

impl<'a> Iterator for BytesIterator<'a> {
    type Item = &'a u8;

    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next()
    }
}

impl std::iter::FromIterator<u8> for Bytes {
    fn from_iter<I: IntoIterator<Item = u8>>(iter: I) -> Self {
        Bytes(iter.into_iter().collect())
    }
}

impl From<Vec<u8>> for Bytes {
    fn from(vec: Vec<u8>) -> Self {
        Bytes(vec)
    }
}

impl fmt::Debug for Bytes {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Bytes{:?}", self.0)
    }
}

fn hash_to_bytes(hash: &str) -> Bytes {
    hex::decode(hash).unwrap().into()
}

fn xor_bytes_to_bytes_1(bytes1: &Bytes, bytes2: &Bytes) -> Bytes {
    bytes1
        .iter()
        .zip(bytes2.iter())
        .map(|(h1, h2)| h1 ^ h2)
        .collect::<Bytes>()
}

fn xor_bytes_to_bytes(byte_slice: &[&Bytes]) -> Bytes {
    let mut iter_bytes = byte_slice.iter().map(|bytes| bytes.iter());

    let first = match iter_bytes.next() {
        Some(iter) => iter,
        None => return Bytes(Vec::new().into()),
    };

    let xor_result = first
        .zip(iter_bytes)
        .map(|(first_byte, other_bytes)| other_bytes.fold(*first_byte, |acc, byte| acc ^ byte))
        .collect();

    xor_result
}

fn ascii_to_chars(ascii_byte: Bytes) -> String {
    let mut flag = "".to_string();

    ascii_byte.into_iter().for_each(|ch| flag.push(ch as char));
    flag
}

fn main() {
    // the hash values for key1
    let key1 = "a6c8b6733c9b22de7bc0253266a3867df55acde8635e19c73313";
    let k1_b = hash_to_bytes(key1);

    // hash value -> evaluated result of key1 ^ key2.
    // not knowing the hash of key2
    let key2_xor_key1 = "37dcb292030faa90d07eec17e3b1c6d8daf94c35d4c9191a5e1e";
    let k2_x_k1_b = hash_to_bytes(key2_xor_key1);

    // key1_bytes -> we first decode the hex number and get ther result of the option.
    // pub fn decode<T: AsRef<[u8]>>(data: T) -> Result<Vec<u8>, FromHexError>
    let key1_bytes = hex::decode(key1).unwrap();

    // this decodes key2 ^ key1 ( this HASH is the evaluated result of key1 ^ key2 not key2 itselt)
    let key2_xor_key1_bytes = hex::decode(key2_xor_key1).unwrap();

    let k2_b = xor_bytes_to_bytes_1(&k1_b, &k2_x_k1_b);
    // inorder to get the byte value for key2
    // we iterate over the bytes of key1 and key1&key2 bytes together using zip using map then collecting the bytes.
    let key2_bytes: Vec<u8> = key1_bytes
        .iter()
        .zip(key2_xor_key1_bytes.iter())
        .map(|(k1, k2)| k1 ^ k2)
        .collect();

    let key2_xor_key3 = "c1545756687e7573db23aa1c3452a098b71a7fbf0fddddde5fc1";
    let key2_xor_key3_bytes = hex::decode(key2_xor_key3).unwrap();

    let k2_k3_b = hash_to_bytes(key2_xor_key3);
    let k3_b = xor_bytes_to_bytes_1(&k2_b, &k2_k3_b);

    // the same process as previous
    let key3_bytes: Vec<u8> = key2_bytes
        .iter()
        .zip(key2_xor_key3_bytes.iter())
        .map(|(k2, k3)| k2 ^ k3)
        .collect();

    let flag_k1_k2_k3 = "04ee9855208a2cd59091d04767ae47963170d1660df7f56f5faf";
    let flag_k1_k2_k3_bytes = hex::decode(flag_k1_k2_k3).unwrap();

    let fk1k2k3_b = hash_to_bytes(flag_k1_k2_k3);

    println!("the flag bytes are:{:?}", flag_k1_k2_k3_bytes);

    let the_flag_b = xor_bytes_to_bytes(&[&fk1k2k3_b, &k1_b, &k2_b, &k3_b]);
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
