fn from_hex(c: u8) -> u8 {
    match c {
        b'a'..=b'f' => c - b'a' + 10,
        b'A'..=b'F' => c - b'A' + 10,
        b'0'..=b'9' => c - b'0',
        _ => panic!()
    }
}

fn xor(src: &str, key: u8) {
    //let mut ret:String = String::with_capacity(src.len());
    let bs = src.as_bytes();
    let mut i = 0;
    let mut b:u8 =0;
    while i < bs.len(){
        b <<= 4;
        b += from_hex(bs[i]);
        if i % 2 == 1 {
            print!("{}", (b ^ key) as char);
            b = 0;
        }

        i += 1;
    }
    println!();
}
fn main() {

    for k in 0..=255 {
        xor("1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736", k);
    }
}
