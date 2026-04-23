import BN from "bn.js";
import { StateProof } from "../../sdk/typescript/src/types";
import { verifyProof } from "../../sdk/typescript/src/verify";

const proof: StateProof = {
  protocol: "lmprk/v1",
  version: 1,
  snapshot: {
    head: {
      slot: new BN(999000),
      blockhash: "00".repeat(32),
      parentSlot: new BN(998999),
      signerCount: 110,
    },
    stateRoot: "00".repeat(32),
    validatorSetSize: 128,
    threshold: 86,
  },
  address: "9xQeWvG816bUx9EPjHmaT23yvVM2ZWbrrpZb9PusVFin",
  accountData: "deadbeefcafe",
  path: { steps: [] },
};

const result = verifyProof(proof);
if (!result.valid) {
  console.error("proof rejected:", result.reason);
  process.exit(1);
}
console.log("proof accepted, computed root", result.computedRoot);
