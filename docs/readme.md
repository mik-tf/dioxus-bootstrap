# dioxus-bootstrap-css Documentation

This directory contains the repository documentation for `dioxus-bootstrap-css`
(`dbcss`), generated as a `hero_doc_generator` booklet.

## Files

- `booklet.toml` — booklet configuration (title, author, template, output formats).
- `content/` — numbered Markdown chapters that make up the document.
- `readme.md` — this file.

## Build the docs

From this directory:

```bash
hero_doc_generator build .
```

By default the generated ebook, HTML, and PDF files are written to
`~/Downloads/dioxus-bootstrap-css/`. Use the `--out` flag to choose a different
output directory:

```bash
hero_doc_generator build . --out ../target/docs
```

To list available templates:

```bash
hero_doc_generator templates
```

To see the processing order of the Markdown files:

```bash
hero_doc_generator check .
```

Only the Markdown source under `content/` is committed; built ebook/HTML/PDF
output is not checked in, and there is no docs CI job.

## Adding chapters

Add new numbered Markdown files to `content/`, for example `08_new_topic.md`.
The generator processes files alphabetically, so keep the `NN_` prefix to
maintain order.
