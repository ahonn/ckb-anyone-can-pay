#![no_std]

use alloc::vec;
use ckb_std::ckb_constants::Source;
use ckb_std::high_level::{load_script, load_tx_hash};
use ckb_std::syscalls::SysError;
use secp256k1::{
  Secp256k1,
  ecdsa::Signature,
};

const BLAKE160_SIZE: usize = 20;

pub enum Error {
    Syscall(SysError),
    Native(AnyoneCanPayError),
}

pub enum AnyoneCanPayError {}

pub fn read_args() -> Result<([u8; BLAKE160_SIZE], u64, u128), SysError> {
    let script = load_script()?;
    let args = script.args().raw_data();

    let mut pubkey_hash = [0u8; BLAKE160_SIZE];
    let mut min_ckb_amount = 0u64;
    let mut min_udt_amount = 0u128;
    let args_len = args.len();
    if args_len >= BLAKE160_SIZE {
        pubkey_hash.copy_from_slice(&args[0..BLAKE160_SIZE]);
    }
    if args_len >= BLAKE160_SIZE + 1 {
        let x = args[BLAKE160_SIZE] as u64;
        min_ckb_amount = 10u64.pow(x as u32);
    }
    if args_len >= BLAKE160_SIZE + 2 {
        let x = args[BLAKE160_SIZE + 1] as u128;
        min_udt_amount = 10u128.pow(x as u32);
    }

    Ok((pubkey_hash, min_ckb_amount, min_udt_amount))
}
