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