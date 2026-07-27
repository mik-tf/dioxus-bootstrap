# Release

`dioxus-bootstrap-css` is released from this repository. There is no upstream
and no mirror: the tag you push here is the release.

> **Publishing is automated by the tag push.** Step 7 (pushing the `vX.Y.Z` tag)
> triggers the `Release` workflow (`.github/workflows/release.yml`), which checks
> that the tag matches the crate version, runs the gates, publishes to crates.io,
> and creates the GitHub Release. The publish step is guarded by an existing-version
> check, so it safely skips if that version is already on crates.io. **Never run
> `cargo publish` manually** — the tag does it, and a manual publish is redundant at
> best and races the workflow at worst.

## Repositories

- Repository, release publisher, and Pages host: https://github.com/mik-tf/dioxus-bootstrap-css
- Crate: https://crates.io/crates/dioxus-bootstrap-css

`development` is the release branch. Tags use the crate version with a leading
`v`, for example `vX.Y.Z`.

## Prerequisites

The `Release` workflow needs a `CARGO_REGISTRY_TOKEN` repository secret — a
crates.io API token with publish rights for this crate. Without it the workflow
fails at the publish step with an explicit error rather than silently skipping.

crates.io ownership should not rest on a single account. Add a co-owner with
`cargo owner --add <user> dioxus-bootstrap-css`; a sole owner is the one failure
mode that no amount of repository hosting protects against.

## Checklist

1. Confirm the worktree is clean and `development` is current.

   ```bash
   git status --short --branch
   git fetch origin
   ```

2. Move the current changelog notes from `Unreleased` to the new version.

   ```markdown
## [Unreleased]

## [X.Y.Z] - Short release title
   ```

3. Bump the crate version in `crates/dioxus-bootstrap/Cargo.toml`.

   ```toml
version = "X.Y.Z"
   ```

4. Run the release checks.

   ```bash
   cargo +1.96 fmt --all -- --check
   cargo +1.96 test -p dioxus-bootstrap-css
   cargo +1.96 package -p dioxus-bootstrap-css --allow-dirty
   npm run lint:bootstrap
   npm run test:migrate
   npm run test:e2e -- --reporter=list
   ```

   For component or example changes, also run the relevant wasm checks and
   clippy commands before tagging.

5. Commit the release metadata.

   ```bash
   git add CHANGELOG.md crates/dioxus-bootstrap/Cargo.toml
git commit -m "chore: release dbcss X.Y.Z"
   ```

6. Push `development`.

   ```bash
   git push origin development
   ```

7. Create and push the release tag. The tag is what triggers publishing.

   ```bash
git tag -a vX.Y.Z -m "dioxus-bootstrap-css X.Y.Z"
git push origin vX.Y.Z
   ```

8. Verify the automated release flow.

   - The `Release` workflow succeeds.
   - Branch CI succeeds.
   - crates.io shows the new version.
   - crates.io repository metadata points to this GitHub repository.
   - The GitHub Release exists for the tag.
   - Pages rebuilds and the live showcase loads from
     https://mik-tf.github.io/dioxus-bootstrap-css/

## Notes

- This repository is the only release publisher.
- A crates.io publish is irreversible — yank is the only undo. The workflow's
  existing-version guard fails closed: if the registry cannot be read, the job
  stops rather than assuming the version is new.
- Do not include credentials, private machine details, or operational secrets in
  release commits, issues, tags, logs, or documentation.
