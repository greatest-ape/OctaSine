# OctaSine

Frequency modulation based VST2 plugin written in Rust

## About

* Four operators with independent parameters such as volume, panning,
  modulation index, feedback, three different frequency modifiers (ratio, free
  and fine) and ASDR volume envelope parameters. The operators can be
  independently switched to white noise mode
* Flexible routing allowing setting the output operator (with some
  limitations) as well as the percentage of signal that is simply added to the
  final output, enabling additive synthesis. By default, operator 4 is routed
  to operator 3, operator 3 to operator 2 and operator 2 to operator 1.
* 128 voices (using them all simultaneously might consume quite a bit
  of CPU time though)
* Fully automatable (nice way of saying there is currently no built-in
  graphical user interface)
* Master volume and master frequency parameters

## Audio examples

https://soundcloud.com/octasine

## Installation

### macOS

If you have already any of the software mentioned below, that step can be skipped.

[Install the rust compiler](https://rustup.rs/). Requires the XCode build tools from Apple, you will probably be prompted to install those.

Install nightly Rust toolchain:

```sh
rustup toolchain install nightly
```

[Install homebrew](https://brew.sh).

Install git and cmake with homebrew:

```sh
brew install git cmake
```

Clone this repository to a folder on your computer:

```sh
mkdir -p "$HOME/Downloads"
cd "$HOME/Downloads"
git clone https://github.com/greatest-ape/OctaSine.git
cd OctaSine
```

Build and install:

```sh
./scripts/macos/build-simd-and-install.sh
```

__Advanced:__ If you don't want SIMD support and/or prefer the stable toolchain, instead run:

```sh
./scripts/macos/build-and-install.sh
```

Binary (pre-built) releases might be uploaded eventually.

### Other platforms

Have a look at the cargo invocations from the macOS section scripts, they
should work fine.

## License

Different parts of this project are licensed under different terms:

  * The `octasine` and `octasine_vst` crates are licensed under the
    __GNU General Public License, Version 3__.
  * The `vst2_helpers` crates as well as the contents of the `scripts`
    directory are licensed under either of __MIT license__ or
    __Apache License, Version 2.0__, at your option.
  * The file located at `contrib/osx_vst_bundler.sh` is licensed under the
    __MIT license__. See the file for specifics.

## Notes

Depends on the following git repositories:

  * https://github.com/greatest-ape/simdeez (__octasine__ branch for avx support and very fast sleef sines)
  * https://github.com/greatest-ape/sleef-sys (__octasine__ branch for static linking and avx instruction generation)
  * https://github.com/greatest-ape/iced_baseview (__octasine__ branch)

Nightly toolchains known to work:

  * rustc 1.43.0-nightly (436494b8f 2020-02-22)

## Trivia

* The name OctaSine comes from the four stereo sine-wave operators
