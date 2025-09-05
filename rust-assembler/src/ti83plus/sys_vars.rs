use phf::phf_map;

pub static SYS_VARS: phf::Map<&'static str, u16> = phf_map! {
    "curRow" => 0x844B,
    "curCol" => 0x844C,
    "penCol" => 0x86D7,
    "penRow" => 0x86D8,
    "OP1" => 0x8478,
    "OP2" => 0x8483,
    "OP3" => 0x848E,
    "OP4" => 0x8499,
    "OP5" => 0x84A4,
    "OP6" => 0x84AF,
    "flags" => 0x89F0,
};
