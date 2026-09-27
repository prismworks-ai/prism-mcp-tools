# Prism MCP Tools implementation plan

Effective: 2026-09-24. Status: planned integration work, not a completed release. Governed by [product strategy](product-strategy.md). Owner: component maintainer; cross-product authorization changes also require review by the Databridle security owner. These are accountable roles, not assumed hires.

## Baseline

cargo metadata --offline --format-version 1 in database_server failed: requested SDK ^0.1.0 does not match local 3.0.1. This is a confirmed compatibility blocker, not merely an unrun test. Existing code and user changes are preserved. New checks below are required acceptance work, not claims that those checks already pass.

## Ordered backlog

| ID | Priority | Change | Deliverable | Dependency | Acceptance evidence |
| --- | --- | --- | --- | --- | --- |
| MT-01 | P0 | Modify | Repair dependency versions/paths and migrate API calls for the chosen pair. | SDK maintainer selects supported version. | Fresh clone resolves and builds without undeclared sibling layout; no version-only fix accepted. |
| MT-02 | P1 | Add | Implement protected reference example and negative fixtures. | G1/G2 and SDK hooks. | Allowed/denied/approved/unknown/revoked outcomes are reproducible with synthetic data. |
| MT-03 | P1 | Modify | Label each remaining example by verified support status. | Per-example build inventory. | README lists only existing paths; no claim of a nonexistent CLI or production readiness. |
| MT-04 | P2 | Remove / defer | Retire unused duplicate examples after migration; no broad catalogue expansion. | Consumer/owner review. | Replacement and rollback notes for every removed consumer path. |

P0 establishes correctness and scope; P1 completes a supported integration; P2 is conditional expansion. Work may run concurrently once interface dependencies are agreed. No calendar duration or production availability is implied.

## Contract acceptance

- Pin supported component, profile and transport versions; reject unsupported security semantics.
- Verify tenant/principal/resource binding, changed arguments, expiry, replay, revocation, retries and missing context on every advertised protected path.
- Keep authorization verdict, execution result and independent verification distinct; interrupted or unobserved effects remain unknown.
- Keep credentials out of prompts/logs/general memory; test redaction and controlled evidence access.
- Demonstrate independent use and supported replacement components; fail closed in protected mode when required controls are unavailable.

## Migration and removal

Before executable code removal, inventory consumers and stored data; add replacement/version migration and compatibility tests; communicate deprecation; ship export/restore and rollback instructions. Do not delete historical evidence, encrypted secrets or user data as documentation cleanup. No published package/API identifier or license changes without a separate compatibility and ownership decision.

## Release definition

Record exact commit/version, supported paths, test commands/results, known limits, installer/upgrade steps and accountable maintainer. Mock tests and schema checks are not production security evidence. Update README and compatibility documentation from the passing report, not from planned checkboxes. No universal integrated/production-ready claim until the relevant execution path passes the complete acceptance matrix.

## Dependency gate reference

G1 freezes validated integration contracts and fixtures. G2 implements Databridle’s protected action/approval/receipt path. G3 connects supervision. G4 validates RabbitLock credential delivery. G5 proves interoperability and replacement hosts/providers. G6 packages a supported release. References to these gates are dependencies, not claims of completion; standalone open-source use remains independent.
