# Security policy

KathaSutra handles authentication, tenant-scoped data, third-party credentials, untrusted source responses, and sandboxed plugin code. Please report vulnerabilities privately.

## Reporting

Use GitHub's private vulnerability reporting feature for this repository. Do not open a public issue, discussion, or pull request containing exploit details, secrets, personal data, or tenant data.

Include, where possible:

- affected commit or release;
- deployment mode and database backend;
- reproduction steps or a minimal proof of concept;
- expected and observed impact;
- whether tenant isolation, authentication, plugin isolation, storage, or secret handling is affected;
- suggested remediation, if known.

## Response objectives

Maintainers aim to acknowledge a complete report within 5 business days, assess severity within 10 business days, and coordinate remediation and disclosure. These are objectives, not warranties.

## Supported versions

Before 1.0, only the latest commit and latest published pre-release are supported. After 1.0, the support window will be documented in release policy.

## Scope priorities

High-priority reports include cross-tenant access, authentication or authorization bypass, OIDC validation errors, secret disclosure, SSRF, arbitrary filesystem/process/database access from source plugins, sandbox escape, unsafe archive/media processing, and supply-chain compromise.

Testing must use systems and accounts you own or are explicitly authorized to test. Do not access third-party content or infrastructure without permission.
