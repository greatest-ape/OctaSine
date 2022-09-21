<h1 align="center">OctaSine</h1>

<p align="center">
Frequency modulation synthesizer plugin. Runs on macOS, Windows and Linux (X11) in VST2-compatible hosts.
</p>

<p align="center">
  <strong>Official website with downloads</strong><br>
  <a href="https://www.octasine.com">OctaSine.com</a>
</p>

<p align="center">
  <strong>Audio examples</strong><br>
  <a href="https://soundcloud.com/octasine">SoundCloud</a>
</p>

## Screenshots

### Light mode

![Screenshot of OctaSine in light mode](images/screenshot-light.png)

### Dark mode

![Screenshot of OctaSine in dark mode](images/screenshot-dark.png)

## About

* Four FM operators with parameters for volume, panning, modulation output, feedback, frequency modifiers (ratio, free and fine), envelope values (attack, decay, sustain, release) and toggling of white noise mode.
* Flexible routing allows setting the operator modulation targets (with some limitations) as well as the amount of signal that is simply added to the final output, enabling additive synthesis.
* Four LFOs with multiple waveforms, oneshot and loop modes and optional DAW BPM sync. They can target most operator parameters and most parameters of lower-index LFOs.
* Each operator is connected to an attack-decay-sustain-release volume envelope with logarithmic slopes.
* Per-operator white noise mode makes it easy to create percussive sounds such as hi-hats.
* Runs on macOS (definitely 10.15.7, probably later versions too), Windows 10 and Linux (X11 only) in VST2-compatible DAWs on 64-bit computers. Synthesis is SIMD-accelerated in many cases (SSE2, AVX).
* Master volume and master frequency parameters
* 128 voices (using them all simultaneously might consume quite a bit of CPU time though)
* Fully automatable

## Installation from source code

Please note that the recommended way to install OctaSine on macOS and Windows
is to [download an official release](https://www.octasine.com), not to build it from source.

### macOS

* [Install the Rust compiler](https://rustup.rs/). Choose the stable toolchain
  when prompted. The compiler requires Apple's XCode build tools. You will
  probably be prompted to install them.
* Install git and cmake. If you're using [homebrew](https://brew.sh), run:

```sh
brew install git cmake
```

* Clone this repository to a folder on your computer:

```sh
mkdir -p "$HOME/Downloads"
cd "$HOME/Downloads"
git clone https://github.com/greatest-ape/OctaSine.git
cd OctaSine
```

* Unless you want to use the bleeding edge development branch, switch to the latest stable version, e.g.:

```sh
git checkout tags/v0.8.0
```

* Build and install:

```sh
./scripts/macos/build-simd-and-install.sh
```

### Windows

* Install [Microsoft C++ Build Tools](https://visualstudio.microsoft.com/visual-cpp-build-tools/). Make sure that the Windows 10 SDK and the English language pack components are included during installation.
* [Install the Rust compiler](https://rustup.rs/). When prompted, choose the stable toolchain and to modify the path variable.
* Install LLVM
* Install [cmake](https://cmake.org/download/). When prompted, choose the option to add cmake to the system executable path.
* Install [git](https://git-scm.com/downloads).
* Clone this repository to a folder on your computer and enter it
* Unless you want to use the bleeding edge development branch, switch to the latest stable version, e.g.:

```sh
git checkout tags/v0.8.0
```

* Build OctaSine:

```cmd
cargo build --release -p octasine-vst2-plugin
```

* Copy `target\release\octasine.dll` to your VST plugin folder.

### Linux

* [Install the Rust compiler](https://rustup.rs/). Choose the stable toolchain when prompted. 
* Install dependencies, e.g.,

```sh
sudo apt-get install cmake git build-essential llvm clang libx11-dev libxcursor-dev libxcb-dri2-0-dev libxcb-icccm4-dev libx11-xcb-dev
```

On Debian 10, you might need to install some more dependencies:

```sh
sudo apt-get install pkg-config libfreetype6-dev libexpat1-dev
```

* Clone this repository to a folder on your computer, e.g.,

```sh
mkdir -p "$HOME/Downloads"
cd "$HOME/Downloads"
git clone https://github.com/greatest-ape/OctaSine.git
cd OctaSine
```

* Unless you want to use the bleeding edge development branch, switch to the latest stable version, e.g.:

```sh
git checkout tags/v0.8.0
```

* Build the OctaSine plugin:

```sh
cargo build --release -p octasine-vst2-plugin
```

* Copy `target/release/liboctasine.so` to your VST plugin folder 

## Architecture

See [ARCHITECTURE.md](ARCHITECTURE.md).

## Copyright and license

Copyright (C) 2019-2022 Joakim Frostegård

OctaSine is distributed under the GNU Affero General Public License, Version 3,
as published by the Free Software Foundation. See [LICENSE](LICENSE) for
details.

Contents of the `contrib` directory are licensed under other terms. Please
refer to the contained directories and/or files for details.
