# hyprstream-synthesis

Teacher-agnostic synthetic corpus pipeline — System One program, DAG node **P1.3**.

Generates `(state, question spec, answer)` triples for decision-model
distillation: procedurally synthesized jev-1 question specs across workflow
families (`triage`, `compliance`, `extraction`, `approvals`) with
rubric/option paraphrase diversity, cardinality variation, and noul/choice/
score primitive mixes, answered by a pluggable teacher ensemble. **No human
labeling** — answers are raw per-teacher probability vectors, persisted
verbatim so corrections (fit by P0.5) can be applied at training time (P1.4).

## Guarantees

- **Mandatory augmentations (S6b1)**: full cyclic option-order closure on
  choice items + rubric/instruction paraphrase variants (always on;
  `paraphrase_variants >= 2` enforced).
- **Firewalls**: the frozen vob-1.1 benchmark manifest is a consumed,
  pin-verified artifact (release id + manifest/DISCLOSURE blake3 checked at
  load). Gate families are never synthesized; item-hash collisions fail the
  run closed.
- **Provenance + distributability**: every row records its answering
  teachers' roster pins (id, version, ToS class). Rows with any encumbered
  (`api-prohibited`) teacher are `distributable = false` — never published,
  never in Apache/MIT artifacts (`Corpus::publishable`).
- **Label-distribution control + dedup**: optional balancing over the
  corrected-ensemble argmax; canonical-hash dedup.
- **Deterministic**: same config + roster ⇒ bit-identical corpus.

## CLI

```text
hyprstream-synthesis pins                          # the consumed frozen pins
hyprstream-synthesis run --roster roster.json [--manifest path] \
    [--config config.json] [--out corpus.jsonl] [--publishable-only]
hyprstream-synthesis roster-md --roster roster.json  # DISCLOSURE table
```

The CLI drives deterministic `HashTeacher` stand-ins (roster versions pinned
`simulated:*`). Real teacher adapters implement `teacher::Teacher` in the
P0.4 eval harness / P1.4 training loop; the roster JSON is the integration
contract. The teacher roster itself is authorized by humans out-of-band and
recorded with ToS classes in `DISCLOSURE.md`.

## Standalone app — API-client dogfood

This repository is a **standalone application** that builds against the
hyprstream platform's **APIs** — it is a client, not a platform crate:

- **Decision API planes** (pluggable `Subject` clients): jev-1 HTTP/JSON
  against the P0.7 stub (`SYNTHESIS_SUBJECT_URL`, live today); the RPC
  decisions API (Cap'n Proto generated clients, with P3.1); and Flight
  SQL/ADBC Arrow batches via the P3.5 `decide()` operator.
- **Artifacts by digest**: the frozen benchmark manifest + DISCLOSURE are
  operator-supplied files verified locally by their pinned BLAKE3 digests
  (`SYNTHESIS_BENCH_ROOT`).

Zero hyprstream crates in the dependency tree. See
`EXTRACTION-REVISION.md` for the architecture correction and status.

## Licensing

Apache-2.0. Permissive-only by policy: no dependency path to any AGPL crate
(the corpus export and this harness feed the MIT-licensed flagship model
artifact). Corpus distributability additionally depends on the teacher
roster's ToS classes — see `DISCLOSURE.md`.
