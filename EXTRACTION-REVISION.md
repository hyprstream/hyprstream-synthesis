# Extraction revision — API-client dogfood (in progress)

**Correction (owner, 2026-09-22):** dogfooding means building against the
platform's **APIs**, not statically linking internal platform crates via
path dependencies. The initial extraction (commit 1f308c6) kept
`hyprstream-decision` + `hyprstream-bench` as path deps — that is the
monolith with extra steps and is being reworked.

## Corrected architecture

Zero platform crates in the dependency tree. The app is a pure **API
client** of the platform:

1. **Decision API (the dogfood surface).** Question authoring and answering
   speak the jev-1 wire JSON against the platform's decision service — the
   P0.7 stub today (`POST /v1/systemone`), the InferenceService decision
   surface (P3.1) later. `src/subject.rs` is the HTTP client
   (`SYNTHESIS_SUBJECT_URL`, default `http://127.0.0.1:8080`); the
   deterministic `HashTeacher` stays for reproducible tests.
2. **Artifacts by digest.** The frozen benchmark manifest + DISCLOSURE are
   consumed as *files supplied by the operator* (path or URL) and verified
   locally by their pinned BLAKE3 digests — no platform crate needed to
   check a hash. (`SYNTHESIS_BENCH_ROOT`; a URL fetcher is a follow-up.)
3. **Wire DTOs owned here.** The app defines its own serde model of the
   jev-1 question/answer JSON (`src/wire.rs`) — exactly like any API
   client defines DTOs from the service's schema. Schema compatibility is
   proven against the stub, not against the platform's Rust types.

## Status

- [x] `src/rng.rs` — splitmix64 PRNG ported verbatim (determinism contract).
- [x] `src/manifest.rs` — manifest subset + blake3_hex + `RELEASE` ported.
- [ ] `src/wire.rs` — local jev-1 DTOs (Entry, QuestionSpec, QuestionBody,
      QuestionKind, NoulCriteria, ChoiceOption, QuestionSet; labels(),
      cardinality(), cyclic_permutation(), canonical JSON for dedup).
- [ ] Rewire gen/item/corpus/pipeline/teacher/firewall to the local model;
      drop the `hyprstream-decision` + `hyprstream-bench` path deps.
- [ ] `src/subject.rs` HTTP client + stub smoke test.
- [ ] CI: build/run the platform stub, point the client at it.
