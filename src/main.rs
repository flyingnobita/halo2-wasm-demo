use halo2_proofs::{circuit::Value, dev::MockProver, pasta::Fp};

use halo2_examples::circuit_1;
use halo2_examples::circuit_2;

fn test_circuit_1() {
    let k = 3;

    let a = Fp::from(2);
    let b = a * a;

    let circuit = circuit_1::Circuit1 { a: Value::known(a) };

    let public_inputs = vec![b];

    let prover = MockProver::run(k, &circuit, vec![public_inputs.clone()]).unwrap();
    prover.assert_satisfied();
    println!("public_input: {:?}", public_inputs);
}

fn test_circuit_2() {
    let k = 3;

    let a = Fp::from(2);
    let b = a * a * a;

    let circuit = circuit_2::Circuit2 { a: Value::known(a) };

    let public_inputs = vec![b];

    let prover = MockProver::run(k, &circuit, vec![public_inputs.clone()]).unwrap();
    prover.assert_satisfied();
    println!("public_input: {:?}", public_inputs);
}

fn main() {
    test_circuit_1();
    test_circuit_2();
}
