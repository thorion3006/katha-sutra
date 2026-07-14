# KathaSutra

[![License: AGPL-3.0-or-later](https://img.shields.io/badge/license-AGPL--3.0--or--later-blue.svg)](LICENSE)
[![Plugin SDK: MIT OR Apache-2.0](https://img.shields.io/badge/plugin%20SDK-MIT%20OR%20Apache--2.0-blue.svg)](NOTICE.md)
[![Status: design and bootstrap](https://img.shields.io/badge/status-design%20%26%20bootstrap-orange.svg)](ROADMAP.md)

**KathaSutra** is a lightweight, client-independent server for manga, comics, and other paged serial media. It is designed from the ground up for low memory usage, multi-user and multi-tenant deployments, OIDC authentication, and safe community source plugins.

KathaSutra is **not** a port of Suwayomi and does **not** execute Mihon APK or JAR extensions. Existing extension source logic may be reimplemented using KathaSutra's declarative source format or sandboxed WebAssembly Component Model SDK.

## Project status

The project is in architecture and implementation bootstrap. The normative product specification is [`docs/PRD-katha-sutra.md`](docs/PRD-katha-sutra.md). Implementation work is split into Codex-ready work packages under [`docs/issues/`](docs/issues/).

The current Rust workspace is intentionally minimal. It establishes repository boundaries without pretending unfinished features exist.

## Core design

- **Rust server only:** no bundled web client or Electron launcher.
- **Turso Database by default:** local in-process storage is the zero-service default; optional Turso sync is supported without exposing libSQL as a product backend.
- **PostgreSQL scale-up path:** supported for large deployments, multiple API replicas, and distributed workers.
- **Multi-user and multi-tenant:** global identities, tenant memberships, tenant-scoped resources, personal reading state, RBAC, quotas, and audit events.
- **OIDC:** Authorization Code Flow with PKCE, secure server-side sessions, multiple administrator-configured providers, and invite-first provisioning.
- **Safe source platform:** declarative sources for common sites and Wasmtime-hosted components for complex logic.
- **Separated client contract:** GraphQL for domain data, REST for authentication/binary/admin operations, SSE for event streams, and OPDS for compatible readers.
- **Media storage abstraction:** local filesystem by default, S3-compatible object storage for larger deployments.

## Repository layout

```text
crates/
  kathasutra-server/          process entrypoint and composition root
  kathasutra-domain/          domain types and invariants
  kathasutra-application/     use cases and ports
  kathasutra-persistence/     backend-neutral persistence contracts
  kathasutra-auth/            identity, sessions, OIDC, and authorization
  kathasutra-source-runtime/  declarative and WebAssembly source execution
  kathasutra-plugin-sdk/      permissively licensed plugin SDK
docs/
  PRD-katha-sutra.md
  adr/
  issues/
wit/
  kathasutra-source.wit
```

## Development bootstrap

Prerequisites:

- Rust stable with the 2024 edition
- `just` for convenience commands
- Git

```bash
cargo test --workspace
cargo fmt --all -- --check
cargo clippy --workspace --all-targets --all-features -- -D warnings
```

The default configuration contract is documented in [`config/kathasutra.example.toml`](config/kathasutra.example.toml). It is not yet wired into the bootstrap binary.

## Contributing

Read [`CONTRIBUTING.md`](CONTRIBUTING.md), [`GOVERNANCE.md`](GOVERNANCE.md), and the [`CODE_OF_CONDUCT.md`](CODE_OF_CONDUCT.md). Contributions use Developer Certificate of Origin sign-off rather than a CLA.

Security vulnerabilities must not be reported in public issues. Follow [`SECURITY.md`](SECURITY.md).

## Licensing

The server and most repository content are licensed under **AGPL-3.0-or-later**. The plugin SDK and WIT interface under `crates/kathasutra-plugin-sdk/` and `wit/` are licensed under **MIT OR Apache-2.0**. See [`NOTICE.md`](NOTICE.md) for the exact boundary.

KathaSutra does not grant rights to third-party content, websites, trademarks, or extension implementations. Operators and source-plugin authors are responsible for complying with applicable law and upstream terms.
