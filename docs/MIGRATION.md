# Migration

This file is kept as a stable link for older references. The canonical guide is
[`MIGRATION_GUIDE.md`](MIGRATION_GUIDE.md).

Use the guide's full workflow:

1. Convert raw Bootstrap RSX with `tools/migrate-bootstrap-rsx.mjs`.
2. Treat `tools/check-no-raw-bootstrap.mjs` as a completeness gate, not a
   visual-fidelity proof.
3. Let the converter map Bootstrap intent to typed props.
4. Flag dynamic or ambiguous class strings instead of guessing.
5. Prove the result with cargo checks and Playwright visual regression.

Core rule: lint-green is not the same as visually identical. A migration is
done only when typed props preserve the original Bootstrap intent and screenshots
prove no meaningful drift.
