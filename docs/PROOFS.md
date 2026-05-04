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