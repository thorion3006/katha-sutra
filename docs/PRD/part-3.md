# PRD-KS-001 — Part 3

## 9. Identity, authentication, tenancy, and authorization {#prd-9}

### 9.1 Identity model {#prd-9-1}

KathaSutra separates installation identity from tenant membership.

```text
User --< ExternalIdentity
User --< LocalCredential
User --< Session
User --< TenantMembership >-- Tenant
TenantMembership --< MembershipRole >-- Role --< RolePermission >-- Permission
```

A `User` is global within one KathaSutra installation. A `TenantMembership` grants that user access to one tenant and carries lifecycle state. Tenant ID MUST NOT be stored as a single-valued field on the user record.

`SEC-ID-001`: Federated identities MUST be keyed by the exact normalized tuple `(issuer, subject)`.

`SEC-ID-002`: Email address, display name, username, group name, and `preferred_username` MUST NOT be used as the durable federated identity key.

`SEC-ID-003`: Profile claims MAY be refreshed after authentication but MUST NOT silently transfer ownership or memberships.

`SEC-ID-004`: Account linking MUST require an authenticated existing session and a newly completed authentication ceremony for the identity being linked.

`SEC-ID-005`: Automatic account merging based only on matching email is prohibited.

### 9.2 Local authentication {#prd-9-2}

Local authentication supports personal installations, emergency access, and deployments without an external identity provider.

| ID | Requirement |
|---|---|
| `SEC-LOCAL-001` | Passwords MUST be hashed using Argon2id with versioned parameters and unique salts. |
| `SEC-LOCAL-002` | Parameters MUST be configurable within safe bounds and upgraded through rehash-on-login. |
| `SEC-LOCAL-003` | Login responses MUST not reveal whether the account exists, is disabled, or has only federated identities. |
| `SEC-LOCAL-004` | Rate limits and progressive delay MUST apply by account key and network source without enabling trivial denial of service. |
| `SEC-LOCAL-005` | Password reset and recovery tokens MUST be random, single-use, expiring, stored hashed, and invalidated on relevant credential changes. |
| `SEC-LOCAL-006` | Optional TOTP, if implemented, MUST use encrypted secrets, replay prevention, recovery codes, and explicit clock-skew policy. |
| `SEC-LOCAL-007` | Disabling local login MUST NOT delete the global user, memberships, or external identities. |

### 9.3 Session management {#prd-9-3}

Browser and native clients may use server-side sessions. Sessions are opaque bearer credentials represented by high-entropy tokens; only a cryptographic digest is stored.

| ID | Requirement |
|---|---|
| `SEC-SES-001` | Session identifiers MUST be rotated after login, privilege elevation, password change, account linking, and tenant-sensitive support grants. |
| `SEC-SES-002` | Sessions have configurable idle and absolute expiry and can be revoked individually, by device, by user, or globally. |
| `SEC-SES-003` | Browser cookies MUST use `Secure`, `HttpOnly`, an explicit `SameSite` policy, a narrow path, and host-only scope unless deployment requirements demand otherwise. |
| `SEC-SES-004` | Cookie-authenticated state-changing requests MUST use CSRF protection and origin checks. |
| `SEC-SES-005` | Session records MAY retain bounded, privacy-conscious device metadata for user review and audit. |
| `SEC-SES-006` | A session does not cache permanent authorization. Membership and permission state MUST be re-evaluated according to a short bounded cache policy. |
| `SEC-SES-007` | Session and CSRF tokens MUST never appear in URLs, logs, metrics, traces, or error details. |

### 9.4 OIDC relying-party behavior {#prd-9-4}

OIDC providers are installation-administrator configured. Initial releases do not allow arbitrary tenant-provided issuer URLs.

Required flow:

1. Discover the provider from the configured issuer.
2. Validate discovery issuer equality.
3. Generate state, nonce, and PKCE verifier/challenge.
4. Store a short-lived, single-use login transaction.
5. Redirect using Authorization Code Flow.
6. Validate exact callback state and redirect URI.
7. Exchange the code server-side.
8. Validate signature, algorithm policy, issuer, audience/authorized party, nonce, expiration, not-before, issued-at, and configured clock skew.
9. Resolve `(issuer, subject)`.
10. Apply explicit provisioning and claim-mapping policy.
11. Create or rotate a local server session.
12. Consume the login transaction and audit the outcome.

| ID | Requirement |
|---|---|
| `SEC-OIDC-001` | Implicit flow, Resource Owner Password Credentials, and accepting unsigned ID tokens are prohibited. |
| `SEC-OIDC-002` | PKCE S256, state, and nonce are mandatory. |
| `SEC-OIDC-003` | JWKS caching MUST honor rotation, unknown-key refresh, bounded stale use, and denial on persistent verification failure. |
| `SEC-OIDC-004` | Supported signature algorithms are allow-listed. Algorithm selection MUST not be taken solely from untrusted token input. |
| `SEC-OIDC-005` | Claim-to-role mapping MUST use configured provider, claim path, expected value/pattern, target tenant, and target role. Arbitrary IdP role strings are not application permissions. |
| `SEC-OIDC-006` | JIT provisioning modes are disabled, invite-only, allow-listed, claim-based, or open. Invite-only is the default. |
| `SEC-OIDC-007` | Provider client secrets and refresh tokens MUST be encrypted or resolved from protected secret references and always redacted. |
| `SEC-OIDC-008` | Provider outage MAY prevent new login and token refresh but MUST not arbitrarily revoke otherwise valid local sessions. |
| `SEC-OIDC-009` | Logout behavior MUST distinguish local session termination from optional provider-initiated logout. |
| `SEC-OIDC-010` | Native clients SHOULD use Authorization Code + PKCE with an approved loopback or claimed HTTPS redirect; device flow MAY be added later. |

### 9.5 Tenant selection and context {#prd-9-5}

After authentication, a user with one active tenant MAY enter it automatically. A user with multiple active memberships MUST select an active tenant or provide one through a validated client flow.

A tenant-selection header, cookie, URL parameter, GraphQL variable, or token claim is only a request. The server MUST verify active membership and required permissions for each request.

Tenant context MUST propagate through:

- HTTP request handlers;
- application use cases;
- repository calls and transactions;
- cache keys;
- media paths/object keys;
- source preferences, cookie jars, and credentials;
- browser-worker sessions;
- durable jobs and event streams;
- tracker bindings and tokens;
- logs, traces, metrics, and audit records where safe.

### 9.6 Permissions and roles {#prd-9-6}

Authorization uses a stable permission vocabulary. Built-in roles are convenience mappings and MAY evolve only with a versioned migration policy.

Representative permissions:

```text
tenant.read
tenant.manage
members.read
members.invite
members.suspend
members.remove
roles.read
roles.manage
library.read
library.write
library.delete
reading_state.read.self
reading_state.write.self
sources.read
sources.manage
sources.credentials.manage
downloads.read
downloads.create
downloads.cancel
downloads.delete
trackers.manage.self
backups.create
backups.restore
audit.read
support.grant
```

Built-in roles SHOULD include owner, administrator, librarian, member, viewer, and automation. Authorization checks MUST ask for permissions and resource ownership, not compare role names.

### 9.7 Invitations {#prd-9-7}

Invitations carry tenant, intended identity hints if any, role assignments, expiry, inviter, state, and a hashed token. Acceptance requires authentication or a controlled authentication bootstrap. Identity hints such as email do not override `(issuer, subject)` uniqueness.

Invitation acceptance MUST atomically:

- verify token, expiry, revocation, and single-use state;
- verify tenant policy and invited identity constraints;
- create or reactivate membership under explicit policy;
- assign allowed roles;
- mark the invitation consumed;
- generate an audit event.

### 9.8 Audit and support access {#prd-9-8}

Audit events include timestamp, tenant where applicable, actor, authentication/session context reference, action, target type/ID, result, request ID, and redacted structured metadata. Audit data MUST avoid raw credentials, tokens, cookies, source bodies, and sensitive personal content.

Support access MUST be explicit, time-limited, least-privilege, revocable, visible to tenant owners, and fully audited. Hidden impersonation or universal backdoor access is prohibited.

---

## 10. Public API contract {#prd-10}

### 10.1 API families {#prd-10-1}

KathaSutra exposes:

- GraphQL for typed domain queries and mutations;
- REST for authentication, OIDC callbacks, administration, media streaming, downloads, backups, migrations, health, discovery, OPDS, and AGPL source disclosure;
- SSE for resumable operation, job, source, and download progress;
- optional private protocols for browser workers and future internal services.

The server MUST publish machine-readable API and capability versions. Clients MUST be able to determine whether a feature, source API version, and authentication method are supported without relying on a bundled UI.

### 10.2 GraphQL requirements {#prd-10-2}

| ID | Requirement |
|---|---|
| `API-GQL-001` | GraphQL objects use opaque IDs and explicit tenant/user ownership fields only where disclosure is authorized. |
| `API-GQL-002` | Lists use cursor pagination with deterministic ordering, bounded page sizes, and stable continuation semantics. |
| `API-GQL-003` | Depth, complexity, aliases, fragments, input size, result size, and execution time are bounded. |
| `API-GQL-004` | Data loaders and batching MUST preserve tenant and authorization scope in cache keys. |
| `API-GQL-005` | Mutations that may be retried support idempotency and optimistic revision inputs. |
| `API-GQL-006` | Long-running mutations return an operation handle rather than blocking until completion. |
| `API-GQL-007` | Introspection policy is configurable but schema documentation remains publishable for client authors. |
| `API-GQL-008` | Deprecated fields remain through the documented compatibility window and include replacement guidance. |

Representative mutation result:

```json
{
  "data": {
    "enqueueChapterDownload": {
      "operation": {
        "id": "op_...",
        "state": "QUEUED",
        "revision": "rev_..."
      }
    }
  }
}
```

### 10.3 REST requirements {#prd-10-3}

REST endpoints MUST use explicit versions for externally stable routes. Responses use bounded JSON or streaming bodies with consistent content types and security headers.

Representative endpoint groups:

```text
/api/v1/auth/local/*
/api/v1/auth/oidc/{provider}/*
/api/v1/sessions/*
/api/v1/tenants/*
/api/v1/media/*
/api/v1/downloads/*
/api/v1/backups/*
/api/v1/migrations/*
/api/v1/events
/api/v1/discovery
/api/v1/source
/health/live
/health/ready
/health/startup
/opds/*
```

Media endpoints MUST authorize every request or use short-lived signed grants bound to tenant, user, object, operation, and expiry. Long-lived bearer credentials in URLs are prohibited.

### 10.4 Error envelope {#prd-10-4}

Public REST errors SHOULD follow:

```json
{
  "error": {
    "code": "KS_AUTH_FORBIDDEN",
    "message": "The requested operation is not permitted.",
    "messageKey": "errors.auth.forbidden",
    "requestId": "req_...",
    "retryable": false,
    "fieldErrors": []
  }
}
```

Public errors MUST NOT include SQL, filesystem paths, stack traces, secret values, tokens, cookies, private source response bodies, or whether a cross-tenant resource exists.

### 10.5 Idempotency and concurrency {#prd-10-5}

An idempotency key is scoped by tenant, actor/client identity, operation family, normalized request hash, and expiry. Reusing a key with a different normalized request MUST fail. Reusing it with the same request returns the original result or current operation representation.

Mutable resources expose revisions. A stale expected revision returns a conflict with a safe current revision and retry guidance.

### 10.6 SSE requirements {#prd-10-6}

- Events have stable type, ID, timestamp, tenant scope, operation/resource reference, and versioned payload.
- Clients may reconnect using `Last-Event-ID` within retention limits.
- Authorization is checked at connection and periodically or when membership/session state changes.
- Per-client queues are bounded. Slow clients are disconnected or coalesced rather than growing memory indefinitely.
- Heartbeats are configurable and do not leak sensitive state.

### 10.7 Reverse proxy and origin policy {#prd-10-7}

Trusted proxy ranges and forwarded-header behavior are explicit. Untrusted forwarded headers are ignored. Public URL, cookie security, callback origins, CORS, CSRF origin checks, and WebSocket/SSE proxy behavior are validated as a coherent deployment configuration.

---

## 11. Source package and plugin contract {#prd-11}

### 11.1 Source package model {#prd-11-1}

Every source package contains or references:

- stable namespaced source ID;
- package version and source API/WIT version;
- display name, languages, content-warning classification, and homepage;
- author/maintainer and license/provenance metadata;
- capability declaration;
- allowed origin patterns and redirect policy;
- preference schema and secret-reference schema;
- implementation type: declarative or Wasm component;
- fixtures and expected golden outputs;
- package integrity information and optional signature;
- changelog and minimum server version.

Source IDs MUST not be Java class names, APK package names, or unstable numeric IDs. A recommended form is `publisher-or-community/source-name/language`.

### 11.2 Core source operations {#prd-11-2}

The versioned contract supports:

- metadata and health;
- popular and latest listings;
- search with page/cursor and filter values;
- manga/title details;
- chapter listing;
- page/resource listing;
- filter and preference schemas;
- optional authentication state checks;
- optional migration of source-specific stable keys.

Every operation returns structured source errors: invalid input, authentication required, rate limited, upstream unavailable, parsing failed, unsupported, policy denied, cancelled, resource limit exceeded, or internal plugin failure.

### 11.3 Declarative source engine {#prd-11-3}

Declarative packages MAY define:

- HTTP method, URL template, query/body/header templates;
- pagination and continuation extraction;
- HTML CSS selectors, JSON pointers/paths, and XML selectors;
- item scopes and field extractors;
- required/optional fields and fallbacks;
- transforms such as trim, normalize whitespace, regex capture, URL resolution, HTML decode, date/number parsing, join/split, and conditional selection;
- filter schemas and serialization;
- cookies, referer/origin policy, and tenant preference/secret references;
- response size, timeout, and rate-limit overrides within administrator bounds.

The engine MUST produce actionable diagnostics that identify operation, fixture, selector/path, field, and transform stage without exposing secrets.

### 11.4 WebAssembly runtime {#prd-11-4}

Executable plugins use WebAssembly Components and WIT. Wasmtime is the reference runtime.

Each invocation has:

- tenant and source context;
- explicit capability set;
- fuel or epoch deadline;
- wall-clock timeout;
- linear-memory, table, instance, and response limits;
- cancellation token;
- bounded host-call payloads;
- structured logs and metrics with redaction.

WASI filesystem, environment, process execution, raw sockets, and direct database access are disabled unless a future ADR introduces a narrowly scoped capability. Network activity occurs only through host-mediated HTTP functions that enforce policy.

### 11.5 Host capabilities {#prd-11-5}

Approved host capabilities MAY include:

- bounded HTTP requests through a shared pool;
- tenant/source cookie jar operations;
- HTML, JSON, and XML parsing handles;
- URL parse/resolve and encoding utilities;
- cryptographic hashes and non-secret encoding helpers;
- source preference reads and writes;
- opaque secret retrieval scoped to declared keys;
- bounded clock and locale information;
- logging and diagnostics;
- optional browser task submission using opaque browser-session handles.

Capabilities are deny-by-default and declared in package metadata. Administrators may further restrict them.

### 11.6 SSRF and source networking {#prd-11-6}

Source and browser networking MUST:

- parse and normalize URLs before policy checks;
- allow only supported schemes;
- resolve DNS and reject loopback, link-local, multicast, metadata, private, and administrator-denied ranges by default;
- revalidate every redirect and relevant DNS resolution;
- bound redirects, headers, request bodies, response bodies, decompression ratios, and time;
- isolate cookie jars by tenant and source;
- prevent arbitrary proxy configuration from plugins;
- record safe metrics without high-cardinality URLs or query values.

### 11.7 Package installation and updates {#prd-11-7}

Package installation validates manifest schema, identity, compatibility, integrity, signature policy, capabilities, origin policy, fixtures, and implementation payload before activation. Updates are staged, validated, atomically activated, and rollback-capable. A failed update MUST leave the previously working package available.

Global package installation is separate from per-tenant enablement, preferences, credentials, cookie jars, content policy, and rate limits.

### 11.8 Mihon source rewriting {#prd-11-8}

KathaSutra MAY analyze legally available Kotlin/Java extension source to assist manual ports. The analyzer MAY identify endpoints, selectors, filters, headers, dependencies, Android APIs, custom JavaScript, and likely declarative compatibility.

The analyzer MUST NOT:

- execute APK/JAR content;
- claim universal automatic translation;
- mark a source complete without fixtures and tests;
- copy incompatible code or content without preserving required licensing and attribution;
- bypass access controls, DRM, or legal restrictions.

Ports target the KathaSutra source contract and are maintained independently of Mihon binary compatibility.

---

## 12. Media, downloads, jobs, backups, OPDS, and trackers {#prd-12}

### 12.1 Media storage {#prd-12-1}

Filesystem object layout and S3 keys MUST begin with tenant scope. User-owned derivatives additionally include user scope where relevant. Path construction uses typed IDs and server-controlled components; source-provided filenames never become unchecked paths.

Objects have media type, size, checksum, storage key, state, creation time, and references. Writes use temporary state and atomic commit semantics appropriate to the backend. Deduplication, if implemented, MUST preserve tenant authorization and deletion correctness.

### 12.2 Download state machine {#prd-12-2}

```text
queued -> resolving -> downloading -> verifying -> committing -> complete
   |          |             |             |             |
   +----------+-------------+-------------+-------------+-> failed/cancelled
```

Retries record attempts and preserve safe resumable state. A complete chapter requires a committed manifest referencing verified media objects. Cancellation is cooperative and bounded. Cleanup jobs reclaim orphaned temporary objects.

### 12.3 Durable jobs {#prd-12-3}

Jobs include ID, tenant, initiator, type, payload version, state, priority, scheduled time, attempt, maximum attempts, lease owner/expiry, fencing token, cancellation state, progress, idempotency/deduplication key, and result/error summary.

Claiming, heartbeat, completion, and failure transitions are atomic. A worker that loses its lease cannot commit using a stale fencing token.

### 12.4 Backup and restore {#prd-12-4}

Tenant backups include tenant configuration, memberships as policy permits, roles, libraries, categories, titles, chapters, personal reading state, source configuration without plaintext secrets, tracker bindings without plaintext tokens, jobs where necessary, and optional media manifests/media.

Deployment backups additionally include installation configuration metadata, providers without plaintext client secrets, global identities, and all tenant data.

Backups use versioned manifests, checksums, dependency order, compatibility range, creation metadata, and encryption metadata if encrypted. Restore supports dry run, conflict policy, validation before mutation, transactional metadata import, media verification, and an audit trail.

### 12.5 Turso-to-PostgreSQL migration {#prd-12-5}

The supported migration workflow:

1. Validate target schema, capacity, and credentials.
2. Enter maintenance mode and stop new ordinary mutations.
3. Drain/cancel jobs according to policy.
4. Create and record a Turso checkpoint.
5. Export records in dependency order with stable IDs.
6. Import transactionally in bounded batches.
7. Rebuild indexes and PostgreSQL-specific structures.
8. Compare counts and deterministic checksums.
9. Run sampled domain/API and tenant-isolation verification.
10. Record a signed migration report.
11. Cut over only when all required verification succeeds.
12. Resume jobs and leave the source untouched until the operator confirms retention/deletion.

A failure before cutover leaves Turso authoritative and the operation resumable or safely restartable.

### 12.6 OPDS {#prd-12-6}

OPDS endpoints expose authorized catalogs and acquisition links. Feeds use stable pagination and content types. Authentication may use approved session/token mechanisms; credentials and durable bearer tokens are not embedded in feed URLs. Personal reading state appears only when requested by and authorized for that user.

### 12.7 Trackers {#prd-12-7}

Tracker accounts are personal and tenant-scoped. Tokens are encrypted and redacted. Local reading progress commits independently of tracker availability. Synchronization is asynchronous, idempotent, retryable, and conflict-aware. Conflicting local and remote updates are recorded for policy-driven or manual resolution rather than silently overwritten.
