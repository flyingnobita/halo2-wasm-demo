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
    const t_prove = performance.now() - start;
    console.log("t_prove: ", t_prove);
    console.log("test.prove(): ", proof);

    // Verifying
    const verification = await workerApi.verify(BigInt(2), proof);
    const t_verify_play = performance.now() - t_prove;
    console.log("t_verify_play: ", t_verify_play);
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
