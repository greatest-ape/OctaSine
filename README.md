# OctaSine

VST2 frequency modulation synthesizer written in Rust. Audio examples are available at [soundcloud.com/octasine](https://soundcloud.com/octasine).

## About

* Four operators with independent parameters such as volume, panning,
  modulation index, feedback, three different frequency modifiers (ratio, free
  and fine) and ADSR volume envelope parameters. The operators can be
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

## Copyright and license

Copyright (C) 2019-2020 Joakim Frostgård

This program is free software: you can redistribute it and/or modify it under
the terms of the GNU General Public License, Version 3, as published by the
Free Software Foundation.

This program is distributed in the hope that it will be useful, but WITHOUT ANY
WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A
PARTICULAR PURPOSE.  See the GNU General Public License for more details.

You should have received a copy of the GNU General Public License along with
this program.  If not, see <https://www.gnu.org/licenses/>.

Contents of `contrib` directory are licensed under other terms. Please
refer to the contained directories and/or files for details.

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

## Notes

Depends on the following git repositories:

  * https://github.com/greatest-ape/simdeez (__octasine__ branch for avx support and very fast sleef sines)
  * https://github.com/greatest-ape/sleef-sys (__octasine__ branch for static linking and avx instruction generation)

## Trivia

* The name OctaSine comes from the four stereo sine-wave operators
