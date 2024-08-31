use serde_json;
use std::env;

// Available if you need it!
// use serde_bencode

#[allow(dead_code)]
fn decode_bencoded_value(encoded_value: &str) -> serde_json::Value {
    // If encoded_value starts with a digit, it's a number
    if let Some(n) = encoded_value
        .strip_prefix('i')
        .and_then(|rest| rest.split_once('e'))
        .and_then(|(digest, _)| digest.parse::<i64>().ok()){
        n.into()
    }else if let Some((len,rest))=encoded_value.split_once(':'){
        if let Ok(len)=len.parse::<usize>(){
            return serde_json::Value::String(rest[..len].to_string())
        }
        panic!("Unhandled encoded value: {}", encoded_value)
    } else {
        panic!("Unhandled encoded value: {}", encoded_value)
    }
}

// Usage: your_bittorrent.sh decode "<encoded_value>"
fn main() {
    let args: Vec<String> = env::args().collect();
    let command = &args[1];

    if command == "decode" {
        let encoded_value = &args[2];
        let decoded_value = decode_bencoded_value(encoded_value);
        println!("{}", decoded_value.to_string());
    } else {
        println!("unknown command: {}", args[1])
    }
}
