import BN from "bn.js";
import { StateProof } from "../src/types";
import { verifyProof } from "../src/verify";

describe("verifyProof", () => {
  function buildProof(overrides: Partial<StateProof> = {}): StateProof {
    return {
      protocol: "lmprk/v1",
      version: 1,
      snapshot: {
        head: {
          slot: new BN(1000),
          blockhash: "00".repeat(32),
          parentSlot: new BN(999),
          signerCount: 100,
        },
        stateRoot: "00".repeat(32),
        validatorSetSize: 128,
        threshold: 86,
      },
      address: "So11111111111111111111111111111111111111112",
      accountData: "deadbeef",
      path: { steps: [] },
      ...overrides,
    };
  }

  it("rejects mismatched protocol", () => {
    const result = verifyProof(buildProof({ protocol: "wrong/v0" }));
    expect(result.valid).toBe(false);
    expect(result.reason).toBe("protocol mismatch");
  });

  it("rejects mismatched version", () => {
    const result = verifyProof(buildProof({ version: 99 }));
    expect(result.valid).toBe(false);
    expect(result.reason).toBe("version mismatch");
  });

  it("rejects insufficient signatures", () => {
    const proof = buildProof();
    proof.snapshot.head.signerCount = 10;
    const result = verifyProof(proof);
    expect(result.valid).toBe(false);
    expect(result.reason).toBe("insufficient signatures");
  });

  it("reports computed root on mismatch", () => {
    const result = verifyProof(buildProof());
    expect(result.valid).toBe(false);
    expect(result.reason).toBe("root mismatch");
    expect(result.computedRoot.length).toBe(64);
  });
});
