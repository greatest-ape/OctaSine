# Changelog

All notable changes to this project will be documented in this file.

## Unreleased

This release contains breaking changes, i.e., changes that can affect
how patches sound.

### Changed

- Use a more traditional LFO design that oscillates around the base value, not
  in a single direction.
- Add LFO wave types reverse triangle, sine and reverse sine

### Fixed

- Fetch BPM once per process call instead of each sample

## 0.6.0 - 2022-01-08

This release contains breaking changes, i.e., changes that can affect
how patches sound.

### Changed

- When triggering note while envelope is still running, restart envelope from
  zero volume
- To prevent artefacts, for very short envelope stages, the normal logarithmic
  slopes will be mixed with linear slopes. This is now done for slightly longer
  envelope stages
- Adjust LFO wave shapes for better beat fit
- Interpolate parameters for 32 samples

### Fixed

- Properly handle audio buffers of arbitrary size
- Properly handle midi event timings
- Fix LFO retrigger issues
- Properly treat MIDI note on/off event with 0 velocity as note off

## 0.5.4 - 2021-12-20

### Added
- Pass back key presses to DAW for virtual keyboard support

### Changed

- Default to glow (OpenGL) backend
- Use much more recent version of graphics dependency iced, from its git repository

### Fixed
- Fix bug where closing plugin window on macOS could cause a crash
- Improve support for screens with high DPI on macOS and Windows

## 0.5.3 - 2021-06-18

### Changed

- Redesign dark and light modes
- Decrease size of plugin window to accommodate laptop screens

### Fixed

- Fix multiple HiDPI issues by updating baseview dependency

## 0.5.2 - 2021-06-05

### Added

- Add GUI dark mode
- Print OctaSine version and OS info to log file

### Changed

- Simplify preset/parameter handling code
- Update dependencies

### Fixed

- Fix bug where audio engine doesn't pick up preset changes

## 0.5.1 - 2021-05-29

### Added

- Enable SIMD audio generation on Windows

### Changed

- Update dependencies

## 0.5.0 - 2021-02-14

First release
