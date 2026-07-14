# Project governance

KathaSutra is currently maintained under a benevolent-maintainer model while the project establishes its contributor community.

## Roles

### Contributors
Anyone who submits code, documentation, testing, design feedback, translations, source packages, or issue analysis under the repository's contribution rules.

### Reviewers
Contributors trusted to review one or more areas. Review authority is scoped; security-sensitive changes still require a maintainer.

### Maintainers
Maintainers may merge changes, manage releases, triage security reports, approve ADRs, and enforce project policies. The initial maintainer is `@thorion3006`.

## Decision process

Routine implementation decisions are made through issue and pull-request review. Decisions that change architecture, public APIs, database semantics, authentication, tenant isolation, WIT contracts, licensing, or compatibility policy require an Architecture Decision Record and PRD update where applicable.

Maintainers should seek rough consensus. When consensus cannot be reached, the responsible maintainer records the decision and rationale in an ADR. Security and legal constraints may override feature preferences.

## Changes to governance

Governance changes require a public pull request, a stated transition plan, and maintainer approval. As the project gains sustained reviewers and contributors, governance should evolve toward multiple maintainers with documented nomination, removal, and succession processes.

## Conduct and conflicts

All participation is governed by `CODE_OF_CONDUCT.md`. Reviewers must disclose material conflicts of interest, especially around hosted services, source registries, commercial distributions, or security vendors.
