<p align="center">
  <img src="https://raw.githubusercontent.com/helloedit/rust-cmark-gfm/master/media/markdown.png" />
</p>

<h2 align="center">libcmark-gfm raw bindings</h2>

---

Rust wrapper for `libcmark-gfm`, GitHub's fork of the reference parser for CommonMark.
For more information on available extensions, see the [documentation](https://github.github.com/gfm/).

### Requirements

As this crates build itself `libcmark-gfm` from source, **you don't need to have `libcmark-gfm` already installed**. You need `cmake` and you will need to meet [bindgen requirements](https://rust-lang.github.io/rust-bindgen/requirements.html).

### License

MIT