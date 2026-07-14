# Contributing to KathaSutra

Thank you for contributing. KathaSutra is a security-sensitive, multi-tenant server; correctness and isolation take priority over speed of merging.

## Before starting

1. Read `docs/PRD-katha-sutra.md`, the relevant `KS-*` issue, and applicable ADRs.
2. Work on one issue per pull request unless the issue explicitly requires integration work.
3. Discuss changes to public APIs, database schemas, WIT contracts, authentication, authorization, or licensing before implementation.
4. Do not use public issues for vulnerabilities; follow `SECURITY.md`.

## Development

Use stable Rust and keep the workspace buildable:

```bash
cargo fmt --all -- --check
cargo clippy --workspace --all-targets --all-features -- -D warnings
cargo test --workspace
```

`just check` is the canonical local validation command once KS-001 is complete.

## Pull-request requirements

Every pull request must:

- link its issue and cite implemented PRD requirement IDs;
- explain security, tenant, database, migration, public-contract, and observability impact;
- include happy-path, failure, cancellation, authorization, and tenant-isolation tests where applicable;
- test Turso and PostgreSQL parity when persistence semantics change;
- update migrations, schemas, examples, ADRs, and documentation together with behavior;
- avoid hidden production TODOs and placeholder implementations;
- keep secrets, tokens, cookies, SQL, private upstream bodies, and stack traces out of logs and errors.

## Architecture rules

- Domain and application crates must not depend on transport, Turso, PostgreSQL, Wasmtime, or browser implementations.
- Turso is the default product backend; PostgreSQL is the scale-up backend. Do not add SQLite or libSQL as user-selectable backends.
- Do not add Mihon APK/JAR execution or bundled client code.
- Source plugins use declarative packages or the versioned WIT Component Model contract.
- All tenant-owned persistence and storage paths require explicit tenant scope.

## Commit sign-off

KathaSutra uses the Developer Certificate of Origin rather than a CLA. Sign every commit:

```bash
git commit -s
```

By signing off, you certify the DCO in `DCO`.

## Licensing

Contributions are accepted under the license of the destination path. The server is AGPL-3.0-or-later. `crates/kathasutra-plugin-sdk/**` and `wit/**` are MIT OR Apache-2.0 as described in `NOTICE.md`.
