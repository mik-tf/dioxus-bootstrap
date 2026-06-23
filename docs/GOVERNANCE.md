# Governance

`dioxus-bootstrap-css` (`dbcss`) is maintained from the Forge primary
repository:

https://forge.ourworld.tf/lhumina_code/dioxus-bootstrap-css

The GitHub repository is kept as a mirror and GitHub Pages host:

https://github.com/mik-tf/dioxus-bootstrap-css

## Development Home

Use the Forge repository for issues, pull requests, release coordination, and
maintenance discussion. GitHub remains useful for visibility, mirroring, and the
live showcase, but Forge is the source of truth for development work.

## Design Rule

If Bootstrap does it, dbcss should expose a typed Dioxus way to express it. If
Bootstrap does not do it, dbcss should not invent it.

That means parity gaps should be fixed in the crate API, converter, migration
gate, examples, or documentation instead of being normalized as downstream raw
Bootstrap workarounds.

## Maintainer Expectations

- Keep `development` green and release-ready.
- Keep the GitHub mirror current after Forge changes.
- Keep the live showcase and examples dogfooding dbcss components.
- Keep the converter and raw-Bootstrap migration gate aligned with the component
  surface.
- Update `CHANGELOG.md`, `README.md`, and `docs/` when behavior, release policy,
  migration rules, or repository locations change.
- Track Bootstrap parity gaps as Forge issues with clear acceptance criteria.

## Release Policy

`development` is the release branch. Release tags use the crate version with a
leading `v`, for example `vX.Y.Z`.

Follow [RELEASE.md](RELEASE.md) for the release checklist. Public release notes
should describe user-visible behavior, migration impact, documentation changes,
and validation results.

## Public Issue Policy

Public issues and comments should focus on code behavior, user-facing
documentation, release state, and contributor-facing process. Private
administrative details stay out of public repository docs and issues.
