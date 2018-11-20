#![feature(uniform_paths)]
#![feature(test)]

extern crate test;

use std::ffi::CStr;
use std::str;

use encoding::all::ISO_8859_1;
use encoding::{DecoderTrap, EncoderTrap, Encoding};

fn string_from_iso_bytes(buffer: &[u8]) -> std::string::String {
    ISO_8859_1.decode(&buffer, DecoderTrap::Strict).unwrap()
}

fn bytes_from_from_iso_string(keyword: &str) -> Vec<u8> {
    ISO_8859_1.encode(keyword, EncoderTrap::Strict).unwrap()
}

fn cstring_from_ptr(buffer: &[i8]) -> &str {
    //note needs null ptr terminator
    let cstr = unsafe { CStr::from_ptr(buffer.as_ptr()) };
    let result = cstr.to_str().unwrap();
    result
}

fn cstring_from_bytes(buffer: &[u8]) -> &str {
    let cstr = CStr::from_bytes_with_nul(&buffer).unwrap();
    cstr.to_str().unwrap()
}

fn string_from_utf8(buffer: &[u8]) -> &str {
    let ui32: &str = str::from_utf8(&buffer).unwrap();
    &ui32
}

#[cfg(test)]
mod tests {

    use super::*;
    use test::{black_box, Bencher};

    #[test]
    fn test_bytes_from_from_iso_string() {
        assert_eq!(bytes_from_from_iso_string("TC0P"), vec![84, 67, 48, 80]);
    }

    #[bench]
    fn bench_bytes_from_from_iso_string(b: &mut Bencher) {
        b.iter(|| {
            for _i in 1..100 {
                black_box(bytes_from_from_iso_string("TC0P"));
            }
        }) //note the lack of semi for return to remove optimization
    }

    #[test]
    fn test_string_from_iso_bytes() {
        let tc0p: [u8; 4] = [84, 67, 48, 80];
        assert_eq!(string_from_iso_bytes(&tc0p), "TC0P".to_string());
    }

    #[bench]
    fn bench_string_from_iso_bytes(b: &mut Bencher) {
        let tc0p: [u8; 4] = [84, 67, 48, 80];

        b.iter(|| {
            for _i in 1..100 {
                black_box(string_from_iso_bytes(&tc0p));
            }
        }) //note the lack of semi for return to remove optimization
    }

    #[test]
    fn test_cstring_from_ptr() -> Result<(), std::io::Error> {
        let tcxc: [i8; 5] = [0x54, 0x43, 0x58, 0x43, 0x0];

        assert_eq!(cstring_from_ptr(&tcxc), String::from("TCXC"));
        Ok(())
    }

    #[bench]
    fn bench_cstring_from_ptr(b: &mut Bencher) {
        let tcxc: [i8; 5] = [0x54, 0x43, 0x58, 0x43, 0x0];
        b.iter(|| {
            for _i in 1..100 {
                black_box({ cstring_from_ptr(&tcxc) });
            }
        }) //note the lack of semi for return to remove optimization
    }

    #[test]
    fn test_cstring_from_bytes() -> Result<(), std::ffi::FromBytesWithNulError> {
        let tcxc: [u8; 5] = [0x54, 0x43, 0x58, 0x43, 0x0];
        assert_eq!(cstring_from_bytes(&tcxc), String::from("TCXC"));
        Ok(())
    }

    #[bench]
    fn bench_cstring_from_bytes(b: &mut Bencher) {
        let tcxc: [u8; 5] = [0x54, 0x43, 0x58, 0x43, 0x0];

        b.iter(|| {
            for _i in 1..100 {
                black_box({ cstring_from_bytes(&tcxc) });
            }
        }) //note the lack of semi for return to remove optimization
    }

    #[test]
    fn test_string_from_utf8() -> Result<(), std::str::Utf8Error> {
        let tcxc = [0x54, 0x43, 0x58, 0x43];

        assert_eq!(string_from_utf8(&tcxc), String::from("TCXC"));
        Ok(())
    }

    #[bench]
    fn bench_string_from_utf8(b: &mut Bencher) {
        let tcxc = [0x54, 0x43, 0x58, 0x43];

        b.iter(|| {
            for _i in 1..100 {
                black_box({ string_from_utf8(&tcxc) });
            }
        }) //note the lack of semi for return to remove optimization
    }

}
