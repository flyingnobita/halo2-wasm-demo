import { expose } from 'comlink';

async function prove(input: bigint) {
    // prepare parameters
    const params = await fetch_params();
    console.log("param length", params.length);
    console.log("params", params);

    // start proving
    const multiThread = await import('halo2_playground');
    await multiThread.default();
    // await multiThread.initThreadPool(navigator.hardwareConcurrency);  // Intel
    await multiThread.initThreadPool(4);  // M1
    multiThread.init_panic_hook();
    const ret = multiThread.prove(input, params);
    return ret;
}

async function verify(input: bigint, proof: any) {
    // prepare parameters
    const params = await fetch_params();
    console.log("param length", params.length);
    console.log("params", params);

    // start verification
    const multiThread = await import('halo2_playground');
    await multiThread.default();
    // await multiThread.initThreadPool(navigator.hardwareConcurrency);  // Intel
    await multiThread.initThreadPool(4);  // M1
    console.log("Starting verify()...")
    const ret = multiThread.verify(input, proof, params);
    return ret;
}

async function fetch_params() {
    const response = await fetch('http://localhost:3000/params.bin');
    const bytes = await response.arrayBuffer();
    const params = new Uint8Array(bytes);
    return params;
}

const exports = {
    prove,
    verify
};
export type HaloWorker = typeof exports;
expose(exports);
