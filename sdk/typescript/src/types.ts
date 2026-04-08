import BN from "bn.js";

export type HexString = string;
export type Base58 = string;

export interface SlotInfo {
  slot: BN;
  blockhash: HexString;
  parentSlot: BN;
  signerCount: number;
}

export interface SlotSnapshot {
  head: SlotInfo;
  stateRoot: HexString;
  validatorSetSize: number;
  threshold: number;
}

export type MerkleSide = "left" | "right";

export interface MerkleStep {
  sibling: HexString;
  side: MerkleSide;
}

export interface MerklePath {
  steps: MerkleStep[];
}

export interface StateProof {
  protocol: string;
  version: number;
  snapshot: SlotSnapshot;
  address: Base58;
  accountData: HexString;
  path: MerklePath;
}
