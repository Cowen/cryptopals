use hex;
use base64;

pub fn str_to_hex(s: &str) -> Result<Vec<u8>, hex::FromHexError> {
    hex::decode(s)
}

pub fn hex_to_base64(hex: &[u8]) -> String {
    base64::encode(hex)
}

pub fn fixed_xor(buf1: &[u8], buf2: &[u8]) -> Vec<u8> {
    // FIXME challenge specifies buffers are same length, never verified here
    buf1.iter().zip(buf2.iter())
        .map(|(b1, b2)| b1 ^ b2)
        .collect::<Vec<u8>>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    /// Section 1 - Challenge 1: Convert a hex string to base64
    /// https://cryptopals.com/sets/1/challenges/1
    fn given_challenge_input_when_converting_hex_to_base64_then_returns_challenge_output() {
        let cleartext = "49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d";

        let decoded_hex = str_to_hex(cleartext).unwrap();
        let encoded_base64 = hex_to_base64(&decoded_hex);

        assert_eq!("SSdtIGtpbGxpbmcgeW91ciBicmFpbiBsaWtlIGEgcG9pc29ub3VzIG11c2hyb29t", encoded_base64);
    }

    #[test]
    /// Section 1 - Challenge 2: Fixed-length XOR
    /// https://cryptopals.com/sets/1/challenges/2
    fn given_challenge_input_when_encoding_fixed_xor_then_returns_challenge_output() {
        let cleartext = "1c0111001f010100061a024b53535009181c";
        let text_hex = str_to_hex(cleartext).unwrap();
        let cipher = "686974207468652062756c6c277320657965";
        let cipher_hex = str_to_hex(cipher).unwrap();

        let encoded = fixed_xor(&text_hex, &cipher_hex);

        let expected_hex = str_to_hex("746865206b696420646f6e277420706c6179").unwrap();
        assert_eq!(expected_hex, encoded);
    }
}
