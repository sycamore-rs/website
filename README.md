# [sycamore.dev](https://sycamore.dev)

This is the official website for [Sycamore](https://github.com/sycamore-rs/sycamore). This site hosts docs and blog posts for Sycamore.

## Building the site

Initialize the documentation checkouts before building:

```sh
git submodule update --init --recursive
```

The submodules under `checkouts/` track Sycamore's development and release
branches. Their tracking branches are configured in `.gitmodules`, which the sync
workflow uses to update all checkouts without a hard-coded version list. Builds
use the committed submodule revisions.

To update the checkouts locally:

```sh
git submodule update --remote --recursive
```

The site loads documentation from `docs/next/` in each configured checkout.
`checkouts/main` supplies the unprefixed latest development docs and blog posts;
other checkouts supply release docs under `/book/<checkout-name>/...`.

To add a hosted release, add a submodule under `checkouts/` with a URL-safe name
and configure its tracking branch in `.gitmodules`. The build reads this
configuration to generate the version list, routes, and navigation. Each checkout
must contain `docs/next/sections.json` and its referenced Markdown files.

Release documentation is marked `noindex` and excluded from the sitemap. Only
latest documentation has canonical URLs and is included in the sitemap; release
pages remain crawlable so search engines can discover the `noindex` directive.
The older external documentation sites remain linked in the version navigation.

`DOCS_DIR` can override the latest checkout's `docs/` directory at compile time;
release checkouts continue to use their configured paths.

Build the site using `trunk build`. This will generate a `dist/` folder which can then be served with a simple web server.

For development, you want to use `make serve` instead. The reason why we don't use `trunk serve` directly is because Trunk does not automatically strip the `.html` suffix off of the generated pages.
