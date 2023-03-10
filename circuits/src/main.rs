use std::fs::File;
use std::io;

use halo2_proofs::pasta::EqAffine;
use halo2_proofs::plonk::{create_proof, keygen_pk, keygen_vk, verify_proof, SingleVerifier};
use halo2_proofs::poly::commitment::Params;
use halo2_proofs::transcript::{Blake2bRead, Blake2bWrite, Challenge255};
use halo2_proofs::{circuit::Value, dev::MockProver, pasta::Fp};

use circuits::circuits::circuits::circuit_1::Circuit1;
use circuits::circuits::circuits::circuit_2::Circuit2;
use rand_core::OsRng;

fn run_circuit_1(input_int: u64) {
    let k = 3;
    let params: Params<EqAffine> = Params::new(k);

    // prepare proving key
    let empty_circuit = Circuit1 {
        a: Value::unknown(),
    };
    let vk = keygen_vk(&params, &empty_circuit).expect("keygen_vk should not fail");
    let pk = keygen_pk(&params, vk.clone(), &empty_circuit).expect("keygen_pk should not fail");
    println!("Successfully generated proving key");

    // prepare circuit
    let a = Fp::from(input_int);
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
    println!("proof: {:?}", proof);

    // Verify Proof

    // prepare instances
    let public_inputs = vec![Fp::from(input_int * input_int)];
    let instance_slice = [&public_inputs.clone()[..]];

    // prepare strategy
    let strategy = SingleVerifier::new(&params);

    // prepare transcript
    let mut transcript_verifier = Blake2bRead::<_, _, Challenge255<_>>::init(&proof[..]);

    let verified = verify_proof(
        &params,
        &vk,
        strategy,
        &[&instance_slice],
        &mut transcript_verifier,
    )
    .is_ok();

    println!("Verification: {:?}", verified);
}

fn run_circuit_1_mock(input_int: u64) {
    let k = 3;

    let a = Fp::from(input_int);
    let b = a * a;

    let circuit = Circuit1 { a: Value::known(a) };

    let public_inputs = vec![b];

    let prover = MockProver::run(k, &circuit, vec![public_inputs.clone()]).unwrap();
    prover.assert_satisfied();
    println!("Circuit 1 Verified Output: {:?}", public_inputs);
}

fn run_circuit_2_mock(input_int: u64) {
    let k = 3;

    let a = Fp::from(input_int);
    let b = a * a * a;

    let circuit = Circuit2 { a: Value::known(a) };

    let public_inputs = vec![b];

    let prover = MockProver::run(k, &circuit, vec![public_inputs.clone()]).unwrap();
    prover.assert_satisfied();
    println!("Circuit 2 Verified Output: {:?}", public_inputs);
}

fn get_user_integer() -> u64 {
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

fn get_user_command() -> u64 {
    let mut input_string = String::new();
    let user_prompt_str = "Press: 
    0 - generate parameter file (params.bin)
    1 - run circuit 1 with full prover and verifier
    2 - run circuit 1 with MockProver
    3 - run circuit 2 with MockProver
    9 - quit";
    println!("{}", &user_prompt_str);
    io::stdin()
        .read_line(&mut input_string)
        .expect("input failed");

    let input_command: u64;
    match input_string.trim().parse::<u64>() {
        Ok(i) => input_command = i,
        Err(e) => {
            println!("Error - incorrect input: {}", e);
            input_command = 0
        }
    };

    // println!("User inputed: {}", input_command);
    input_command
}

fn write_params() {
    const K: u32 = 14;
    let params_filename = "params.bin".to_string();
    println!("Writing {}", params_filename);
    let mut params_file = File::create(&params_filename).unwrap();
    let params: Params<EqAffine> = Params::new(K);
    params.write(&mut params_file).unwrap();
    println!("Finished writing {}", params_filename);
}

fn main() {
    loop {
        let input_command = get_user_command();
        match input_command {
            0 => write_params(),
            1 => {
                let input_int = get_user_integer();
                run_circuit_1(input_int)
            }
            2 => {
                let input_int = get_user_integer();
                run_circuit_1_mock(input_int)
            }
            3 => {
                let input_int = get_user_integer();
                run_circuit_2_mock(input_int)
            }
            9 => break,
            _ => println!("Command not recognize"),
        }
    }
}
