#![feature(int_to_from_bytes)]
#![feature(slice_patterns)]

use regex::Regex;

fn main() {
    let keys = [
        "TCXC", "TC0P", "TC0H", "TC0D", "TC0E", "TC0F", "TC1C", "TC2C", "TC3C", "TC4C", "TC5C",
        "TC6C", "TC7C", "TC8C", "TCAH", "TCAD", "TC1P", "TC1H", "TC1D", "TC1E", "TC1F", "TCBH",
        "TCBD", "TCSC", "TCSA", "TCGC", "TG0P", "TG0D", "TG1D", "TG0H", "TG1H", "TS0S", "TM0P",
        "TM1P", "TM8P", "TM9P", "TM0S", "TM1S", "TM8S", "TM9S", "TN0D", "TN0P", "TN1P", "TN0C",
        "TN0H", "TP0D", "TPCD", "TP0P", "TA0P", "TA1P", "TH0H", "TH1H", "TH2H", "TS0P", "TB0P",
        "TL0P", "TW0P", "TH0P", "TH1P", "TH2P", "TH3P", "TO0P", "TB0T", "TB1T", "TB2T", "TB3T",
        "TP0C", "TP1P", "TP1C", "TP2P", "TP3P", "TP4P", "TP5P", "TS0C", "TA0S", "TA1S", "TA2S",
        "TA3S", "VC0C", "VC1C", "VC2C", "VC3C", "VC4C", "VC5C", "VC6C", "VC7C", "VV1R", "VG0C",
        "VM0R", "VN1R", "VN0C", "VD0R", "VD5R", "VP0R", "VP0C", "VV2S", "VR3R", "VV1S", "VH05",
        "VV9S", "VD2R", "VV7S", "VV3S", "VV8S", "VEES", "VBAT", "VB0R", "IC0C", "IC1C", "IC2C",
        "IC0R", "IC5R", "IC8R", "IC0G", "IC0M", "IG0C", "IM0C", "IM0R", "IN0C", "ID0R", "ID5R",
        "IO0R", "IB0R", "IPBR", "PC0C", "PC1C", "PC2C", "PC3C", "PC4C", "PC5C", "PC6C", "PC7C",
        "PCPC", "PCPG", "PCPD", "PCTR", "PCPL", "PC1R", "PC5R", "PGTR", "PG0R", "PM0R", "PN0C",
        "PN1R", "PC0R", "PD0R", "PD5R", "PH02", "PH05", "PP0R", "PD2R", "PO0R", "PBLC", "PB0R",
        "PDTR", "PSTR",
    ];

    println!("pub enum Key {{");

    for (_i, key) in keys.iter().enumerate() {
        let byte_array_ref = key.as_bytes();
        let byte_array: [u8; 4] = [
            byte_array_ref[0],
            byte_array_ref[1],
            byte_array_ref[2],
            byte_array_ref[3],
        ];
        let num = u32::from_be_bytes(byte_array);

        println!("{} = {:?},", key, num);
    }
    println!("}}");

    let types = [
        "fp5b", "fpa6", "fpc4", "sp1e", "sp3c", "sp4b", "sp96", "spb4", "spf0", "{pwm", "ui32",
        "ui8 ", "flag", "ui16", "hex_", "ch8*", "fp88", "{ali", "{alp", "{alc", "fp1f", "{alv",
        "si16", "sp87", "sp78", "flt ", "sp5a", "si8 ", "{clc", "{clh", "{hdi", "{lim", "{lkb",
        "{lks", "fpe2", "{fds", "fp79", "fp6a", "{mss", "rev ", "char",
    ];

    println!("static TYPES: phf::Map<u32, Type> = phf_map! {{");

    for (_i, type_) in types.iter().enumerate() {
        let byte_array_ref = type_.as_bytes();
        let byte_array: [u8; 4] = [
            byte_array_ref[0],
            byte_array_ref[1],
            byte_array_ref[2],
            byte_array_ref[3],
        ];
        let num = u32::from_be_bytes(byte_array);

        let re = Regex::new(r"[{*_\s]").unwrap();
        let result = re.replace_all(type_, "");

        println!("{}u32 => Type::{},", num, result);
    }
    println!("}};");

    println!("pub enum Type {{");

    for (_i, type_) in types.iter().enumerate() {
        let re = Regex::new(r"[{*_\s]").unwrap();
        let result = re.replace_all(type_, "");

        println!("{},", result);
    }
    println!("}}");
}
