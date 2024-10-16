# Hello World - Counting Edition

This is a fairly minimal app that show a "Hello World" web app in Rust with an interactive counter.

## Usage

### Installation

If you don't already have it installed, it's time to install Rust: <https://www.rust-lang.org/tools/install>.
The rest of this guide assumes a typical Rust installation which contains both `rustup` and Cargo.

To compile Rust to WASM, we need to have the `wasm32-unknown-unknown` target installed.
If you don't already have it, install it with the following command:

```bash
rustup target add wasm32-unknown-unknown
```

Now that we have our basics covered, it's time to install the star of the show: [Trunk].
Simply run the following command to install it:

```bash
cargo install trunk wasm-bindgen-cli
```

That's it, we're done!

### Running

```bash
trunk serve
```

Rebuilds the app whenever a change is detected and runs a local server to host it.

There's also the `trunk watch` command which does the same thing but without hosting it.

### Release

```bash
trunk build --release
```

This builds the app in release mode similar to `cargo build --release`.
You can also pass the `--release` flag to `trunk serve` if you need to get every last drop of performance.

Unless overwritten, the output will be located in the `dist` directory.

```bash
.
└── dist
    ├── index-155441f587b62503.css  # Generation of the index.scss file
    ├── index.html  # Generation of the index.html file
    ├── trunk-template-6ffbdd40bd88aaf2.js  # Some code to load the .wasm app
    └── trunk-template-6ffbdd40bd88aaf2_bg.wasm # WASM generation of your Rust React App.
```

## Contributing

If you want to contribute, There are a few things you have to look into before everything else.

### Entrypoint

The code in [src/main.rs](src/main.rs) is the entrypoint of the browser to this app. It will load the components [src/app.rs](src/app.rs) into the body of the [index.html](index.html).

It'll load the components module;

### Top-Level component

The top level component of the app is [src/app.rs](src/app.rs), it's loaded in the [src/main.rs](src/main.rs) entrypoint and will be used as the Rust "React App".

It'll use the [<Counter />](src/counter.rs) component inside.

### License

This repo is shipped with the MIT license.

[trunk]: https://github.com/thedodd/trunk

blabla