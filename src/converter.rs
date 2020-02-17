use hex;
use base64;

pub fn str_to_hex(s: &str) -> Result<Vec<u8>, hex::FromHexError> {
    hex::decode(s)
}

pub fn hex_to_base64(hex: &[u8]) -> String {
    base64::encode(hex)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    /// Section 1 - Challenge 1: Convert a hex string to base64
    /// https://cryptopals.com/sets/1/challenges/1
    fn given_challenge_input_converting_then_returns_challenge_output() {
        let cleartext = "49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d";

        let decoded_hex = str_to_hex(cleartext).unwrap();
        let encoded_base64 = hex_to_base64(&decoded_hex);

        assert_eq!("SSdtIGtpbGxpbmcgeW91ciBicmFpbiBsaWtlIGEgcG9pc29ub3VzIG11c2hyb29t", encoded_base64);
    }

}
