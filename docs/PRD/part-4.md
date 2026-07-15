# PRD-KS-001 — Part 4

## 13. Non-functional requirements {#prd-13}

### 13.1 Performance and memory {#prd-13-1}

| ID | Requirement |
|---|---|
| `NFR-PERF-001` | A release build using local Turso, no active browser worker, no Wasm invocation, and an empty/small library SHOULD remain at or below 80 MiB RSS on the reference Linux amd64 environment. |
| `NFR-PERF-002` | The project MUST continuously measure idle RSS, startup time, request latency, download throughput, scheduler overhead, and plugin invocation cost. |
| `NFR-PERF-003` | Image, archive, backup, export, and migration paths MUST stream or use bounded batches; no implementation may require buffering an entire large object by default. |
| `NFR-PERF-004` | Caches MUST be bounded by bytes and entries, expose hit/eviction metrics, and include tenant/security scope in keys. |
| `NFR-PERF-005` | Default request, source, download, job, and Wasm concurrency MUST be bounded and configurable within safe limits. |
| `NFR-PERF-006` | GraphQL list page sizes, query complexity, response size, and execution time MUST be bounded. |
| `NFR-PERF-007` | Resource limits MUST degrade through typed errors and backpressure rather than uncontrolled allocation or process failure. |

Reference performance budgets before 1.0:

- startup readiness with a migrated local Turso database: target <=2 seconds on the reference host;
- cached simple metadata request p95: target <=100 ms excluding client network;
- uncached internal database list request p95: target <=250 ms at reference scale;
- cancellation observation for queued/running jobs: target <=2 seconds;
- bounded incremental RSS during one 50 MiB media transfer: target <=16 MiB beyond baseline;
- bounded Wasm invocation memory as declared by source/runtime policy.

These are release gates only after the benchmark harness and reference environment are committed. Until then they are explicit design targets.

### 13.2 Reliability and recovery {#prd-13-2}

| ID | Requirement |
|---|---|
| `NFR-REL-001` | The process MUST survive malformed client input, source responses, plugin traps, optional worker failure, and tracker failure without corrupting durable state. |
| `NFR-REL-002` | Durable state transitions MUST be restart-safe and idempotent or explicitly compensatable. |
| `NFR-REL-003` | Readiness MUST become false during incompatible migrations, unavailable required storage/database dependencies, or unsafe maintenance states. |
| `NFR-REL-004` | Optional dependencies such as browser workers and trackers MUST not make the core unready unless configured as required. |
| `NFR-REL-005` | Graceful shutdown MUST stop new requests/jobs, drain within a configured deadline, and leave unfinished work reclaimable. |
| `NFR-REL-006` | Backup and restore procedures MUST be exercised automatically on supported backends. |
| `NFR-REL-007` | The supported upgrade path MUST cover a fresh install and every release within the documented support window. |

### 13.3 Scalability {#prd-13-3}

The local Turso mode targets personal, household, and modest shared deployments. PostgreSQL plus S3-compatible storage is the supported mode for multiple API replicas, distributed workers, high sustained write concurrency, and larger tenant populations.

| ID | Requirement |
|---|---|
| `NFR-SCALE-001` | Business behavior MUST not fork by backend. Backend-specific optimizations remain behind shared ports and conformance tests. |
| `NFR-SCALE-002` | Horizontally scaled deployments MUST use distributed-safe job claims, leases, fencing, and idempotency. |
| `NFR-SCALE-003` | In-memory state MUST be reconstructible or non-authoritative when multiple replicas are supported. |
| `NFR-SCALE-004` | SSE/event routing for multiple replicas MUST document affinity or shared-distribution requirements. |
| `NFR-SCALE-005` | Database-per-tenant MAY be introduced later through routing abstractions but is not required for 1.0. |

### 13.4 Accessibility and internationalization {#prd-13-4}

The server does not ship a full client UI, but protocol and minimal server pages MUST support accessibility and localization.

- Error codes are stable and accompanied by localization keys.
- User-facing server pages use semantic HTML, keyboard navigation, sufficient contrast, and meaningful labels.
- Locale-sensitive source parsing is explicit and testable; stored dates and numbers use normalized representations.
- User-provided Unicode is normalized where identity/security requires it while preserving display text appropriately.
- APIs do not require English parsing by clients.

### 13.5 Maintainability {#prd-13-5}

| ID | Requirement |
|---|---|
| `NFR-MAINT-001` | Public behavior changes include tests and documentation in the same change. |
| `NFR-MAINT-002` | Architecture, security, compatibility, and licensing changes require ADRs. |
| `NFR-MAINT-003` | Production code MUST NOT contain silent placeholders or TODO behavior that returns success without implementing the contract. |
| `NFR-MAINT-004` | Dependency additions require purpose, maintenance, license, security, and footprint review. |
| `NFR-MAINT-005` | Unsafe Rust is forbidden by default and any exception requires an ADR, safety invariants, and focused tests. |
| `NFR-MAINT-006` | The plugin WIT/SDK is versioned independently of the server release while maintaining a documented compatibility matrix. |

---

## 14. Security and privacy requirements {#prd-14}

### 14.1 Trust boundaries {#prd-14-1}

KathaSutra treats the following as untrusted or partially trusted:

- all client requests and identifiers;
- reverse-proxy forwarding metadata outside configured trusted proxies;
- OIDC discovery, JWKS, tokens, and claims until fully validated;
- third-party source packages and all upstream responses;
- browser workers and browser-rendered content;
- archive and media metadata;
- tracker responses;
- backup files supplied for restore;
- database content that may have been created by an older or compromised version.

### 14.2 Tenant isolation {#prd-14-2}

| ID | Requirement |
|---|---|
| `SEC-TEN-001` | Every tenant-owned repository operation MUST require tenant scope. |
| `SEC-TEN-002` | Every personal repository operation MUST require tenant and user scope. |
| `SEC-TEN-003` | Cross-tenant denials MUST not disclose resource existence through body, status variance where avoidable, timing-sensitive secondary lookups, or logs visible to the caller. |
| `SEC-TEN-004` | Cache, object-storage, cookie-jar, source-preference, tracker, browser-session, and event-stream keys MUST include security scope. |
| `SEC-TEN-005` | PostgreSQL RLS is defence in depth and MUST NOT replace application-layer predicates. |
| `SEC-TEN-006` | Automated tests MUST cover every role, membership state, transport, background job, storage backend, and database backend relevant to an operation. |

### 14.3 Secrets and cryptography {#prd-14-3}

- Secrets are supplied through protected files, environment/credential mechanisms, or an approved secret manager, not ordinary checked-in configuration.
- Application-managed refresh tokens, tracker tokens, TOTP secrets, and source secrets are encrypted at rest using a versioned envelope with key identifiers and rotation support.
- Passwords use Argon2id; token comparisons use constant-time primitives where applicable.
- Random credentials use an operating-system CSPRNG and sufficient entropy.
- Cryptographic algorithms and key sizes follow maintained library defaults and explicit policy; custom cryptography is prohibited.
- Logs, traces, metrics, audit metadata, panic reports, and support bundles use centralized redaction.

### 14.4 Network security {#prd-14-4}

| ID | Requirement |
|---|---|
| `SEC-NET-001` | TLS termination is required for non-loopback production access, either in-process or at a trusted reverse proxy. |
| `SEC-NET-002` | Trusted proxies and forwarded headers are allow-listed. |
| `SEC-NET-003` | Source, tracker, OIDC, webhook, and browser outbound requests enforce timeouts, size limits, redirect limits, DNS/IP policy, and safe TLS validation. |
| `SEC-NET-004` | SSRF defenses cover initial resolution, redirects, DNS rebinding, IPv4/IPv6 variants, and metadata endpoints. |
| `SEC-NET-005` | CORS is disabled or restrictive by default; wildcard credentialed origins are prohibited. |
| `SEC-NET-006` | Authentication, invitation, backup, migration, and expensive search endpoints have separate rate-limit policies. |

### 14.5 Plugin and content security {#prd-14-5}

- Source plugins are untrusted code even when signed.
- Package signatures establish provenance, not safety; capabilities and runtime limits still apply.
- Wasm components have no ambient authority.
- Parser handles and host resources are bounded and invalid after invocation completion.
- HTML is treated as data; KathaSutra does not render arbitrary upstream scripts in core pages.
- Archive extraction rejects traversal, absolute paths, unsafe links, excessive entries, excessive expanded size, and excessive compression ratios.
- Image metadata and decoders are treated as hostile inputs and run through maintained libraries with resource limits.
- Browser workers are isolated processes/containers with ephemeral profiles and no core filesystem/database credentials.

### 14.6 Privacy {#prd-14-6}

| ID | Requirement |
|---|---|
| `SEC-PRIV-001` | Collect only operationally necessary identity and device metadata. |
| `SEC-PRIV-002` | Reading history, bookmarks, tracker accounts, and private collections are personal data and are not shared merely because media is shared. |
| `SEC-PRIV-003` | Administrators receive only permissions explicitly granted by product policy; support access is visible and audited. |
| `SEC-PRIV-004` | Data export and deletion are supported for users and tenants subject to documented audit/legal retention. |
| `SEC-PRIV-005` | Telemetry is opt-in unless strictly local, contains no source URLs/query terms/title names/user IDs by default, and documents every field. |
| `SEC-PRIV-006` | Backup secrets are excluded by default and any secret export requires explicit encryption and recent authorization. |

### 14.7 Security response {#prd-14-7}

The repository maintains private vulnerability reporting, a supported-version policy, severity triage, coordinated disclosure, and release advisories. Security fixes include regression tests when safe to publish. Public issues must not contain exploit details or real secrets.

---

## 15. Operations and deployment {#prd-15}

### 15.1 Configuration model {#prd-15-1}

Configuration is strict, versioned TOML with environment overrides and secret-file references. Unknown fields fail startup by default. Effective configuration output is redacted. Startup validates path permissions, database backend, public URL/origin policy, OIDC providers, storage, source directories, worker roles, and unsafe option combinations before binding public sockets.

Only two public database values are supported:

```toml
[database]
backend = "turso" # default
```

or

```toml
[database]
backend = "postgres"
```

Turso local-only is the default and requires only a writable path. Optional Turso synchronization may add URL/token configuration but remains `backend = "turso"`.

### 15.2 Filesystem and process safety {#prd-15-2}

- The service runs as an unprivileged user.
- Configuration, data, media, temporary, plugin, and cache paths are explicit.
- Secrets and database files receive restrictive permissions.
- Temporary files are created safely in controlled directories.
- The OCI image runs non-root, supports a read-only root filesystem, and declares writable volumes.
- Process execution is not available to source plugins.
- PID 1 and signal handling are correct in containers.

### 15.3 Health and diagnostics {#prd-15-3}

- Liveness indicates the process event loop can respond and MUST not fail for optional dependencies.
- Startup indicates configuration and initialization progress.
- Readiness indicates required migrations, database, storage, and mandatory worker dependencies are usable.
- Detailed diagnostics require administrator authorization and redact secrets.
- A `doctor` command validates configuration, permissions, database schema, storage access, OIDC discovery, source packages, browser connectivity, and clock sanity without performing unsafe mutations.

### 15.4 Observability {#prd-15-4}

Structured logs include timestamp, level, service version, request/job/operation ID, safe tenant/source identifiers where appropriate, event name, and redacted fields. Metrics use bounded labels; raw user IDs, title names, URLs, query values, and tokens are prohibited as labels.

Required metric families include:

- request count, duration, status/error code, in-flight count;
- database operation duration and pool/queue state;
- job queue depth, age, claim, retry, failure, dead-letter, and lease loss;
- download bytes, duration, retries, cancellation, and storage errors;
- source invocation duration, errors, rate limits, parser failures, and resource-limit terminations;
- Wasm compile/cache/invocation/fuel/memory outcomes;
- OIDC/local login outcomes and session counts without account enumeration;
- backup, restore, migration, and cleanup results;
- process RSS, CPU, file descriptors, and runtime queues.

OpenTelemetry tracing is optional and off by default. Trace export follows the privacy policy.

### 15.5 Backup, recovery, and maintenance {#prd-15-5}

Operators receive documented procedures for:

- consistent local Turso backup/checkpoint and restore;
- PostgreSQL logical/physical backup integration;
- filesystem and S3 media backup;
- tenant/deployment application backup;
- disaster recovery from database-only, media-only, and combined failures;
- source package rollback;
- credential/key rotation;
- Turso-to-PostgreSQL migration;
- upgrade, downgrade limitations, and maintenance mode.

Recovery procedures MUST identify RPO/RTO assumptions and validation commands. A backup is not considered supported until restore is tested.

### 15.6 Packaging and releases {#prd-15-6}

Release targets SHOULD include Linux amd64 and arm64 native binaries and OCI images; additional platforms may follow. Each release includes:

- versioned source archive;
- binary/image digests;
- SBOM;
- license and notice bundle;
- signatures or attestations;
- changelog and security notes;
- PRD, API, migration, backup, and WIT compatibility versions;
- upgrade instructions and known limitations.

A Nix package/flake and NixOS module are required or explicitly tracked before 1.0 because NixOS is a primary deployment environment for the product owner.

---

## 16. Verification, delivery, risks, and metrics {#prd-16}

### 16.1 Test strategy {#prd-16-1}

| Layer | Required testing |
|---|---|
| Domain | Unit and property tests for invariants, identifiers, permissions, revisions, cursors, and state machines. |
| Persistence | Shared conformance suite against Turso and PostgreSQL; migration, rollback, concurrency, pagination, isolation, and crash tests. |
| Authentication | Local credential/session tests and mock OIDC provider matrix including key rotation, replay, malformed claims, outages, and clock skew. |
| API | Schema snapshots, contract tests, authorization matrix, complexity/size limits, idempotency, cancellation, and safe errors. |
| Declarative sources | Schema tests, sanitized offline fixtures, golden outputs, malformed content, timeout and size limits. |
| Wasm | ABI compatibility, prohibited imports, fuel/time/memory exhaustion, trap recovery, cancellation, and tenant isolation. |
| Media/downloads | Traversal, corruption, resume, cancellation, streaming memory, archive bombs, S3/filesystem parity, and cleanup. |
| Jobs | Claim races, lease expiry, fencing, retries, dead-letter, fairness, restart and duplicate schedule tests. |
| Backup/migration | Round-trip restore, corrupt/incompatible input, Turso-to-PostgreSQL verification and rollback behavior. |
| Security | Tenant matrix, SSRF/DNS rebinding, CSRF, CORS, session fixation, path/archive fuzzing, secret redaction and dependency review. |
| Performance | Reproducible RSS, latency, throughput, queue, and plugin benchmarks with retained baselines. |

Live third-party websites MUST NOT be part of normal CI. Opt-in scheduled compatibility checks, where legally and operationally appropriate, use dedicated credentials, strict rate limits, no private data, and failure semantics that do not block unrelated contributions without review.

### 16.2 End-to-end acceptance scenarios {#prd-16-2}

`AC-E2E-001 — Local first run`

- **Given** a clean host, writable data/media paths, and no cloud account,
- **When** the operator starts KathaSutra with local Turso and completes bootstrap,
- **Then** the server becomes ready, creates one owner and tenant, and exposes discovery/health without requiring PostgreSQL, S3, OIDC, browser, or Java.

`AC-E2E-002 — Multi-user isolation`

- **Given** two users in one tenant and another tenant,
- **When** they share selected library content and update reading progress,
- **Then** shared records are visible according to permissions, personal progress remains separate, and cross-tenant identifiers do not reveal data.

`AC-E2E-003 — OIDC login`

- **Given** an administrator-configured provider and invite-first policy,
- **When** an invited user completes Authorization Code + PKCE,
- **Then** identity resolves by `(issuer, subject)`, the invitation is consumed once, mapped permissions are explicit, and a rotated local session is issued.

`AC-E2E-004 — Source and download`

- **Given** a validated declarative or Wasm source fixture,
- **When** a user searches, adds a title, refreshes chapters, and downloads one chapter,
- **Then** source calls obey capabilities/limits, records preserve source provenance, media streams to storage, checksums verify, and progress events resume after reconnect.

`AC-E2E-005 — Worker crash recovery`

- **Given** a running download/update job,
- **When** the worker terminates after acquiring a lease,
- **Then** another worker reclaims the expired job, the stale fencing token cannot commit, and no duplicate completed effect is produced.

`AC-E2E-006 — Backup and restore`

- **Given** a tenant with shared and personal data,
- **When** an authorized owner creates and restores a validated backup into an empty compatible installation,
- **Then** data and media references round-trip, excluded secrets remain excluded, and users retain separate personal state.

`AC-E2E-007 — Turso to PostgreSQL`

- **Given** a healthy Turso deployment and empty compatible PostgreSQL target,
- **When** the migration workflow runs,
- **Then** jobs drain, checkpoint/export/import complete, counts/checksums/API samples and isolation checks pass, and cutover occurs only after successful verification.

### 16.3 Implementation sequence {#prd-16-3}

The implementation backlog is normative in intent and represented by GitHub issues KS-001 through KS-020:

1. workspace and CI;
2. domain and tenant primitives;
3. configuration/bootstrap/lifecycle;
4. Turso persistence;
5. PostgreSQL parity;
6. local authentication;
7. OIDC;
8. tenancy/RBAC/audit;
9. API foundation;
10. library and reading state;
11. source package/WIT contract;
12. declarative runtime;
13. Wasmtime runtime;
14. storage/downloads;
15. scheduler/jobs;
16. backup/migration/OPDS;
17. trackers;
18. browser worker;
19. Mihon source-porting assistance and initial sources;
20. packaging, hardening, observability, performance, and 1.0 gates.

### 16.4 Definition of done {#prd-16-4}

A work package is complete only when:

- all referenced requirements and acceptance criteria are implemented;
- tests include failure, cancellation, authorization, and tenant-isolation cases;
- Turso/PostgreSQL parity is tested when persistence is affected;
- schemas, migrations, API/WIT contracts, examples, and docs are updated together;
- security and privacy impact is documented;
- new metrics/logs are bounded and redacted;
- no production placeholder remains;
- `cargo fmt`, Clippy with warnings denied, workspace tests, dependency/license checks, and applicable integration tests pass;
- the pull request is DCO-signed and reviewable as one coherent change.

### 16.5 Product metrics {#prd-16-5}

Project success metrics include:

- idle and workload RSS by release;
- startup/readiness time;
- successful fresh installations and upgrades;
- Turso/PostgreSQL conformance pass rate;
- tenant-isolation regression count;
- OIDC provider conformance coverage;
- download/job failure and recovery rates;
- backup restore success rate;
- source fixture pass rate and mean repair time;
- API/WIT compatibility break count;
- median contribution review time;
- signed artifact and SBOM coverage;
- open critical security findings.

Metrics MUST be collected in tests or operator-local telemetry unless explicit opt-in community telemetry is approved.

### 16.6 Key risks and mitigations {#prd-16-6}

| Risk | Mitigation |
|---|---|
| Turso/PostgreSQL semantic divergence | Domain ports, separate migrations, shared conformance suite, explicit capability differences. |
| Cross-tenant leakage | Mandatory scope, typed context, negative matrix, cache/storage scope, RLS defence in depth. |
| OIDC complexity | Standards-compliant libraries, mock-provider matrix, strict issuer/audience/nonce/PKCE policy. |
| Plugin sandbox escape or SSRF | Component Model, no ambient WASI, host capabilities, Wasmtime limits, outbound network policy. |
| Website churn | Declarative-first sources, offline fixtures, diagnostics, health reporting, independent source updates. |
| Browser memory cost | Optional external worker with strict quotas and no effect on core idle footprint. |
| Large media memory usage | Streaming, bounded batches, byte-limited caches, archive/decompression limits. |
| Community source legal risk | Provenance/license checklist, prohibited-source policy, takedown process, no circumvention features. |
| AGPL discourages plugin authors | Permissive MIT OR Apache-2.0 SDK/WIT boundary. |
| Premature API lock-in | Pre-1.0 versioning, explicit compatibility policy, contract tests and deprecation windows. |

### 16.7 Dependencies and integrations {#prd-16-7}

Expected implementation classes include an async Rust runtime, HTTP server/client, GraphQL server, serialization/validation, Turso Database Rust SDK, PostgreSQL client/pool, Argon2id, OIDC/JWT, Wasmtime Component Model, HTML/JSON/XML parsing, archive/image handling, S3-compatible storage, OpenTelemetry/Prometheus support, and cryptographic utilities.

Exact crates are chosen through implementation review and recorded in Cargo metadata/ADRs where architectural. Dependencies MUST be maintained, license-compatible, and measured for binary/runtime footprint.

### 16.8 Open decisions {#prd-16-8}

The following may be resolved during implementation through ADRs without changing core product intent:

- exact Rust HTTP and GraphQL frameworks;
- exact Turso and PostgreSQL adapter libraries and pooling strategy;
- server-side session encryption versus token-digest-only storage for each field;
- inclusion and timing of TOTP;
- exact S3 client and multipart threshold;
- exact private browser-worker protocol;
- OPDS 1.2 versus 2.0 initial priority;
- initial tracker adapters;
- signed package registry format and trust roots;
- support window length after 1.0.

Decisions that alter the public backend choices, tenant model, OIDC identity key, server-only boundary, APK/JAR prohibition, plugin capability model, or licensing boundary require a PRD revision.

### 16.9 Version history {#prd-16-9}

| Version | Date | Change |
|---|---|---|
| `0.1.0-draft` | 2026-07-15 | Initial implementation-grade PRD: Rust server, Turso default, PostgreSQL scale-up, OIDC, multi-tenancy, declarative/Wasm sources, server/client separation. |

### 16.10 Release traceability {#prd-16-10}

Every release MUST record:

- source commit;
- PRD version;
- API schema/version;
- database schema/migration version;
- backup format version;
- source package and WIT versions;
- minimum/maximum compatible versions where applicable;
- build toolchain;
- artifact digests and provenance.

### 16.11 Primary references {#prd-16-11}

- Suwayomi Server: `https://github.com/Suwayomi/Suwayomi-Server`
- Turso Rust quickstart: `https://docs.turso.tech/sdk/rust/quickstart`
- Turso Database: `https://github.com/tursodatabase/turso`
- OpenID Connect Core 1.0: `https://openid.net/specs/openid-connect-core-1_0.html`
- OIDC Discovery: `https://openid.net/specs/openid-connect-discovery-1_0.html`
- PKCE: `https://www.rfc-editor.org/rfc/rfc7636`
- OAuth 2.0 Security Best Current Practice: `https://www.rfc-editor.org/rfc/rfc9700`
- Wasmtime: `https://wasmtime.dev/`
- WebAssembly Component Model: `https://component-model.bytecodealliance.org/`
- WIT reference: `https://component-model.bytecodealliance.org/design/wit.html`
- OPDS: `https://opds.io/`
- GNU AGPL v3: `https://www.gnu.org/licenses/agpl-3.0.html`
- Developer Certificate of Origin: `https://developercertificate.org/`

---

## 17. Approval {#prd-17}

This PRD is approved for implementation bootstrap. GitHub issue KS-000 is the parent delivery epic. Conflicts between an implementation issue and this PRD are resolved in favor of the latest committed PRD version unless an approved ADR and PRD amendment explicitly supersede it.
