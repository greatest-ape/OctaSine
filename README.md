# OctaSine

Frequency modulation based VST2 plugin

## About

* Four operators with independent parameters such as volume, panning, modulation index, feedback, three different frequency modifiers (ratio, free and fine) and ASDR volume envelope parameters. The operators can be independently switched to white noise mode
* Flexible routing allowing setting the output operator (with some limitations) as well as the percentage of signal that is simply added to the final output, enabling additive synthesis
* 128 voices (using them all simultaneously might consume quite a bit of CPU time though)
* Fully automatable (nice way of saying there is currently no built-in graphical user interface)
* The name OctaSine comes from the four stereo sine-wave operators

## Installation

After cloning the repository and installing the rust compiler (including a nightly toolchain), build and install by running:

```sh
./scripts/build-simd-and-install.sh
```

If you don't want SIMD support and/or prefer the stable toolchain, instead run:

```sh
./scripts/build-and-install.sh
```

Binary (pre-built) releases might be uploaded eventually.

## License

OctaSine is licensed under the GNU GPL 3.0. The support crate simd_sleef_sin35 is
licensed under the Apache 2.0 license.