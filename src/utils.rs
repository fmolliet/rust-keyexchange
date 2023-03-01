use std::num::ParseIntError;
use std::fmt::Write;
use std::convert::TryInto;

pub fn decode_hex(hex: &String)-> Result<Vec<u8>,ParseIntError>{
  (0..hex.len())
      .step_by(2)
      .map(|i| u8::from_str_radix(&hex[i..i + 2], 16))
      .collect()
}

pub fn encode_hex(bytes: &[u8]) -> String {
    let mut s = String::with_capacity(bytes.len() * 2);
    for &b in bytes {
        write!(&mut s, "{:02x}", b).unwrap();
    }
    s
}

// struct Payload{
//   publicKey: String
// }

pub fn vec_to_array<T, const N: usize>(v: Vec<T>) -> [T; N] {
    v.try_into()
        .unwrap_or_else(|v: Vec<T>| panic!("Expected a Vec of length {} but it was {}", N, v.len()))
}

// fn main() {
//   let hex:String = String::from("F0DA0915FF");
//   let mut payload:Payload = Payload{publicKey: hex};
  
//   let hexBytes = decode_hex(&payload.publicKey);
  
//   println!("{}", payload.publicKey);
//   println!("{:?}", hexBytes);
  
//   if let Ok(hbytes) =hexBytes {
//     println!("{}", encode_hex(&hbytes))
// }