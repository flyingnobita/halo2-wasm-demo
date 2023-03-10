import { expose } from 'comlink';

async function prove(input: bigint, threadPoolSize: number) {
    // prepare parameters
    const params = await fetch_params();
    console.log("param length", params.length);
    console.log("params", params);

    // start proving
    const multiThread = await import('halo2-wasm-demo');
    await multiThread.default();
    await multiThread.initThreadPool(threadPoolSize);
    multiThread.init_panic_hook();
    const ret = multiThread.prove(input, params);
    return ret;
}

async function verify(input: bigint, proof: any, threadPoolSize: number) {
    // prepare parameters
    const params = await fetch_params();
    console.log("param length", params.length);
    console.log("params", params);

    // start verification
    const multiThread = await import('halo2-wasm-demo');
    await multiThread.default();
    await multiThread.initThreadPool(threadPoolSize);
    console.log("Starting verify()...")
    const ret = multiThread.verify(input, proof, params);
    return ret;
}

async function fetch_params() {

    const params_file = "https://raw.githubusercontent.com/flyingnobita/halo2-wasm-demo/master/react_app/public/params.bin"

    const response = await fetch(params_file);
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
