# PRD-KS-001 — Part 2

## 5. Personas and user stories {#prd-5}

### 5.1 Personas

| ID | Persona | Primary needs |
|---|---|---|
| `P-001` | Personal operator | One native process, local Turso file, local media, low memory, simple recovery. |
| `P-002` | Household owner | Shared libraries and downloads with separate progress, bookmarks, filters, and invitations. |
| `P-003` | Tenant administrator | OIDC, membership lifecycle, RBAC, quotas, source policy, and audit events. |
| `P-004` | Reader | Search, library management, reading state, downloads, trackers, and multiple clients. |
| `P-005` | Client developer | Stable schemas, discoverable capabilities, predictable auth, pagination, and events. |
| `P-006` | Source maintainer | Declarative/Wasm SDKs, offline fixtures, diagnostics, compatibility and update policy. |
| `P-007` | Platform operator | PostgreSQL, object storage, replicas, migrations, metrics, backups, and recovery. |
| `P-008` | Security reviewer | Explicit trust boundaries, bounded resources, tenant tests, and auditable decisions. |

### 5.2 Core stories

- `US-001`: A personal operator can start with only a writable local Turso path and create the first owner.
- `US-002`: An owner can configure OIDC and invite users without using email as the durable identity key.
- `US-003`: A user may belong to multiple tenants and must explicitly select an active tenant.
- `US-004`: Shared manga and media do not imply shared personal reading progress or bookmarks.
- `US-005`: Administrators can enable sources, assign permissions, set quotas, and inspect audit events.
- `US-006`: A reader can search a source, add a title, refresh chapters, download content, and resume from another client.
- `US-007`: A source maintainer can run deterministic parsing tests without network access.
- `US-008`: A Wasm plugin can request approved host operations without raw host access.
- `US-009`: A larger operator can migrate from Turso to PostgreSQL with verification and rollback safety.
- `US-010`: Clients can discover server, API, plugin-contract, and feature versions.
- `US-011`: Tenant backups exclude deployment and user secrets unless a separate encrypted export is explicitly requested.
- `US-012`: Security-relevant membership, role, credential, and destructive actions are attributable.

### 5.3 Failure stories

- OIDC outage blocks new federated logins but does not arbitrarily destroy valid local sessions.
- Membership revocation takes effect on the next authorization check, not at session expiry.
- Malformed source responses, oversized decompression, and archive bombs fail within configured limits.
- Wasm fuel, time, memory, or host-call violations terminate only the offending invocation.
- Job leases and fencing prevent duplicate or stale-worker commits.
- Object-storage failure never produces a falsely complete download.
- Failed Turso-to-PostgreSQL verification leaves Turso authoritative and maintenance mode active.
- DNS rebinding and redirects to private networks are revalidated and denied.
- Idempotent client retries return the original operation result rather than duplicating effects.

---

## 6. System architecture {#prd-6}

### 6.1 Logical model

```text
Independent clients
  |-- GraphQL domain queries and mutations
  |-- REST auth, admin, media, backup, health and OPDS
  `-- SSE operation and progress events
          |
          v
+---------------------- KathaSutra ----------------------+
| protocol adapters -> authentication -> tenant context |
| -> permission checks -> application use cases          |
|                                                        |
| domain invariants and transaction orchestration        |
| durable jobs, scheduler, downloads and trackers        |
| source registry                                        |
|   |-- declarative source engine                        |
|   `-- Wasmtime Component Model runtime                 |
| persistence ports           media-storage ports         |
|   |-- Turso default         |-- filesystem default     |
|   `-- PostgreSQL scale-up   `-- S3-compatible          |
+--------------------------------------------------------+
          |
          `-- optional isolated browser-worker pool
```

### 6.2 Workspace boundaries

| Crate | Responsibility | Must not depend on |
|---|---|---|
| `kathasutra-domain` | IDs, entities, value objects, policies, invariants, domain errors | HTTP, SQL, filesystem, OIDC, Wasmtime |
| `kathasutra-application` | Use cases, ports, unit of work, authorization orchestration | Concrete DB clients and web frameworks |
| `kathasutra-persistence` | Repository contracts, migrations, Turso/PostgreSQL adapters | HTTP transport and presentation concerns |
| `kathasutra-auth` | Local credentials, sessions, OIDC, RBAC adapters | Manga/source implementation details |
| `kathasutra-source-runtime` | Package validation, declarative execution, WIT host, Wasmtime limits | Direct plugin access to domain persistence |
| `kathasutra-plugin-sdk` | Permissive WIT bindings and author helpers | Server-internal crates |
| `kathasutra-server` | Composition root, HTTP adapters, process lifecycle | Duplicated business rules |

`ARCH-001`: CI MUST enforce the inward dependency direction.

### 6.3 Deployment modes

**Default:** one `kathasutra-server` process, local Turso database file, filesystem media, and in-process worker.

**Scale-up:** multiple API replicas and worker replicas using PostgreSQL and S3-compatible storage, with an optional external browser-worker pool.

The binary SHOULD expose separable `server`, `worker`, `migrate`, `doctor`, and administrative roles.

### 6.4 Request and system context

Every authenticated tenant operation MUST use a non-forgeable context containing request ID, actor ID, active tenant ID, session ID, authentication method, and resolved permission set. Client headers may request a tenant but never establish authority.

Background jobs MUST carry tenant scope, initiator where applicable, payload version, attempt number, and fencing token. Unscoped cross-tenant jobs require explicit system-administration authorization and dedicated code paths.

### 6.5 Transactions and side effects

- Multi-aggregate mutations use an explicit unit of work.
- External effects use an outbox, durable job, or compensating-state pattern.
- Database commits MUST NOT assume tracker, object storage, browser, or source HTTP calls are atomic.
- Mutable aggregates use optimistic revisions.
- Cursor pagination uses a deterministic total order with a stable tie-breaker.

---

## 7. Functional requirements {#prd-7}

### 7.1 Bootstrap and configuration

| ID | Requirement |
|---|---|
| `FR-BOOT-001` | An empty local Turso database enters bootstrap mode and permits creation of exactly one initial owner using a single-use, expiring bootstrap credential or local command. |
| `FR-BOOT-002` | Bootstrap completion is transactional and cannot be re-enabled by deleting a config file. |
| `FR-CFG-001` | Configuration supports TOML, environment overrides, and secret-file references. |
| `FR-CFG-002` | Complete validation occurs before public sockets bind. Unknown keys fail by default. |
| `FR-CFG-003` | Administrators can inspect redacted effective configuration and version/schema information. |
| `FR-ADM-001` | Diagnostics expose server, PRD, API, database, migration, feature, and source-runtime versions. |
| `FR-ADM-002` | Destructive administration requires recent authentication or explicit step-up. |
| `FR-ADM-003` | Maintenance mode pauses ordinary mutations and schedulers while preserving recovery and migration operations. |

### 7.2 Identity and tenancy

| ID | Requirement |
|---|---|
| `FR-TEN-001` | Users are global installation identities and may have multiple tenant memberships. |
| `FR-TEN-002` | Membership has explicit invited, active, suspended, and revoked states. |
| `FR-TEN-003` | Every tenant-owned request and job resolves current membership and permission state. |
| `FR-TEN-004` | The last active tenant owner cannot be removed or demoted. |
| `FR-INV-001` | Invitations are tenant-scoped, expiring, revocable, single-use, and stored as hashes. |
| `FR-RBAC-001` | Authorization checks permissions, not role names. Built-in and custom roles map to permissions. |
| `FR-AUD-001` | Security-relevant actions generate append-only, redacted audit events. |
| `FR-QUOTA-001` | Tenant and optional user quotas apply to media, jobs, source requests, and other bounded resources. |

### 7.3 Authentication

| ID | Requirement |
|---|---|
| `FR-AUTH-001` | Local authentication supports Argon2id credentials and secure recovery without becoming a general IdP. |
| `FR-AUTH-002` | Server-side sessions use opaque, hashed tokens, rotation, idle and absolute expiry, and revocation. |
| `FR-AUTH-003` | Browser sessions use Secure, HttpOnly cookies and CSRF protection. |
| `FR-OIDC-001` | OIDC uses Authorization Code Flow with PKCE, state, nonce, discovery, signature, issuer, audience, and time validation. |
| `FR-OIDC-002` | Federated identity uniqueness is `(issuer, subject)`; email is profile metadata only. |
| `FR-OIDC-003` | Providers are administrator-configured; arbitrary tenant issuer URLs are out of initial scope. |
| `FR-OIDC-004` | Provisioning is invite-first by default and claim-to-role mapping is explicit policy. |
| `FR-OIDC-005` | Accounts are never automatically merged solely because email addresses match. |

### 7.4 Library and personal state

| ID | Requirement |
|---|---|
| `FR-LIB-001` | Tenants own libraries, shared collections, title records, chapter records, and shared media. |
| `FR-LIB-002` | Personal progress, read state, bookmarks, history, filters, and tracker accounts include both tenant and user scope. |
| `FR-LIB-003` | Titles may link to multiple source records without losing provenance. |
| `FR-LIB-004` | Duplicate merge and split operations are explicit, audited, and reversible where data remains available. |
| `FR-LIB-005` | Bulk operations are bounded, permission checked, idempotent where retryable, and produce operation status. |

### 7.5 Source platform

| ID | Requirement |
|---|---|
| `FR-SRC-001` | Source packages have stable namespaced identity, semantic version, API version, capabilities, provenance, content classification, preferences, and fixture metadata. |
| `FR-SRC-002` | Sources are installed globally but enabled and configured per tenant. |
| `FR-SRC-003` | Declarative packages support bounded HTTP, HTML/JSON/XML parsing, filters, pagination, transforms, cookies, and preference references. |
| `FR-SRC-004` | Executable source logic uses the WebAssembly Component Model and the versioned WIT contract. |
| `FR-SRC-005` | Plugins receive only explicit host capabilities; filesystem, process, environment, raw sockets, and direct database access are denied. |
| `FR-SRC-006` | Source HTTP is subject to origin policy, DNS/redirect revalidation, SSRF protection, response/decompression limits, timeouts, and rate limits. |
| `FR-SRC-007` | Normal CI source tests use committed sanitized fixtures and no live network. |
| `FR-SRC-008` | APK/JAR execution and universal bytecode translation are prohibited. |

### 7.6 Downloads, jobs, and media

| ID | Requirement |
|---|---|
| `FR-DL-001` | Downloads use a durable state machine and are complete only after verified media and manifest commit. |
| `FR-DL-002` | Global, tenant, and source concurrency is bounded; cancellation and retry are explicit. |
| `FR-DL-003` | Media streams without full-object buffering and uses checksums, atomic writes, and safe paths/keys. |
| `FR-JOB-001` | Durable jobs carry tenant scope, payload version, scheduling, priority, attempts, cancellation, and deduplication. |
| `FR-JOB-002` | Leases, heartbeats, reclaim, and fencing prevent duplicate or stale-worker commits. |
| `FR-JOB-003` | The scheduler supports library refresh, automatic downloads, cleanup, backup, tracker, and source-health work. |
| `FR-MEDIA-001` | Filesystem is default; S3-compatible storage is supported without changing domain behavior. |

### 7.7 APIs, backup, migration, OPDS, and trackers

| ID | Requirement |
|---|---|
| `API-001` | GraphQL is the primary domain query/mutation contract with bounded depth, complexity, result size, and cursor pagination. |
| `API-002` | REST handles authentication, administration, media, health, backup, migration, discovery, and source disclosure. |
| `API-003` | SSE provides resumable, authorized, backpressured operation and progress events. |
| `API-004` | Public errors use stable codes, localization keys, safe details, and request correlation without leaking internals. |
| `FR-BKP-001` | Tenant and deployment backups are versioned, checksummed, dry-runnable, and exclude secrets by default. |
| `FR-MIG-001` | Turso-to-PostgreSQL migration pauses jobs, checkpoints, copies in dependency order, verifies counts/checksums/API samples, and does not cut over on mismatch. |
| `FR-OPDS-001` | Authenticated OPDS exposes only authorized libraries and media without long-lived credentials in URLs. |
| `FR-TRK-001` | Trackers are personal integrations with encrypted tokens, idempotent sync, retries, and explicit conflicts. |

---

## 8. Data and persistence requirements {#prd-8}

### 8.1 Backend policy

| ID | Requirement |
|---|---|
| `DATA-DB-001` | `turso` is the default backend and supports a fully local in-process database file with no account, token, endpoint, or network dependency. |
| `DATA-DB-002` | Optional Turso synchronization remains a mode of the same backend. Public configuration MUST NOT expose `sqlite` or `libsql`. |
| `DATA-DB-003` | `postgres` is the supported scale-up backend and implements the same normative repositories and transaction semantics. |
| `DATA-DB-004` | Driver connection, row, query, transaction, and error types remain inside persistence adapters. |
| `DATA-DB-005` | Backend-specific SQL and optimizations are allowed behind shared conformance tests. |

### 8.2 Repository model

Repositories are domain-oriented and require explicit tenant scope. Generic SQL execution is not exposed to application services. A save operation includes expected revision and returns the committed revision or a typed conflict.

### 8.3 Required logical entities

The initial schema includes installation and migration state; users, local credentials, external identities, OIDC providers and login transactions; sessions; tenants, memberships, invitations, roles, permissions and support grants; libraries, categories, titles, source links, chapters, personal state, bookmarks and history; source packages, tenant source configuration, preferences, secret references, cookies and health; downloads, media objects and references; jobs, schedules, attempts and outbox events; tracker accounts/bindings; audit, idempotency, backup and migration operations.

### 8.4 Ownership and isolation

- Every tenant-owned row has direct or immutable-parent tenant scope.
- Security-critical tables SHOULD contain direct `tenant_id` columns to reduce join omission risk.
- Personal rows contain both tenant and user scope.
- PostgreSQL RLS MAY provide defence in depth, but application queries remain tenant-scoped for backend parity.
- Public IDs are opaque, URL-safe, non-sequential, and globally unique; UUIDv7-style identifiers are preferred.
- Persisted instants are UTC with explicit semantics; uncertain source dates retain precision information.

### 8.5 Concurrency and migrations

- Mutable aggregates use optimistic revisions.
- Conflicts return typed errors rather than last-write-wins.
- Idempotency keys are scoped by tenant, actor/client, operation family, and expiry.
- Turso and PostgreSQL have separate migration files sharing logical migration IDs.
- Startup refuses partial, unsupported, or future schemas.
- Migration execution is mutually exclusive and observable.
- Fresh installation and supported-version upgrades are tested on both backends.
- External media deletion is queued and retryable; database transactions do not assume object-store deletion succeeds.

### 8.6 Retention and deletion

Sessions, OIDC login transactions, idempotency records, temporary files, and incomplete downloads have bounded retention. Audit retention is configurable, while security-relevant tombstones may outlive content deletion. User deletion supports purge or anonymization policies while preserving necessary audit facts.
