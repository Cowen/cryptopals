mod converter;
pub use converter::*;


fn main() {
    let cleartext = "1c0111001f010100061a024b53535009181c";
    let text_hex = str_to_hex(cleartext).unwrap();

    let cipher = "686974207468652062756c6c277320657965";
    let cipher_hex = str_to_hex(cipher).unwrap();

    let encoded = fixed_xor(&text_hex, &cipher_hex);

    let expected_hex = str_to_hex("746865206b696420646f6e277420706c6179").unwrap();

    println!("{:?}", std::str::from_utf8(&expected_hex).unwrap());
    println!("{:?}", std::str::from_utf8(&encoded).unwrap());
}
