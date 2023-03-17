use super::*;
use ckb_testtool::ckb_crypto::secp::Generator;
use ckb_testtool::ckb_hash::blake2b_256;
use ckb_testtool::ckb_types::{bytes::Bytes, core::TransactionBuilder, packed::*, prelude::*};
use ckb_testtool::context::Context;

const MAX_CYCLES: u64 = 10_000_000;

fn blake160(data: &[u8]) -> [u8; 20] {
    let mut buf = [0u8; 20];
    let hash = blake2b_256(data);
    buf.clone_from_slice(&hash[..20]);
    buf
}

fn gen_pubkey_hash() -> Bytes {
    let privkey = Generator::random_privkey();
    let pubkey = privkey.pubkey().expect("pubkey");
    let pubkey_hash = blake160(&pubkey.serialize());
    Bytes::copy_from_slice(&pubkey_hash)
}

#[test]
fn test_unlock_by_anyone() {
    let mut context = Context::default();
    let contract: Bytes = Loader::default().load_binary("anyone-can-pay");
    let anyone_can_pay_out_point = context.deploy_cell(contract);
    let anyone_can_pay_lock_script = context
        .build_script(&anyone_can_pay_out_point.clone(), gen_pubkey_hash())
        .expect("script");
    let lock_script_dep = CellDep::new_builder().out_point(anyone_can_pay_out_point).build();

    let input_out_point = context.create_cell(
        CellOutput::new_builder()
            .capacity(1000u64.pack())
            .lock(anyone_can_pay_lock_script.clone())
            .build(),
        Bytes::new(),
    );
    let input = CellInput::new_builder()
        .previous_output(input_out_point)
        .build();

    // outputs
    let outputs = vec![CellOutput::new_builder()
        .capacity(1000u64.pack())
        .build()];

    let outputs_data = vec![Bytes::new()];

    let tx = TransactionBuilder::default()
        .input(input)
        .outputs(outputs)
        .cell_dep(lock_script_dep)
        .outputs_data(outputs_data.pack())
        .build();

    let tx = context.complete_tx(tx);

    context
        .verify_tx(&tx, MAX_CYCLES)
        .expect("pass verification");
}
