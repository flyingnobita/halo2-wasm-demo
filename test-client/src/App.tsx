import React from "react";
import "./App.css";
import { wrap } from "comlink";

function App() {
  const worker = new Worker(new URL("./halo-worker", import.meta.url), {
    name: "halo-worker",
    type: "module",
  });
  const workerApi = wrap<import("./halo-worker").HaloWorker>(worker);

  async function test() {
    console.log(
      "navigator.hardwareConcurrency: ",
      navigator.hardwareConcurrency
    );

    // Proving
    const start = performance.now();
    const proof = await workerApi.prove(BigInt(2));
    const prove_finish = performance.now();
    const t_prove = prove_finish - start;
    console.log("Time to prove (s): ", t_prove / 1000);
    console.log("test.prove(): ", proof);

    // Verifying
    const verification = await workerApi.verify(BigInt(2), proof);
    const verify_finish = performance.now();
    const t_verify = verify_finish - prove_finish;
    console.log("Time to verify (s): ", t_verify / 1000);
    console.log("Verification", verification);
  }

  return (
    <div className="App">
      <header className="App-header">
        <button onClick={test}>Start</button>
      </header>
    </div>
  );
}

export default App;
