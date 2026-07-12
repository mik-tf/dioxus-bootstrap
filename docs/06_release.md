# Release

`dioxus-bootstrap-css` releases are cut from the Forge primary repository and
mirrored to GitHub.

> **Publishing is automated by the tag push.** Step 7 (pushing the `vX.Y.Z` tag)
> triggers the Forge `Release` workflow (`.forgejo/workflows/release.yml`), which runs
> the gates and `cargo publish`es to crates.io — guarded by an existing-version check,
> so it safely skips if that version is already on crates.io. **Never run
> `cargo publish` manually**; the tag does it (a manual publish is redundant at best and
> races the workflow at worst). GitHub release + Pages are automated mirror outputs, so
> the GitHub remote is never pushed by hand either.

## Repositories

- Primary repository: https://forge.ourworld.tf/lhumina_code/dioxus-bootstrap-css
- GitHub mirror and Pages host: https://github.com/mik-tf/dioxus-bootstrap-css
- Crate: https://crates.io/crates/dioxus-bootstrap-css

`development` is the release branch. Tags use the crate version with a leading
`v`, for example `vX.Y.Z`.

## Checklist

1. Confirm the worktree is clean and `development` is current.

   ```bash
   git status --short --branch
   git fetch origin
   git fetch github
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

6. Push `development` to the Forge primary. Do **not** push the GitHub remote by
   hand — GitHub is a pull-based mirror (see the note below).

   ```bash
   git push origin development
   ```

7. Create and push the release tag to the Forge primary. The tag is what
   triggers publishing.

   ```bash
git tag -a vX.Y.Z -m "dioxus-bootstrap-css X.Y.Z"
git push origin vX.Y.Z
   ```

8. Verify the automated release flow. crates.io publishes within minutes; the
   GitHub mirror, release, and Pages follow on the scheduled sync (a manual
   `Sync from Forge` dispatch on GitHub speeds it up).

   - Forge release workflow succeeds.
   - Forge branch CI succeeds.
   - crates.io shows the new version.
   - crates.io repository metadata points to the Forge primary repository.
   - GitHub sync mirrors `development` and the new tag, then the GitHub release
     workflow and Pages rebuild succeed.
   - Live showcase loads from https://mik-tf.github.io/dioxus-bootstrap-css/

## Notes

- Forge is the only release publisher.
- GitHub release and Pages are mirror outputs.
- Do not include credentials, private machine details, or operational secrets in
  release commits, issues, tags, logs, or documentation.
