// Import from `core` instead of from `std` since we are in no-std mode
use core::result::Result;

// Import heap related library from `alloc`
// https://doc.rust-lang.org/alloc/index.html

// Import CKB syscalls and structures
// https://docs.rs/ckb-std/

use alloc::string::{ToString, String};
use ckb_std::syscalls::debug;

use crate::{error::Error, anyone_can_pay::read_args};

pub fn main() -> Result<(), Error> {
    let (pubkey_hash, min_ckb_amount, min_udt_amount) = read_args().unwrap();
    Ok(())
}
