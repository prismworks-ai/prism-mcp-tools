# Prism MCP Tools product strategy

Effective: 2026-09-27. Source-review evidence below retains its original review date, 2026-09-24. Active direction for new work. Implementation is tracked in the [plan](implementation-plan.md); proposed additions are not shipped features.

## Authority ecosystem alignment — 2026-09-27

PrismWorks builds authority infrastructure for autonomous AI. Databridle's first commercial package is delegated financial operations in existing ERP/payment environments. Its core object is a business mandate with authenticated delegation rights, trusted purpose/business objects, attenuated child grants, shared limits, expiry and revocation. Engineering/IT and security response expand the same core after the initial package is repeatable. This direction supersedes engineering-first or security-platform positioning in earlier planning.

**Prism MCP Tools's boundary:** Maintained protocol examples and interoperability fixtures, not an enterprise payment or authorization service.

**Required implementation change:** After existing dependency/API repairs, demonstrate a synthetic financial executor with exact request binding and explicit success/failure/unknown results. Label mock behavior and avoid real money. Verify replacement hosts/providers before publishing compatibility claims.

Carry mandate/root/parent/grant and current-authority revision through a versioned adapter, with exact request digest and reservation reference when applicable. Databridle's runtime Authority Graph and transactional reservation ledger are authoritative; no accelerator keeps a competing writable copy. Workflow/evidence dependencies do not grant authority. Child grants cannot multiply aggregate limits or silently combine permissions.

**Acceptance before claiming integration:** issuer has the right to delegate; changed beneficiary/resource/request rejected; cross-tenant references rejected; concurrent descendants cannot overrun a shared root limit; accepted-but-timed-out writes remain unknown and reserved until reconciled; revoked/stale authority cannot authorize the next protected use. Apply each check at the component's advertised boundary and use explicit substitution fixtures for responsibilities owned elsewhere. No real financial transaction is required.

**Scope removed or deferred:** independent enterprise authority platform, graph administration/federation ahead of runtime correctness, and any mandatory all-accelerator installation. Preserve existing useful behavior, licenses, ownership and independent use. These are planned adapter requirements, not a claim of deployed capabilities. Existing remediation and migration checks below remain prerequisites.

## Purpose and boundary

**Runnable examples, interoperability fixtures and diagnostics for the supported MCP integration path.**

Own a small maintained set of example executors/clients and test utilities. SDK owns protocol implementation; Databridle owns security policy. Examples are not production applications without deployment and security acceptance.

## Current repository evidence

Review of the local working tree, including pre-existing changes; source presence is not deployment evidence.

| Source | Observation |
| --- | --- |
| [prism-mcp-servers/database_server/Cargo.toml](../prism-mcp-servers/database_server/Cargo.toml) | Depends on local prism-mcp-rs ^0.1.0; adjacent SDK is 3.0.1. |
| [prism-mcp-clients/http_client/Cargo.toml](../prism-mcp-clients/http_client/Cargo.toml) | Client examples have the same old SDK dependency family. |
| [transport_benchmark/Cargo.toml](../transport_benchmark/Cargo.toml) | Local SDK relative path and version need reconciliation. |
| [prism-test-utils/Cargo.toml](../prism-test-utils/Cargo.toml) | Test utility also requests SDK ^0.1.0. |
| [mcp-inspector/Cargo.toml](../mcp-inspector/Cargo.toml) | SDK dependency is commented; inspector is not proof of a working protected integration. |

Verification this review: cargo metadata --offline --format-version 1 in database_server failed: requested SDK ^0.1.0 does not match local 3.0.1. This is a confirmed compatibility blocker, not merely an unrun test.

## Retain

Retain working behavior, user data, existing integrations, tests, licensing and independent use within the boundary above. Maintain existing support obligations. Preserve local changes from other work; this review does not release or deploy code.

## Remove or defer

- Remove blanket production-ready claims and nonexistent CLI/template capabilities from current documentation.
- Defer broad benchmark/transport catalogue; archive unused examples only after usage review, not as part of this documentation edit.

## Modify

- Migrate manifests and actual APIs to a selected compatible SDK, including correct relative paths or reproducible release dependencies.
- Select one HTTP client/server pair and test utility as the maintained reference; leave other examples explicitly experimental until checked.
- Use synthetic resources and restricted execution defaults; no real production change in sample workflows.

## Add

- A reproducible protected incident-workflow fixture with allow/deny/approval/unknown/revoke cases.
- Fresh-clone build instructions, feature/version matrix and structured evidence outputs.
- Negative security fixtures reusable by Databridle and independent SDK users.

## Integration rules

This is the target integration design, not a claim of an existing Databridle adapter. Components communicate through versioned contracts and keep independent storage. No shared database, forced cloud account or mandatory all-product installation.

The host proposes work; Databridle decides protected enterprise actions; the credential provider enforces its own access conditions; the executor performs only the bound action. Local restrictions can deny but cannot widen an enterprise grant. Human software acceptance and credential approval remain distinct from exact-action security approval.

Carry tenant/principal/delegation/task/run/action identifiers, policy revision, exact request digest, expiry and decision/receipt references through authenticated adapters. Treat client-supplied identity and trace fields as untrusted until bound by the trusted host. Keep secrets and raw sensitive payloads out of default logs, prompts and project memory. Distinguish allow/deny from executed/failed/unknown and from independent verification.

Protected mode stops on missing authority or unavailable required controls; it cannot silently invoke an unprotected path. Retries need idempotency or reconciliation. Revocation of future access does not undo completed work or erase credentials already delivered. Record actual transport, version and bypass coverage before describing an integration as supported.

## Investment and success

Prioritize a supported, reusable protected workflow over feature breadth. Measure integration effort, correctly completed work, denied unauthorized operations, evidence completeness and ongoing maintenance. This component's role does not create a new license, transfer IP or approve a pricing/partnership claim.
