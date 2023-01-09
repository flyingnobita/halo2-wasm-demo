import React, { useEffect, useState } from "react";
// import "./App.css";
import { wrap } from "comlink";

import {
  Body,
  Container,
  Title,
  Link,
  LinkLogoContainer,
  LinkLogo,
  DivScrollable,
  Pre,
  Details,
  Summary,
  ZkDetails,
  DivLeftAlign,
  DetailButton,
  DivFlexInputContainer,
  DivFlexInput,
  DivFlexFormContainer,
  DivFlexForm,
  ZKDetailStatus,
} from "./components";

import githubLogo from "./assets/images/GitHub-Mark-120px-plus.png";

function App() {
  const [zkStatus, setZkStatus] = useState("");
  const [threadPoolSize, setThreadPoolSize] = useState("1");
  const [proverInput, setProverInput] = useState("2");
  const [verifierInput, setVerifierInput] = useState("4");
  const [proof, setProof] = useState(null);

  useEffect(() => {
    setThreadPoolSize(navigator.hardwareConcurrency.toString());
  }, []);

  const showZkStatus = (inputStatus: string) => {
    setZkStatus(inputStatus);
  };

  const worker = new Worker(new URL("./halo-worker", import.meta.url), {
    name: "halo-worker",
    type: "module",
  });
  const workerApi = wrap<import("./halo-worker").HaloWorker>(worker);

  async function prove() {
    showZkStatus("Generating proof...");
    console.log("proverInput: " + proverInput);
    const start = performance.now();
    console.log("Starting prove()...");
    const proof = await workerApi.prove(
      BigInt(proverInput),
      parseInt(threadPoolSize)
    );
    setProof(proof);
    const prove_finish = performance.now();
    const t_prove = prove_finish - start;
    console.log("Time to prove (s): ", t_prove / 1000);
    console.log("test.prove(): ", proof);
    // showZkStatus("test.prove(): " + proof);
    showZkStatus("Proof successfully generated");
    return proof;
  }

  async function verify() {
    showZkStatus("Verifying proof...");
    console.log("verifierInput: " + verifierInput);
    const verification = await workerApi.verify(
      BigInt(verifierInput),
      proof,
      parseInt(threadPoolSize)
    );
    const verify_finish = performance.now();
    // const t_verify = verify_finish - prove_finish;
    // console.log("Time to verify (s): ", t_verify / 1000);
    console.log("Verification: ", verification);
    showZkStatus("Verification: " + String(verification));
  }

  async function handleButtonProve(event: React.ChangeEvent<HTMLInputElement>) {
    event.preventDefault();
    prove();
  }

  async function handleButtonVerify(
    event: React.ChangeEvent<HTMLInputElement>
  ) {
    event.preventDefault();
    verify();
  }

  const handleProverInputChange = (
    event: React.ChangeEvent<HTMLInputElement>
  ) => {
    event.persist();
    const re = /^[0-9\b]+$/;
    if (event.target.value === "" || re.test(event.target.value)) {
      setProverInput(event.target.value);
    }
  };

  const handleVerifierInputChange = (
    event: React.ChangeEvent<HTMLInputElement>
  ) => {
    event.persist();
    const re = /^[0-9\b]+$/;
    if (event.target.value === "" || re.test(event.target.value)) {
      setVerifierInput(event.target.value);
    }
  };

  const handleThreadPoolSizeChange = (
    event: React.ChangeEvent<HTMLInputElement>
  ) => {
    event.persist();
    const re = /^[0-9\b]+$/;
    if (event.target.value === "" || re.test(event.target.value)) {
      setThreadPoolSize(event.target.value);
    }
  };

  return (
    <div className="App">
      <Container>
        <Body>
          <Title>halo2</Title>
          <DivLeftAlign>
            <Details>
              <Summary>Prove You Know The Square</Summary>
              <ZkDetails>
                <DivFlexInputContainer>
                  <label>
                    Thread Pool Size (Change to 4 if running on Apple M1):{" "}
                  </label>
                  <DivFlexInput
                    type="number"
                    value={threadPoolSize}
                    onChange={handleThreadPoolSizeChange}
                  />
                </DivFlexInputContainer>
                <h2>Prover</h2>
                <DivFlexFormContainer>
                  <DivFlexForm onSubmit={handleButtonProve}>
                    <DivFlexInputContainer>
                      <label>Integer: </label>
                      <DivFlexInput
                        type="text"
                        value={proverInput}
                        onChange={handleProverInputChange}
                      />
                    </DivFlexInputContainer>
                    <DetailButton type="submit" value="Prove">
                      Prove
                    </DetailButton>
                  </DivFlexForm>
                </DivFlexFormContainer>
                <h2>Verifier</h2>
                <DivFlexInputContainer>
                  <label>Square: </label>
                  <DivFlexInput
                    type="text"
                    value={verifierInput}
                    onChange={handleVerifierInputChange}
                  />
                </DivFlexInputContainer>
                <h3>Proof</h3>
                {proof != null && (
                  <DivScrollable>
                    <Pre>{JSON.stringify(proof, null, 2)}</Pre>
                  </DivScrollable>
                )}
                <DetailButton onClick={handleButtonVerify}>Verify</DetailButton>
                <ZKDetailStatus>{zkStatus}</ZKDetailStatus>
              </ZkDetails>
            </Details>
          </DivLeftAlign>
          <LinkLogoContainer>
            <Link href="https://github.com/flyingnobita/halo2-playground">
              <LinkLogo src={githubLogo} alt="github" />
            </Link>
          </LinkLogoContainer>
        </Body>
      </Container>
    </div>
  );
}

export default App;
