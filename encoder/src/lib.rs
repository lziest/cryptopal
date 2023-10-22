pub fn base64_encode(raw: Vec<u8>) -> String {
    let mut index = 0;
    let len = raw.len();
    let mut result:String= String::from("");
    while index < len {
        let mut three_byte:u32 = u32::from(raw[index])<<16;
        let mut pads:u32 = 0;
        if index + 1 < len {
            three_byte += u32::from(raw[index+1])<<8;
        } else {
            pads += 1;
        }
        if index + 2 < len {
            three_byte += u32::from(raw[index+2]);
        } else {
            pads += 1;
        }
        index += 3;
        result += &three_byte_to_four_digit(&mut three_byte, pads);
    }
    result
}

fn three_byte_to_four_digit(three_byte:&mut u32, pads:u32) -> String {
    let mut i:u32 = 0;
    let mut result:String= String::with_capacity(4);
    while i < 4 {
        let digit:u8 = (*three_byte & 0x3f) as u8;
        let ch: char;
        *three_byte >>= 6;
        match digit {
            0..=25 => ch = (b'A'+digit).into(),
            26..=51 => ch = (b'a'+digit-26).into(),
            52..=61 => ch = (b'0'+digit-52).into(),
            62 => ch = '+',
            63 => ch = '-',
            _ => panic!(),
        }
        if i < pads {
            result.insert(0, '=');
        } else {
            result.insert(0, ch);
        }
        i+=1;
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;
    use hex::decode;

    #[test]
    fn it_works() {
        let result = base64_encode(vec![b'M']);
        assert_eq!(result, "TQ==");
        let result = base64_encode(vec![b'M', b'a', b'n']);
        assert_eq!(result, "TWFu");
        let result = base64_encode(vec![b'M', b'a']);
        assert_eq!(result, "TWE=");
        let result = base64_encode(br"Many hands make light work.".to_vec());
        assert_eq!(result, "TWFueSBoYW5kcyBtYWtlIGxpZ2h0IHdvcmsu");
        let raw_bytes = hex::decode("49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d").unwrap();
        let result = base64_encode(raw_bytes);
        assert_eq!(result, "SSdtIGtpbGxpbmcgeW91ciBicmFpbiBsaWtlIGEgcG9pc29ub3VzIG11c2hyb29t");
    }
}
