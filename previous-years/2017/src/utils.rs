//use std::num::ParseIntError;
use std::fs::File;
use std::io::{BufReader, BufRead};

//pub fn unhexlify(s: &str, radix: u32) -> Result<Vec<u8>, ParseIntError> {
//    (0..s.len())
//        .map(|idx| u8::from_str_radix(&s[idx..idx+1], radix))
//        .collect()
//}

pub fn filelines(fname: &str) -> ::std::io::Result<Vec<String>> {
    let fobj = File::open(fname).map(BufReader::new)?;
    fobj.lines().collect()
}

#[cfg(test)]
mod tests {
//    use super::*;

//    #[test]
//    fn test_unhexlify_base_10() {
//        assert_eq!(unhexlify("12345", 10), Ok(vec![1,2,3,4,5]));
//    }
//
//    #[test]
//    fn test_unhexlify_base_16() {
//        assert_eq!(unhexlify("ABCDEF", 16), Ok(vec![10, 11, 12, 13, 14, 15]));
//    }
}