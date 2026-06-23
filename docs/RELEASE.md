# Release

`dioxus-bootstrap-css` releases are cut from the Forge primary repository and
mirrored to GitHub.

## Repositories

- Primary repository: https://forge.ourworld.tf/lhumina_code/dioxus-bootstrap-css
- GitHub mirror and Pages host: https://github.com/mik-tf/dioxus-bootstrap-css
- Crate: https://crates.io/crates/dioxus-bootstrap-css

`development` is the release branch. Tags use the crate version with a leading
`v`, for example `v0.5.4`.

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

   ## [0.5.4] - Short release title
   ```

3. Bump the crate version in `crates/dioxus-bootstrap/Cargo.toml`.

   ```toml
   version = "0.5.4"
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
   git commit -m "chore: release dbcss 0.5.4"
   ```

6. Push `development` to Forge primary and GitHub mirror.

   ```bash
   git push origin development
   git push github development
   ```

7. Create and push the release tag to both remotes.

   ```bash
   git tag -a v0.5.4 -m "dioxus-bootstrap-css 0.5.4"
   git push origin v0.5.4
   git push github v0.5.4
   ```

8. Verify the automated release flow.

   - Forge release workflow succeeds.
   - Forge branch CI succeeds.
   - crates.io shows the new version.
   - crates.io repository metadata points to the Forge primary repository.
   - GitHub release workflow succeeds.
   - GitHub Pages rebuild succeeds.
   - Live showcase loads from https://mik-tf.github.io/dioxus-bootstrap-css/

## Notes

- Forge is the only release publisher.
- GitHub release and Pages are mirror outputs.
- Do not include credentials, private machine details, or operational secrets in
  release commits, issues, tags, logs, or documentation.
