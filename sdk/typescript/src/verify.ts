import { createHash } from "crypto";
import { StateProof } from "./types";
import { computeRoot, hexToBytes, bytesToHex } from "./proof";
import { PROTOCOL_NAME, PROTOCOL_VERSION } from "./index";

const LEAF_DOMAIN = Buffer.from("lmprk:leaf:v1");
const NODE_DOMAIN = Buffer.from("lmprk:node:v1");