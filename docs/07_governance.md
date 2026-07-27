# Governance

`dioxus-bootstrap-css` (`dbcss`) is developed in this repository:

https://github.com/mik-tf/dioxus-bootstrap-css

It is the origin, the release publisher, and the GitHub Pages host. There is no
upstream and no mirror.

## Development Home

Use this repository for issues, pull requests, release coordination, and
maintenance discussion. It is the single source of truth for development work,
and crates.io is the only distribution channel.

The crate is project-agnostic by design: it is a typed layer over Bootstrap 5.3
and nothing else. It carries no assumptions about the application consuming it,
which is what makes it usable by any Dioxus project.

### Adopting work from downstream forks

The crate is Apache-2.0, so anyone may fork it, rename it, and grow it. Where a
fork has built something worth having, adopt the *idea* by implementing it here:
read the source, write our own, and record what was adopted and from which
upstream state.

Two rules keep that honest:

- **Content, never an address.** Record what came in by content hash in the
  [Adoption Log](10_adoption.md). Do not add a fork as a git remote, submodule,
  or path dependency, and do not point crate metadata or documentation at one. A
  dependency on somebody else's hosting is a dependency we do not have today
  and should not acquire.
- **The parity contract still decides.** A fork's additions are adopted only if
  they pass the Design chapter's test — Bootstrap defines it, so we express it.
  Application-level arrangements do not become crate API because a fork put
  them in its own copy.

## Design Rule

If Bootstrap does it, dbcss should expose a typed Dioxus way to express it. If
Bootstrap does not do it, dbcss should not invent it.

That means parity gaps should be fixed in the crate API, converter, migration
gate, examples, or documentation instead of being normalized as downstream raw
Bootstrap workarounds.

## Maintainer Expectations

- Keep `development` green and release-ready.
- Keep the live showcase and examples dogfooding dbcss components.
- Keep the converter and raw-Bootstrap migration gate aligned with the component
  surface.
- Update `CHANGELOG.md`, `README.md`, and `docs/` when behavior, release policy,
  migration rules, or repository locations change.
- Track Bootstrap parity gaps as GitHub issues with clear acceptance criteria.

## Release Policy

`development` is the release branch. Release tags use the crate version with a
leading `v`, for example `vX.Y.Z`.

Follow [the Release chapter](08_release.md) for the release checklist. Public release notes
should describe user-visible behavior, migration impact, documentation changes,
and validation results.

## Public Issue Policy

Public issues and comments should focus on code behavior, user-facing
documentation, release state, and contributor-facing process. Private
administrative details stay out of public repository docs and issues.
