#![feature(slice_patterns)]
#![feature(test)]
extern crate test;

use std::f32;

pub fn atoi_via_bytes(chars: [u8; 4]) -> u32 {
    u32::from_be_bytes(chars)
}

// pub fn shifted_atoi_via_bytes(chars: Vec<u8>) -> u32 {
//     let len = chars.len();

//     let (prefix, shorts, _suffix) = chars.align_to::<u32>();

//     let shift = |acc, (index, value)| -> u32 {
//         let shift = ((len - 1 - index) as u32) * 8;
//         acc + ((value as u32) << shift)
//     };

//     let sum: u32 = chars.into_iter().enumerate().fold(0, shift);
//     sum
// }

pub fn shifted_atoi_via_fold(chars: Vec<u8>) -> u32 {
    let len = chars.len();

    let shift = |acc, (index, value)| -> u32 {
        let shift = ((len - 1 - index) as u32) * 8;
        acc + ((value as u32) << shift)
    };

    let sum: u32 = chars.into_iter().enumerate().fold(0, shift);
    sum
}

pub fn shifted_atoi_via_map(chars: Vec<u8>) -> u32 {
    let len = chars.len();

    let shift = |(index, value)| -> u32 {
        let shift = ((len - 1 - index) as u32) * 8;
        ((value as u32) << shift)
    };

    let sum: u32 = chars.into_iter().enumerate().map(shift).sum();
    sum
}

pub fn shifted_atoi_via_loop(chars: Vec<u8>) -> u32 {
    let mut sum = 0u32;

    let len = chars.len();

    for i in 0..len {
        sum += (chars[i] as u32) << (((len - 1 - i) as u32) << 3);
    }
    sum
}

pub fn compose_f32(exponent: f32, mantissa: f32) -> f32 {
    mantissa * exponent.exp2()
}

//https://stackoverflow.com/questions/37668886/slice-to-fixed-size-array
fn clone_into_array<A, T>(slice: &[T]) -> A
where
    A: Sized + Default + AsMut<[T]>,
    T: Clone,
{
    let mut a = Default::default();
    <A as AsMut<[T]>>::as_mut(&mut a).clone_from_slice(slice);
    a
}

pub fn getu16_stable(my_array: [u8; 4]) -> u16 {
    u16::from_be_bytes(clone_into_array(&my_array[0..2]))
}

pub fn getu16(my_array: [u8; 4]) -> u16 {
    let &[a, b, ..] = &my_array;
    let abs: [u8; 2] = [a, b];
    u16::from_be_bytes(abs)
}

pub fn getu32(my_array: [u8; 4]) -> u32 {
    let &[a, b, c, d, ..] = &my_array;
    let abs: [u8; 4] = [a, b, c, d];
    u32::from_be_bytes(abs)
}

#[cfg(test)]
mod tests {

    use super::*;
    use byteorder::{BigEndian, ReadBytesExt};
    use std::io::Cursor;
    use test::{black_box, Bencher};

    #[no_mangle]
    fn get_type() -> u32 {
        0x41480000
    }

    #[no_mangle]
    fn decode() -> Vec<u8> {
        vec![84, 67, 48, 80]
    }

    // #[no_mangle]
    // fn decode2(key: &str) -> [u8; 4] {
    //     assert_eq!(4, key.len());

    //     key.as_bytes()
    // }

    #[no_mangle]
    fn my_array1() -> [u8; 4] {
        [0x23, 0xd9, 0xc4, 0x3a]
    }

    #[no_mangle]
    fn my_array0() -> [u8; 4] {
        [0x0, 0x0, 0x2, 0x31]
    }

    #[test]
    fn test_fold() {
        let decoded = decode();
        let sum = shifted_atoi_via_fold(decoded);
        assert_eq!(sum, 1413689424);
    }

    #[test]
    fn test_map() {
        let decoded = decode();
        let sum = shifted_atoi_via_map(decoded);
        assert_eq!(sum, 1413689424);
    }

    #[test]
    fn test_loop() {
        let decoded = decode();
        let sum = shifted_atoi_via_loop(decoded);
        assert_eq!(sum, 1413689424);
    }

    // #[test]
    // fn test_as_bytes() -> Result<(), std::io::Error> {
    //     // let key: [u8; 4] = [0x7, 0x37, 0x037, 0x38];

    //     // let x = b"sp78";
    //     // // let x = String::from("sp78!").as_bytes();
    //     // // let x = "sp78".as_bytes();
    //     // // assert_eq!(x.len(), 4);

    //     // let &[a, b, c, d, _..] = x;
    //     // let four: [u8; 4] = [a, b, c, d];

    //     // let sum = atoi_via_bytes(four);
    //     assert_eq!(sum, 1936734008);
    //     Ok(())
    // }

    #[test]
    fn test_byteorder_u32() -> Result<(), std::io::Error> {
        let mut cursor = Cursor::new(my_array0());

        let result = cursor.read_u32::<BigEndian>()?;

        assert_eq!(result, 561);
        Ok(())
    }

    #[test]
    fn test_byteorder_flt() -> Result<(), std::io::Error> {
        let mut cursor = Cursor::new(my_array1());

        let result = cursor.read_f32::<BigEndian>()?;

        assert_eq!(result, 561.0);
        Ok(())
    }

    #[test]
    fn test_compose_fpe2() -> Result<(), std::io::Error> {
        let exponent = 2.0f32;
        let mantissa = -22.0f32;

        let result = compose_f32(exponent, mantissa);

        assert_eq!(result, -88.0);
        Ok(())
    }

    #[test]
    fn test_compose_sp78() -> Result<(), std::io::Error> {
        let exponent = 2.0f32;
        let mantissa = -22.0f32;

        let result = compose_f32(exponent, mantissa);

        assert_eq!(result, -88.0);
        Ok(())
    }

    // #[test]
    // fn test_as_bytes_sp78() -> Result<(), std::io::Error> {
    //     let my_array = [42, 32, 0, 0, 0];

    //     // convertedVal = (int(val[0]) << 6) + (int(val[1]) >> 2);
    //     let result: f32 = ((((my_array[0]) as i32) << 8) + (((my_array[1]) as i32) >> 8)) as f32;

    //     u16::from_be_bytes(abs) >> 8

    //     assert_eq!(result, 88.0);
    //     Ok(())
    // }

    #[test]
    fn test_shift_sp78() -> Result<(), std::io::Error> {
        let my_array = [42, 32, 0, 0, 0];

        // convertedVal = (int(val[0]) << 6) + (int(val[1]) >> 2);
        let result: f32 = ((((my_array[0]) as i32) << 8) + (((my_array[1]) as i32) >> 8)) as f32;

        assert_eq!(result, 88.0);
        Ok(())
    }

    #[test]
    fn test_shift_fp88() -> Result<(), std::io::Error> {
        let my_array = [0x01, 0x00];

        // convertedVal = (int(val[0]) << 6) + (int(val[1]) >> 2);
        let result: f32 = ((((my_array[0]) as i32) << 6) + (((my_array[1]) as i32) >> 2)) as f32;

        assert_eq!(result, 88.0);
        Ok(())
    }

    #[test]
    fn test_shift_fp2e() -> Result<(), std::io::Error> {
        let my_array = [0x12, 0xd0];

        // convertedVal = (int(val[0]) << 6) + (int(val[1]) >> 2);
        let result: f32 = ((((my_array[0]) as u16) << 8) + ((my_array[1]) as u16)) as f32 / 256.0;

        assert_eq!(result, 1204.0);
        Ok(())
    }

    #[test]
    fn test_from_bytes_u32() -> Result<(), std::io::Error> {
        let result = u32::from_be_bytes(my_array0());

        assert_eq!(result, 561);
        Ok(())
    }

    #[test]
    fn test_from_bytes_flt() -> Result<(), std::io::Error> {
        let result = u32::from_be_bytes(my_array1());

        // let result = u32::from_be_bytes(my_array) as f32;
        // let result = u32::from_ne_bytes(my_array) as f32;

        assert_eq!(result, 1204);
        Ok(())
    }

    #[test]
    fn test_from_bits() -> Result<(), std::io::Error> {
        let result = f32::from_bits(0x41480000);

        assert_eq!((26.699f32).to_bits(), 0x41D5978D);

        assert_eq!(result, 12.5);
        Ok(())
    }

    #[bench]
    fn bench_from_bytes_u32(b: &mut Bencher) {
        b.iter(|| {
            let n = black_box(1000);

            (0..n).fold(0, |_a, _b| u32::from_be_bytes(my_array0()))
        })
    }

    #[bench]
    fn bench_from_bytes_u16(b: &mut Bencher) {
        let my_array = my_array0();
        b.iter(|| {
            let n = black_box(1000);

            (0..n).fold(0, |_a, _b| getu16(my_array))
        })
    }

    #[bench]
    fn bench_from_bytes_u16_stable(b: &mut Bencher) {
        let my_array = my_array0();
        b.iter(|| {
            let n = black_box(1000);

            (0..n).fold(0, |_a, _b| getu16_stable(my_array))
        })
    }

    #[bench]
    fn bench_from_bytes_flt(b: &mut Bencher) {
        b.iter(|| {
            let n = black_box(1000);

            (0..n).fold(0, |_a, _b| u32::from_be_bytes(my_array1()))
        })
    }

    #[bench]
    fn bench_compose_f32(b: &mut Bencher) {
        let exponent = 2.0f32;
        let mantissa = -22.0f32;
        b.iter(|| {
            let n = black_box(1000);

            (0..n).fold(1.0, |_a, _b| compose_f32(exponent, mantissa))
        })
    }

    #[bench]
    fn bench_parse_f32(b: &mut Bencher) {
        b.iter(|| {
            let n = black_box(1000);

            (0..n).fold(1.0, |_a, _b| "3.14".parse::<f32>().unwrap())
        })
    }

    #[bench]
    fn bench_from_bits_f32(b: &mut Bencher) {
        b.iter(|| {
            let n = black_box(1000);

            (0..n).fold(1.0, |_a, _b| f32::from_bits(0x41480000))
        })
    }

    #[bench]
    fn bench_match_lookup(b: &mut Bencher) {
        let r = &[1, 2, 3, 4];

        b.iter(|| {
            let n = black_box(1000);

            (0..n).fold(1, |_a, _b| match r {
                &[x, y, ..] => x + y,
            })
        })
    }

    #[bench]
    fn bench_match_lookup2(b: &mut Bencher) {
        let r = get_type();

        b.iter(|| {
            let n = black_box(1000);

            (0..n).fold(0, |_a, _b| match r {
                1 => 1,
                2 => 2,
                3 => 0,
                4 => 1,
                5 => 75,
                6 => 23,
                7 => 54,
                8 => 42,
                9 => 21,
                10 => 9,
                11 => 5,
                12 => 2,
                13 => 4,
                14 => 5,
                15 => 6,
                16 => 8,
                17 => 9,
                18 => 7,
                19 => 99,
                20 => 2,
                21 => 4,
                22 => 6,
                23 => 56,
                24 => 4,
                25 => 3,
                26 => 2,
                27 => 7,
                28 => 6,
                29 => 5,
                30 => 4,
                31 => 3,
                0x41480000 => 10,
                _ => 1,
            })
        })
    }

    #[bench]
    fn bench_shift_fp88(b: &mut Bencher) {
        let my_array = [12, 64];

        b.iter(|| {
            let n = black_box(1000);

            (0..n).fold(1, |_a, _b| ((my_array[0] as u16) << 8) | my_array[1] as u16)
        })
    }

    #[bench]
    fn bench_byteorder_u32(b: &mut Bencher) {
        b.iter(|| {
            let n = black_box(1000);

            (0..n).fold(1, |_a, _b| {
                let mut cursor = Cursor::new(my_array0());

                cursor.read_u32::<BigEndian>().unwrap()
            })
        })
    }

    #[bench]
    fn bench_byteorder_f32(b: &mut Bencher) {
        b.iter(|| {
            let n = black_box(1000);

            (0..n).fold(1.0, |_a, _b| {
                let mut cursor = Cursor::new(my_array0());

                cursor.read_f32::<BigEndian>().unwrap()
            })
        })
    }

    #[bench]
    fn bench_fold(b: &mut Bencher) {
        b.iter(|| {
            let n = black_box(1000);

            (0..n).fold(1, |_a, _b| shifted_atoi_via_fold(decode()))
        })
    }

    #[bench]
    fn bench_map(b: &mut Bencher) {
        b.iter(|| {
            let n = black_box(1000);

            (0..n).fold(1, |_a, _b| shifted_atoi_via_map(decode()))
        })
    }

    #[bench]
    fn bench_loop(b: &mut Bencher) {
        b.iter(|| {
            let n = black_box(1000);

            (0..n).fold(1, |_a, _b| shifted_atoi_via_loop(decode()))
        })
    }

    // #[bench]
    // fn bench_loop(b: &mut Bencher) {
    //     b.iter(|| {
    //         for _i in 1..100 {
    //             let str_slice: &str = c_str.to_str().unwrap();
    //             black_box(shifted_atoi_via_loop(decode()));
    //         }
    // }) //note the lack of semi for return to remove optimization
    // }

    // #[bench]
    // fn bench_bytes(b: &mut Bencher) {
    //     b.iter(|| {
    //         for _i in 1..100 {
    //             black_box(atoi_via_bytes(decode2("TC0P")));
    //         }
    // }) //note the lack of semi for return to remove optimization
    // }

}
