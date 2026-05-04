# Proof format

A `StateProof` is the wire-level bundle carried between the host backend and
clients. It is a struct of four pieces:

| Field | Purpose |
|-------|---------|
| `protocol` + `version` | Header used to refuse unknown formats. |
| `snapshot` | The slot snapshot the proof anchors against. |
| `address` + `account_data` | The account being proven. |
| `path` | The merkle inclusion path from the account leaf to the snapshot root. |

## Verification order

The verifier runs the checks in a fixed order to keep error messages stable:

1. `protocol` and `version` must match the consumer's expectations.
2. `snapshot.signer_count` must meet `snapshot.threshold`.
3. The merkle path must reproduce `snapshot.state_root` from the account leaf.