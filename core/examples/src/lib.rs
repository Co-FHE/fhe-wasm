use fwasm_sdk::decrypt;
use fwasm_sdk::encrypt;
use fwasm_sdk::init;
use fwasm_sdk::operation;
use t::{D, E};

pub mod t {
    use codec::{Decode, Encode};

    #[derive(Debug, Decode)]
    pub struct E {
        pub a: Vec<u32>,
        pub b: i32,
        pub c: u32,
    }

    #[derive(Debug, Encode)]
    pub struct D {
        pub b: i32,
    }
}

// #[init]
// pub fn init(e: t::E, u: u32) {
// }

// #[encrypt]
// pub fn encrypt(e: E) -> Result<D, ()> {
//     storage::encrypt(b"hello", b"world").map_err(|_| ())?;
//     Ok(D { b: 1 })
// }

// #[encrypt("encryptOne")]
// pub fn encrypt1(e: E) {
//     storage::encrypt(b"hello", b"world").unwrap();
// }

// #[decrypt]
// pub fn decrypt(e: E) -> D {
//     storage::encrypt(b"hello", b"world").unwrap();
//     D { b: 1 }
// }
