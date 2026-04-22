import { computeRoot, emptyPath, hexToBytes, bytesToHex, pathDepth } from "../src/proof";
import { hashLeaf, hashNode } from "../src/verify";

describe("merkle helpers", () => {
  it("empty path returns the leaf hash", () => {
    const leaf = hashLeaf(new Uint8Array([1, 2, 3]));
    const root = computeRoot(emptyPath(), leaf);
    expect(bytesToHex(root)).toBe(bytesToHex(leaf));
  });

  it("depth zero for empty path", () => {
    expect(pathDepth(emptyPath())).toBe(0);
  });

  it("hex round trip works", () => {
    const bytes = new Uint8Array([0xde, 0xad, 0xbe, 0xef]);
    expect(bytesToHex(bytes)).toBe("deadbeef");
    expect(Array.from(hexToBytes("deadbeef"))).toEqual([0xde, 0xad, 0xbe, 0xef]);
  });

  it("two leaves under one node combine deterministically", () => {
    const a = hashLeaf(new Uint8Array([1]));
    const b = hashLeaf(new Uint8Array([2]));
    const left = hashNode(a, b);
    const right = hashNode(a, b);
    expect(bytesToHex(left)).toBe(bytesToHex(right));
  });
});
