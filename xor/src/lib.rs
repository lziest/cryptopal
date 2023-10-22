pub fn xor(left: &str, right: &str) -> Option<String> {
    if left.len() != right.len() {
        None
    } else {
        let mut ret:String = String::with_capacity(left.len());
        let mut i:usize = 0;
        let lbytes = left.as_bytes();
        let rbytes = right.as_bytes();
        while i<left.len() {
            let x = Some(from_hex(lbytes[i])? ^ from_hex(rbytes[i])?);
            match x {
                Some(v) => ret.insert(i, to_hex(v)),
                None => return None,
            }
            i+=1;
        };
        Some(ret)
    }
}

fn from_hex(c: u8) -> Option<u32> {
    match c {
        b'a'..=b'f' => Some(u32::from(c) - u32::from(b'a') + 10),
        b'A'..=b'F' => Some(u32::from(c) - u32::from(b'A') + 10),
        b'0'..=b'9' => Some(u32::from(c) - u32::from(b'0')),
        _ => None
    }
}

fn to_hex(v: u32) -> char {
    match v {
        0..=9 => (b'0'+v as u8) as char,
        10..=15 => (b'a'+ v as u8 - 10) as char,
        _ => panic!(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = xor("a", "a");
        assert_eq!(result.unwrap(), "0");
        let result = xor("1", "2");
        assert_eq!(result.unwrap(), "3");
        let result = xor("1c0111001f010100061a024b53535009181c", "686974207468652062756c6c277320657965");
        assert_eq!(result.unwrap(), "746865206b696420646f6e277420706c6179");
    }
}
