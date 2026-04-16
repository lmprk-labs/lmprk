import { createHash } from "crypto";
import { StateProof } from "./types";
import { computeRoot, hexToBytes, bytesToHex } from "./proof";
import { PROTOCOL_NAME, PROTOCOL_VERSION } from "./index";