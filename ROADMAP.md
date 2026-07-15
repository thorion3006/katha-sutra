# KathaSutra roadmap

The detailed delivery plan is tracked in issues KS-000 through KS-020 and governed by `docs/PRD-katha-sutra.md`.

## 0.1 — Foundation

Rust architecture, configuration/bootstrap, Turso persistence, domain model, local authentication, tenancy, RBAC, audit, and API foundations.

## 0.2 — Core reading server

Libraries, categories, manga, chapters, personal reading state, declarative sources, downloads, and durable jobs.

## 0.3 — Extensible sources

Stable WIT/plugin SDK, Wasmtime runtime, package lifecycle, source registry model, and initial source examples.

## 0.4 — Scale and portability

PostgreSQL parity, S3-compatible storage, backup/restore, OPDS, Turso-to-PostgreSQL migration, and trackers.

## 0.5 — Difficult sources and community tooling

Optional browser worker, Mihon source-porting analyzer, provenance workflow, and representative community source set.

## 1.0 — Stable community release

Security review, performance gates, API/WIT compatibility policy, signed multi-architecture releases, SBOMs, deployment documentation, Nix/NixOS support, and tested recovery procedures.

Dates are intentionally not promised. Each phase ships when its acceptance and security criteria are met.
