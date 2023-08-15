use hex;
use itertools::multiunzip;
use std::fmt;
use std::iter::Iterator;

#[derive(Debug)]
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

impl FromIterator<u8> for Bytes {
    fn from_iter<I: IntoIterator<Item = u8>>(iter: I) -> Self {
        Bytes(iter.into_iter().collect())
    }
}

impl <'a> FromIterator<&'a u8> for Bytes {
    fn from_iter<I: IntoIterator<Item = &'a u8>>(iter: I) -> Self {
        Bytes(iter.into_iter().copied().collect())
    }
}

fn hash_to_bytes(hash: &str) -> Bytes {
    hex::decode(hash).unwrap().iter().collect()
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

    ascii_byte.iter().for_each(|ch| flag.push(*ch as char));
    flag
}

fn main() {
    let key1 = "a6c8b6733c9b22de7bc0253266a3867df55acde8635e19c73313";
    let k1_b = hash_to_bytes(key1);

    let key2_xor_key1 = "37dcb292030faa90d07eec17e3b1c6d8daf94c35d4c9191a5e1e";
    let k2_x_k1_b = hash_to_bytes(key2_xor_key1);

    let k2_b = xor_bytes_to_bytes_1(&k1_b, &k2_x_k1_b);

    let key2_xor_key3 = "c1545756687e7573db23aa1c3452a098b71a7fbf0fddddde5fc1";

    let k2_k3_b = hash_to_bytes(key2_xor_key3);
    let k3_b = xor_bytes_to_bytes_1(&k2_b, &k2_k3_b);

    let flag_k1_k2_k3 = "04ee9855208a2cd59091d04767ae47963170d1660df7f56f5faf";
    let fk1k2k3_b = hash_to_bytes(flag_k1_k2_k3);
    let the_flag_b = xor_bytes_to_bytes(&[&fk1k2k3_b, &k1_b, &k2_b, &k3_b]);
}
