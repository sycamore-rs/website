# [sycamore.dev](https://sycamore.dev)

This is the official website for [Sycamore](https://github.com/sycamore-rs/sycamore). This site hosts docs and blog posts for Sycamore.

## Building the site

Build the site using `trunk build`. This will generate a `dist/` folder which can then be served with a simple web server.

For development, you want to use `make serve` instead. The reason why we don't use `trunk serve` directly is because Trunk does not automatically strip the `.html` suffix off of the generated pages.
