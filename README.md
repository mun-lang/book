# The Mun Programming Language

[![Netlify Status](https://api.netlify.com/api/v1/badges/2f75b222-0ec6-4fa5-b2be-be25ee1dde14/deploy-status)](https://app.netlify.com/sites/docs-mun-lang/deploys)

This repository contains the source of "The Mun Programming Language" book. It
is hosted on [netlify](https://www.netlify.com/).

## Requirements

Building the book requires
[mdBook](https://github.com/rust-lang-nursery/mdBook), ideally version 0.3.x. To
install it, run:

```
$ cargo install mdbook --vers [version-num]
```

## Building

The Mun book uses a [custom version of
Highlight.js](https://github.com/mun-lang/highlight.js) to enable highlighting
of Mun code. The build version of Highlight.js is required by mdbook in the
theme/ folder but it is not distributed with the source. Instead, it can be
build by invoking the build script:

```bash
./ci/build-highlight-js
```

Every time you change something in the custom version of highlight.js you have
to call the above script to ensure you locally use the latest version.

After generating the custom minified Highlight.js, to build the book, type:

```
$ mdbook build 
```

The output will be in the book subdirectory. To view the book, open it in your web
browser.

### Local development

For local development use `mdbook serve` instead of `mdbook build`. This will
start a local webserver on port `3000` that serves the book and rebuilds the
content when changes are detected.

### Single command

All of the above is also combined in a single shell script that can be invoked
by simply running:

```bash
./ci/build
```

The Netlify deployment works by simply invoking this script. 

### Testing

To test the `rust` source code in the book, run `mdbook -L path/to/target/debug/deps`. For this to
work, there can only be one `libmun_runtime-{HASH}.rlib` file in the provided library path.
