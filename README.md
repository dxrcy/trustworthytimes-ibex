# Ibex Template

Simple template for an [Ibex](https://github.com/darccyy/ibex) SSG site.

- [Live on GitHub Pages](https://darccyy.github.io/ibex-template)

# Usage

```sh
# Clone the repo
git clone https://github.com/darccyy/ibex-template my-ibex-app
cd my-ibex-app

# Install some dev dependencies
cargo install just cargo-watch basic-http-server

# Build and open a local server on localhost:4000
just serve
```

## GitHub Pages Setup

GitHub will automatically build to the `gh-pages` branch, with the `deploy.yaml` Action. The action will automatically run when the repository is initially cloned.

In repository settings, navigate to the `Pages` tab, and change 'Branch' to `gh-pages`. GitHub pages will automatically update, and the website should be live soon.

![Ibex logo](static/icon.png)

