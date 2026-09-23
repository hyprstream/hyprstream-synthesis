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

## Standalone app — dogfooding the platform

This repository is a **standalone application**, not a platform crate: it
*consumes* the hyprstream platform and is the dogfood proof that the
platform's published contracts are sufficient to build on. Exactly two
platform contracts cross the boundary:

- `hyprstream-decision` — the jev-1 decision IR types,
- `hyprstream-bench` — the frozen vob-1.1 manifest + DISCLOSURE pins.

Default layout is a sibling checkout (path dependencies resolve to
`../hyprstream/crates/...`):

```
parent/
├── hyprstream/              # the platform (github.com/hyprstream/hyprstream)
└── hyprstream-synthesis/    # this app
```

Other layouts: set `SYNTHESIS_BENCH_ROOT` to the platform's
`crates/hyprstream-bench` directory for the firewall pins, and adjust the
path dependencies in `Cargo.toml` for the two platform libraries. CI checks
the platform out at `hyprstream/` inside the workspace, which matches the
default layout.

## Licensing

Apache-2.0. Permissive-only by policy: no dependency path to any AGPL crate
(the corpus export and this harness feed the MIT-licensed flagship model
artifact). Corpus distributability additionally depends on the teacher
roster's ToS classes — see `DISCLOSURE.md`.
