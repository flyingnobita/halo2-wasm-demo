use halo2_proofs::pasta::EqAffine;
use halo2_proofs::plonk::{create_proof, keygen_pk, keygen_vk, verify_proof, SingleVerifier};
use halo2_proofs::poly::commitment::Params;
use halo2_proofs::transcript::{Blake2bRead, Blake2bWrite, Challenge255};
use halo2_proofs::{circuit::Value, pasta::Fp};
use js_sys::Uint8Array;
use rand_core::OsRng;
use std::io::BufReader;
use wasm_bindgen::prelude::*;
pub use wasm_bindgen_rayon::init_thread_pool;

use crate::circuits::circuits::circuit_1::Circuit1;

extern crate console_error_panic_hook;

#[wasm_bindgen]
pub fn init_panic_hook() {
    console_error_panic_hook::set_once();
}

#[wasm_bindgen]
pub async fn prove(input: u64, params_ser: JsValue) -> JsValue {
    // prepare parameters
    let params_vec = Uint8Array::new(&params_ser).to_vec();
    let params = Params::<EqAffine>::read(&mut BufReader::new(&params_vec[..])).unwrap();

    // prepare proving key
    let empty_circuit = Circuit1 {
        a: Value::unknown(),
    };
    let vk = keygen_vk(&params, &empty_circuit).expect("keygen_vk should not fail");
    let pk = keygen_pk(&params, vk.clone(), &empty_circuit).expect("keygen_pk should not fail");
    println!("Successfully generated proving key");

    // prepare circuit
    let a = Fp::from(input);
    let circuit = Circuit1 { a: Value::known(a) };

    // prepare instances
    let b = a * a;
    let public_inputs = vec![b];
    let instance_slice = [&public_inputs.clone()[..]];

    // prepare transcript
    let mut transcript = Blake2bWrite::<_, _, Challenge255<_>>::init(vec![]);

    create_proof(
        &params,
        &pk,
        &[circuit.clone()],
        &[&instance_slice],
        OsRng,
        &mut transcript,
    )
    .expect("proof generation should not fail");

    let proof: Vec<u8> = transcript.finalize();
    JsValue::from_serde(&proof).unwrap()
}

#[wasm_bindgen]
pub fn verify(input: u64, proof_js: JsValue, params_ser: JsValue) -> bool {
    // prepare parameters
    let params_vec = Uint8Array::new(&params_ser).to_vec();
    let params = Params::<EqAffine>::read(&mut BufReader::new(&params_vec[..])).unwrap();

    // prepare verifying key
    let empty_circuit = Circuit1 {
        a: Value::unknown(),
    };
    let vk = keygen_vk(&params, &empty_circuit).expect("keygen_vk should not fail");

    // prepare strategy
    let strategy = SingleVerifier::new(&params);

    // prepare instances
    let public_inputs = vec![Fp::from(input)];
    let instance_slice = [&public_inputs.clone()[..]];

    // prepare transcript
    let proof = proof_js.into_serde::<Vec<u8>>().unwrap();
    let mut transcript = Blake2bRead::<_, _, Challenge255<_>>::init(&proof[..]);

    verify_proof(&params, &vk, strategy, &[&instance_slice], &mut transcript).is_ok()
}
