import { MerklePath, MerkleStep, StateProof } from "./types";
import { hashNode } from "./verify";

export function emptyPath(): MerklePath {
  return { steps: [] };
}

export function pathDepth(path: MerklePath): number {
  return path.steps.length;
}

export function foldStep(acc: Uint8Array, step: MerkleStep): Uint8Array {
  const sib = hexToBytes(step.sibling);
  if (step.side === "left") {
    return hashNode(sib, acc);
  }
  return hashNode(acc, sib);
}

export function computeRoot(path: MerklePath, leaf: Uint8Array): Uint8Array {
  let acc = leaf;
  for (const step of path.steps) {
    acc = foldStep(acc, step);
  }
  return acc;
}

export function proofByteSize(proof: StateProof): number {
  return 8 + 96 + proof.accountData.length / 2 + proof.path.steps.length * 33;
}

export function hexToBytes(hex: string): Uint8Array {
  const clean = hex.startsWith("0x") ? hex.slice(2) : hex;
  if (clean.length % 2 !== 0) {
    throw new Error("hex string has odd length");
  }
  const out = new Uint8Array(clean.length / 2);
  for (let i = 0; i < out.length; i++) {
    out[i] = parseInt(clean.slice(i * 2, i * 2 + 2), 16);
  }
  return out;
}

export function bytesToHex(bytes: Uint8Array): string {
  let hex = "";
  for (const b of bytes) {
    hex += b.toString(16).padStart(2, "0");
  }
  return hex;
}
