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

The site currently loads docs and blog posts from `checkouts/main/docs`.

Build the site using `trunk build`. This will generate a `dist/` folder which can then be served with a simple web server.

For development, you want to use `make serve` instead. The reason why we don't use `trunk serve` directly is because Trunk does not automatically strip the `.html` suffix off of the generated pages.
