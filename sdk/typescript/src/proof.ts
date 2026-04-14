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