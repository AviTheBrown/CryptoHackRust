fn main() {
    let string = "label";
    let the_vec = convert_string_to_binary(string);
    let the_num: u32 = 13;

    let new_num = xor(&the_vec, format!("{:08b}", the_num));
   let the_ascii_vec =  binary_to_ascii(&new_num.join(""));
    let ascii_char = ascii_to_chars(the_ascii_vec);
    println!("crypto{{{}}}", ascii_char);
}

fn convert_string_to_binary(string: &str) -> Vec<String> {
    string
        .chars()
        .map(|c| format!("{:08b}", c as u8))
        .collect()
}

fn xor(vec_to_compare: &Vec<String>, value: String) -> Vec<String> {
    vec_to_compare
        .iter()
        .map(|el| {
            let xor_result = el
                .chars()
                .zip(value.chars())
                .map(|(ch, vch)| if ch == vch { '0' } else { '1' })
                .collect::<String>();
            xor_result
        })
        .collect()
}

fn binary_to_ascii(binary_string: &str) -> Vec<u8> {
    binary_string
        .chars()
        .collect::<Vec<_>>()
        .chunks(8)
        .map(|chunk| {
            let byte = u8::from_str_radix(&chunk.iter().collect::<String>(), 2)
            .expect("failed to parse");
            byte
        })
        .collect()
}
fn ascii_to_chars(ascii_vec: Vec<u8>) -> String {
    let mut word = "".to_string();
    
    ascii_vec.into_iter().for_each(|ch| {
        word.push(char::from_u32(ch.into()).unwrap())
    });
    word
}
