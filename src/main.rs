extern crate rustc_serialize;

pub struct PS {
    w: u8,
    d: u8,
    m: i64,
    u: [i64; 10],
    r: u8,
    a: u8,
}

fn main() {
    let info = ["NORX.init: Parameter R set to default 4.",
                "NORX.init: Parameter A set to default 4."];

    let error = ["NORX: Uninitialized.",
                 "NORX.init: Initialization failed.",
                 "NORX.encrypt: Invalid key size.",
                 "NORX.encrypt: Invalid nonce size.",
                 "NORX.decrypt: Invalid key size.",
                 "NORX.decrypt: Invalid nonce size.",
                 "NORX.decrypt: Decryption failed."];

    let params = PS {
        w: 32,
        d: 1,
        m: 0xFFFFFFFF,
        u: [
		        0x243F6A88, 0x85A308D3,
		        0x13198A2E, 0x03707344,
		        0x254F537A, 0x38531D48,
		        0x839C6E83, 0xF97A3AE5,
		        0x8C91D88C, 0x11EAFB59,
	      ],
        r: 0,
        a: 0,
    };
}
