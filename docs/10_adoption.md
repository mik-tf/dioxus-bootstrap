# Adoption Log

This crate is Apache-2.0, so anyone may fork it, rename it, and grow it. Where a
fork has built something worth having, we adopt the *idea* by implementing it
here — read the source, write our own, record what was adopted.

This chapter is that record. It exists so that "we took the good bits from
downstream" is a checkable statement rather than an untracked divergence.

## Rules

1. **Content, never an address.** Record what was assessed by content hash and
   version. Never add a fork as a git remote, submodule, or path dependency, and
   never point crate metadata or documentation at one. A dependency on somebody
   else's hosting is a dependency this crate does not have and should not acquire.
2. **The parity contract decides.** See the Design chapter. Something is adopted
   only if Bootstrap 5.3 defines it and this crate does not yet express it. A
   fork's addition does not become crate API merely because the fork found it
   useful — that is precisely the "extra components that are not Bootstrap
   components" the contract excludes.
3. **Declining is a result.** Record what was assessed and rejected, with the
   reason. An assessment that only records adoptions cannot be distinguished from
   one that was never done.

## 2026-07-27 — `ui_components` 0.6.0

**Assessed:** `ui_components` 0.6.0, a downstream fork of this crate (renamed,
Apache-2.0, restructured into `src/elements/` + `src/widgets/` + `catalog.rs`).
Upstream state `e5f7ea31489c`, 2026-07-27. Compared against `dioxus-bootstrap-css`
0.5.15.

### Element layer — at parity, nothing adopted

Module-for-module comparison of the fork's `src/elements/` against this crate's
`src/`: **34 of 35 modules correspond**. The crate is not behind.

The single module without a counterpart here is `embed.rs`, and it is **declined**
as out of contract:

| | |
|---|---|
| sha256 | `88f034ba795d5dddccce9761584d640392b72f0a1331f4722aa0966813f7e989` |
| what it is | a full-bleed `<iframe>` with hardcoded inline sizing (`min-height:480px`), plus a URL percent-encoding helper |
| why declined | Bootstrap 5.3 defines no `Embed` component; the inline sizing is custom CSS beyond Bootstrap; the module documents itself as the shared primitive that the fork's per-service component crates delegate to, which is application infrastructure |
| Bootstrap's own answer | responsive embeds are `ratio`, which this crate already exposes as `Ratio` |

Adopting it would have added an app-level primitive and a URL utility to a crate
whose contract is "typed Dioxus over Bootstrap, nothing else."

### Widget layer — declined in full

The fork adds a second layer, `src/widgets/` (18 modules), described upstream as
"pure arrangements … composed from the Bootstrap 5.3 element primitives."

**None of it is adopted, and none of it should be.** Measured against the Design
chapter's "What Does Not Belong In The Crate", every module falls outside:

| Modules | Why they are out of contract |
|---|---|
| `planning` (Kanban, Calendar), `activity` (Timeline, CommentThread, ChatLog), `people` (Avatar, ContactList), `media` (MarkdownViewer, Gallery, AudioBar, VideoChrome), `charts` (BarChart, Sparkline, DonutRing) | Bootstrap 5.3 defines none of these. They are application features, not Bootstrap components. |
| `shell`, `shell_impls`, `landing`, `layout` | App-specific page layout and branding — a self-described "opinionated shell *implementation*", plus marketing-page bands and mastheads. The contract excludes opinionated defaults Bootstrap does not define. Where a Bootstrap equivalent exists (`Drawer` ≈ Offcanvas, `TabbedPanel` ≈ Tabs), this crate already ships it. |
| `display` (DataTable, StatCard, EntityCard, ServiceTile, Kpi), `entities`, `feedback` (EmptyState, Rating, Stepper), `controls` (ConfirmButton, ContextMenu) | Compositions and invented components. Several are shaped around the fork's own service model rather than anything general. |
| `crud`, `health` | Carry a backend/transport notion (`Backend`, `backend_fn`, `RpcBackendCheck`). This crate has no transport and should acquire none. |
| `forms` (`TextField`, `SelectField`, `FieldGrid`, `TagInput`, …) | The closest call. Bootstrap does define form layout, floating labels and input groups — and this crate already exposes them as `FormGroup`, `Input`, `Select`, `Textarea`, `FloatingLabel`, `InputGroup`, `FormFeedback`, `FormText`. The `*Field` types bundle label + control + help + error into one opinionated arrangement, which is a consuming application's choice to make, not the crate's. `TagInput` is not a Bootstrap control at all. |
| `format` (`money`, `thousands`, `initials`, `epoch_date`) | Not UI. Formatting helpers belong to the application. |
| `style` (`ComponentsHead`) | Ships the widget layer's own stylesheet. The contract's "custom CSS beyond what Bootstrap provides" excludes exactly this — and its existence is the clearest evidence that the widget layer is a design system of its own rather than a Bootstrap expression. |

**The conclusion is not that the widget layer is bad work.** It is that it is a
different product: an application component library that happens to sit on
Bootstrap. This crate is the Bootstrap layer underneath, and the Design chapter's
rule — *if Bootstrap does not define the behavior, it belongs in the consuming
application* — is what keeps it project-agnostic and therefore usable by any
Dioxus project.

Applications wanting a Kanban board or a data table should build them on top of
this crate, which is the arrangement the fork itself uses.
