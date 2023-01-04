use wasm_bindgen::prelude::*;
pub use wasm_bindgen_rayon::init_thread_pool;

use crate::circuits::circuits::circuit_1::Circuit1;
use halo2_proofs::{circuit::Value, pasta::Fp};

extern crate console_error_panic_hook;

#[wasm_bindgen]
pub fn init_panic_hook() {
    console_error_panic_hook::set_once();
}

#[wasm_bindgen]
pub async fn prove(params_ser: JsValue) -> JsValue {
    let a = Fp::from(3);
    let circuit = Circuit1 { a: Value::known(a) };

    JsValue::from_str("ok!")
}
