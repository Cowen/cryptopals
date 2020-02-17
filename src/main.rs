mod converter;
pub use converter::*;


fn main() {
    let cleartext = "49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d";

    let decoded_hex = converter::str_to_hex(cleartext).unwrap();

    println!("{:?}", cleartext);

    let encoded_base64 = hex_to_base64(&decoded_hex);

    println!("Base64: {:?}", encoded_base64);
}
