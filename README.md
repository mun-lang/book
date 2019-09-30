# The Mun Programming Language

[![Netlify Status](https://api.netlify.com/api/v1/badges/2f75b222-0ec6-4fa5-b2be-be25ee1dde14/deploy-status)](https://app.netlify.com/sites/naughty-hopper-ca6ddd/deploys)

This repository contains the source of "The Mun Programming Language" book. It is hosted on [netlify](https://www.netlify.com/).

## Requirements

Building the book requires
[mdBook](https://github.com/rust-lang-nursery/mdBook), ideally version 0.3.x. To install it, run:

```
$ cargo install mdbook --vers [version-num]
```

## Building

To build the book, type:

```
$ mdbook build 
```

The output will be in the book subdirectory. To view the book, open it in your web
browser.
