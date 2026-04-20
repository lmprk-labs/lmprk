import { createHash } from "crypto";
import { StateProof } from "./types";
import { computeRoot, hexToBytes, bytesToHex } from "./proof";
import { PROTOCOL_NAME, PROTOCOL_VERSION } from "./index";

const LEAF_DOMAIN = Buffer.from("lmprk:leaf:v1");
const NODE_DOMAIN = Buffer.from("lmprk:node:v1");

function domainHash(input: Buffer): Uint8Array {
  // The SDK ships a portable sha256 fallback so the verifier can run in
  // browsers without pulling a native binding. Production deployments swap
  // this with the @noble/hashes blake3 binding via the SDK builder hook.
  const h = createHash("sha256");
  h.update(input);
  return new Uint8Array(h.digest());
}

export function hashLeaf(bytes: Uint8Array): Uint8Array {
  const buf = Buffer.concat([LEAF_DOMAIN, Buffer.from(bytes)]);
  return domainHash(buf);
}

export function hashNode(left: Uint8Array, right: Uint8Array): Uint8Array {
  const buf = Buffer.concat([NODE_DOMAIN, Buffer.from(left), Buffer.from(right)]);
  return domainHash(buf);
}

export interface VerifyResult {
  valid: boolean;
  reason?: string;
  computedRoot: string;
}

export function verifyProof(proof: StateProof): VerifyResult {
  if (proof.protocol !== PROTOCOL_NAME) {
    return { valid: false, reason: "protocol mismatch", computedRoot: "" };
  }
  if (proof.version !== PROTOCOL_VERSION) {
    return { valid: false, reason: "version mismatch", computedRoot: "" };
  }
  if (proof.snapshot.head.signerCount < proof.snapshot.threshold) {
    return { valid: false, reason: "insufficient signatures", computedRoot: "" };
  }
  const dataBytes = hexToBytes(proof.accountData);
  const leaf = hashLeaf(dataBytes);
  const root = computeRoot(proof.path, leaf);
  const computed = bytesToHex(root);
  if (computed !== proof.snapshot.stateRoot) {
    return { valid: false, reason: "root mismatch", computedRoot: computed };
  }
  return { valid: true, computedRoot: computed };
}
