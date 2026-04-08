import { MerklePath, MerkleStep, StateProof } from "./types";
import { hashNode } from "./verify";

export function emptyPath(): MerklePath {
  return { steps: [] };
}