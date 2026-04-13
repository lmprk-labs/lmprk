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