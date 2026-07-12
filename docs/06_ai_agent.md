# Migrating with an AI agent

The migration is mechanical enough to hand to an AI coding agent, provided the
agent is held to the same rules a careful human would follow: identify the source
form, preserve intent, never keep Bootstrap JS, and prove the result objectively
instead of asking a human to eyeball it.

## Agent prompt

Use this when asking an AI coding agent to migrate a page:

```text
Convert Bootstrap HTML/RSX to Dioxus RSX using dioxus-bootstrap-css 0.5.

Rules:
1. First identify each control's source form: static markup, declarative data-bs-*,
   or imperative JS / a web component. For the imperative form, read the JS to
   recover the Bootstrap options AND the exact HTML it injects — the markup alone
   is not enough.
2. Match the existing DOM structure unless a typed component requires a wrapper.
3. Replace Bootstrap JS behavior with Dioxus signals. Never keep data-bs-*
   attributes or Bootstrap JavaScript.
4. Map component intent to typed props; do not drop color, outline, size, href,
   target, state, or slot information. Preserve residual utility/layout classes.
5. Flag dynamic or ambiguous class strings and Bootstrap attributes instead of guessing.
6. Prove it objectively: capture the original (golden) and the converted (candidate)
   yourself in a headless browser at a fixed viewport with a solid background, then
   compare geometry (getBoundingClientRect) AND pixels (tools/visual-parity.mjs).
   Never ask a human to diff screenshots. Classify every delta intended vs regression.
7. If the crate cannot reproduce a Bootstrap behaviour, fix the crate and add a test.
   Never hand-patch the app.
```

Each rule maps to an earlier chapter: rule 1 to "Reading the original", rules 2–5
to the converter and component mapping, and rule 6 to the objective parity gate.
Rule 7 is the design rule — parity gaps are fixed in the crate, never worked
around downstream.
