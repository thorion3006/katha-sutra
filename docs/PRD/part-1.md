# Product Requirements Document: KathaSutra

## 1. Document control {#prd-1}

| Field | Value |
|---|---|
| Document ID | `PRD-KS-001` |
| Product | KathaSutra |
| Repository | `thorion3006/katha-sutra` |
| Version | `0.1.0-draft` |
| Status | Approved for implementation bootstrap |
| Date | 2026-07-15 |
| Product owner | Sajeev / `@thorion3006` |
| Primary implementation language | Rust, edition 2024 |
| Default database | Turso Database, local-only by default |
| Scale-up database | PostgreSQL |
| Core license | AGPL-3.0-or-later |
| Plugin SDK/WIT license | MIT OR Apache-2.0 |
| Supersedes | Initial Suwayomi native-rewrite discussion |
| Version pointer | The PRD version implemented by a release MUST be recorded in release metadata and the generated server build information. |

### 1.1 Normative language {#prd-1-1}

The terms **MUST**, **MUST NOT**, **SHOULD**, **SHOULD NOT**, and **MAY** are normative. Requirement identifiers are stable cross-references. A requirement may only be removed through a PRD version update that records the rationale and migration impact.

### 1.2 Source references {#prd-1-2}

The design is informed by the following primary references:

- Suwayomi Server at release commit `1d583ca4a718646e17a4c62d23fe8e5edccaf774`, particularly its extension loading, Android compatibility, source models, library, downloads, GraphQL, OPDS, and backup implementation.
- Turso Rust quickstart and `turso` crate documentation: the official crate supports a local in-process database and optional explicit synchronization using the Turso Database engine.
- OpenID Connect Core 1.0, OAuth 2.0 Authorization Code Flow, PKCE, OAuth 2.0 Security Best Current Practice, and OIDC Discovery.
- Wasmtime and the WebAssembly Component Model/WIT specifications.
- OPDS 1.2 and OPDS 2.0 specifications.

Reference links are collected in Part 4, Section 16.11.

### 1.3 Requirement taxonomy {#prd-1-3}

| Prefix | Meaning |
|---|---|
| `FR-` | Functional requirement |
| `NFR-` | Non-functional requirement |
| `SEC-` | Security or privacy requirement |
| `DATA-` | Data and persistence requirement |
| `API-` | Public API requirement |
| `OPS-` | Deployment and operations requirement |
| `TEST-` | Verification requirement |
| `AC-` | Acceptance scenario |
| `CON-` | Explicit constraint |

---

## 2. Executive summary {#prd-2}

KathaSutra is a low-memory, client-independent server for manga, comics, webtoons, and similar paged serial media. It provides source discovery, metadata, chapter lists, page resolution, libraries, categories, reading progress, downloads, scheduled updates, backups, trackers, and OPDS without bundling a client application.

The product is a clean Rust implementation. It does not preserve Mihon API compatibility and does not execute Mihon APK or JAR extensions. Instead, KathaSutra exposes a stable source-plugin platform with two tiers:

1. a declarative schema for conventional HTTP/HTML/JSON/XML sources; and
2. sandboxed WebAssembly Components for source-specific executable logic.

KathaSutra is multi-user and multi-tenant from the first persistence migration. A global user may be a member of multiple tenants. Shared tenant data and personal reading state are separated. Authentication includes built-in local accounts and standards-compliant OIDC. Authorization uses internal permissions and tenant-scoped RBAC; identity-provider claims are inputs to explicit mapping policy, never direct application permissions.

Turso Database is the default persistence engine. A zero-service deployment runs Turso locally in process using a database file. Optional Turso synchronization remains part of the same `turso` backend. PostgreSQL is the supported scale-up backend for large deployments, sustained concurrent writes, multiple API replicas, and distributed workers. SQLite and libSQL are not public product backends.

The server exposes GraphQL for domain queries and mutations, REST for authentication, administration, media, backup, health, and compatibility endpoints, Server-Sent Events for progress streams, and OPDS for supported readers. Media is stored on the local filesystem by default or in S3-compatible object storage.

### 2.1 Product promise {#prd-2-1}

> Run a secure, efficient, multi-user serial-media server as one native process with a local Turso database, while retaining a supported path to OIDC federation, object storage, PostgreSQL, distributed workers, and community-maintained source plugins.

### 2.2 Success definition {#prd-2-2}

KathaSutra reaches 1.0 when:

- the public API, database migration policy, and source-plugin contract are stable;
- Turso local and PostgreSQL pass the same normative domain conformance suite;
- tenant isolation and OIDC flows have received focused security review;
- an installation can search a source, add a title, track personal reading state, download chapters, run scheduled updates, back up and restore tenant data, and use a separate client without Mihon interoperability;
- a source author can implement and deterministically test a declarative or WebAssembly source without importing server internals;
- multi-architecture binaries and OCI images are signed and reproducible enough for community verification.

---

## 3. Background and problem statement {#prd-3}

### 3.1 Existing architecture problem {#prd-3-1}

Suwayomi Server derives much of its value from running extensions written for Mihon/Tachiyomi. That compatibility requires a JVM process, Kotlin/Java dependencies, Android compatibility types, APK metadata parsing, DEX-to-JAR conversion, child-first class loading, reflection, and compatibility implementations of source and networking interfaces. Some sources additionally require a Chromium-based WebView or external anti-bot service.

For a new server that does not require Mihon interoperability, preserving this compatibility layer would retain the dominant complexity and resource cost while constraining API and plugin design to Android-originated abstractions. Rewriting the server in a native language while embedding a JVM for extensions would therefore deliver only partial benefit.

### 3.2 User problems {#prd-3-2}

- A personal operator wants a small server that does not require Java, Electron, or a separate database service.
- A household wants separate reading progress and content preferences while sharing selected libraries and downloads.
- An organization wants OIDC, invitations, role assignment, auditability, quotas, and tenant isolation.
- A larger operator wants PostgreSQL and multiple process replicas without changing the application contract.
- A source maintainer wants a portable, testable source SDK rather than Android APIs and dynamically loaded JVM bytecode.
- A client author wants a stable, documented server contract that is not coupled to a bundled web interface.
- A security-conscious operator wants third-party source code isolated from the host filesystem, process table, database, and secrets.

### 3.3 Opportunity {#prd-3-3}

A greenfield architecture can remove compatibility obligations and optimize for server workloads:

- streaming rather than buffering images and archives;
- bounded concurrency and byte-budgeted caches;
- explicit tenant context;
- domain repositories rather than ORM leakage;
- safe component plugins with capability-based host calls;
- a local-first Turso database with a PostgreSQL scale-up path;
- modern identity and authorization;
- client/API separation;
- deterministic plugin fixtures instead of live-site CI.

### 3.4 Constraints {#prd-3-4}

| ID | Constraint |
|---|---|
| `CON-001` | The implementation language is Rust. Go and Odin are not target implementations. |
| `CON-002` | The repository concentrates on the server. No bundled web, desktop, Android, iOS, or Electron client is included. |
| `CON-003` | Mihon API, backup, tracker, extension, APK, JAR, and client compatibility are not normative goals. One-way import MAY be implemented where useful. |
| `CON-004` | The product exposes only `turso` and `postgres` database backends. It MUST NOT present SQLite or libSQL as selectable public backends. |
| `CON-005` | Turso MUST work fully local-only with no cloud account or network dependency. |
| `CON-006` | A user may belong to multiple tenants. Tenant membership MUST NOT be encoded as a single field on the user. |
| `CON-007` | Source plugins MUST NOT receive unrestricted filesystem, process, raw-socket, environment, or database access. |
| `CON-008` | CI source tests MUST NOT depend on live third-party websites. |
| `CON-009` | The server MUST remain usable behind a reverse proxy and MUST NOT require direct public exposure. |
| `CON-010` | The core server is AGPL-3.0-or-later; the plugin WIT and SDK are MIT OR Apache-2.0. |

---

## 4. Product scope {#prd-4}

### 4.1 Goals {#prd-4-1}

| ID | Goal |
|---|---|
| `G-001` | Provide a materially smaller steady-state memory footprint than a JVM/Android compatibility server for equivalent native features. |
| `G-002` | Make a single-process local Turso deployment the default and easiest installation. |
| `G-003` | Support a well-defined PostgreSQL scale-up mode without business-logic forks. |
| `G-004` | Enforce tenant and personal-data boundaries across HTTP, jobs, storage, cache, and plugins. |
| `G-005` | Support local authentication and generic OIDC relying-party operation. |
| `G-006` | Offer a source platform that is language-neutral at the ABI and safer than in-process JVM extensions. |
| `G-007` | Provide a stable contract for independent clients. |
| `G-008` | Make backup, restore, export, and Turso-to-PostgreSQL migration supported workflows. |
| `G-009` | Make community contribution, security reporting, release provenance, and governance first-class. |

### 4.2 Non-goals {#prd-4-2}

| ID | Non-goal |
|---|---|
| `NG-001` | Running existing Mihon APK or JAR extensions. |
| `NG-002` | Automatic universal translation of arbitrary JVM bytecode to Rust or WebAssembly. |
| `NG-003` | Shipping or maintaining a first-party graphical client in this repository. |
| `NG-004` | Acting as a general identity provider. |
| `NG-005` | Providing DRM circumvention, credential theft, CAPTCHA farms, or unauthorized access mechanisms. |
| `NG-006` | Distributed consensus or multi-primary writes on Turso local files. |
| `NG-007` | Database-per-tenant provisioning in the initial stable release. The architecture may permit it later. |
| `NG-008` | Storing chapter image blobs inside the relational database. |
| `NG-009` | Guaranteeing that every third-party website can be supported. |
| `NG-010` | Preserving every historical Suwayomi endpoint or data shape. |

### 4.3 Release scope {#prd-4-3}

#### 4.3.1 MVP / 0.1–0.3

- configuration and secure secret references;
- Turso local persistence;
- global users, tenants, memberships, roles, permissions, invitations;
- local authentication and OIDC;
- GraphQL/REST/SSE foundations;
- library, categories, manga, chapters, reading progress;
- declarative sources and fixture runner;
- WebAssembly source runtime;
- local media downloads and scheduler.

#### 4.3.2 0.4–0.5

- S3-compatible media storage;
- PostgreSQL adapter and parity suite;
- backup/restore/export and backend migration;
- OPDS;
- trackers;
- optional browser worker;
- source porting tools and initial community source set.

#### 4.3.3 1.0

- stable API and source ABI;
- signed releases, upgrade guarantees, operational documentation;
- security hardening and performance budgets;
- recovery and migration validation.

### 4.4 UX and design boundary {#prd-4-4}

KathaSutra has no bundled application UI. It MUST nevertheless provide client-facing design primitives:

- machine-readable discovery and capability endpoints;
- consistent error codes and localization keys;
- pagination, sorting, filtering, and cursor semantics;
- asynchronous operation handles and event streams;
- OIDC redirects suitable for browser, native, and device clients;
- accessible, minimal server-rendered pages only where protocol flows require them, such as login errors, consent-independent OIDC callbacks, invitation acceptance, and AGPL source disclosure.

Any server-rendered page MUST be functional without client-side JavaScript unless a protocol requirement makes that impractical.
