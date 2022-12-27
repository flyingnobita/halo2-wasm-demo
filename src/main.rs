use std::io;

use halo2_proofs::{circuit::Value, dev::MockProver, pasta::Fp};

use halo2_playground_circuits::circuit_1;
use halo2_playground_circuits::circuit_2;

fn run_circuit_1(input_num: u64) {
    let k = 3;

    let a = Fp::from(input_num);
    let b = a * a;

    let circuit = circuit_1::Circuit1 { a: Value::known(a) };

    let public_inputs = vec![b];

    let prover = MockProver::run(k, &circuit, vec![public_inputs.clone()]).unwrap();
    prover.assert_satisfied();
    println!("Circuit 1 Proven Output: {:?}", public_inputs);
}

fn run_circuit_2(input_num: u64) {
    let k = 3;

    let a = Fp::from(input_num);
    let b = a * a * a;

    let circuit = circuit_2::Circuit2 { a: Value::known(a) };

    let public_inputs = vec![b];

    let prover = MockProver::run(k, &circuit, vec![public_inputs.clone()]).unwrap();
    prover.assert_satisfied();
    println!("Circuit 2 Proven Output: {:?}", public_inputs);
}

fn main() {
    let input = get_user_input();

    run_circuit_1(input);
    run_circuit_2(input);
}
fn get_user_input() -> u64 {
    let mut input_string = String::new();
    println!("Enter a positive integer");
    io::stdin()
        .read_line(&mut input_string)
        .expect("input failed");

    let input_int: u64;
    match input_string.trim().parse::<u64>() {
        Ok(i) => input_int = i,
        Err(e) => {
            println!("Error: {}", e);
            input_int = 0
        }
    };

    println!("User inputed: {}", input_int);
    input_int
}
