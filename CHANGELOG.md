# Changelog

## Next release

This release contains breaking changes, i.e., changes that can affect
how patches sound.

- Properly handle audio buffers of arbitrary size
- Properly handle midi event timings
- When triggering note while envelope is still running, restart envelope from
  zero volume
- Tend towards linear envelopes rather than logarithmic for slightly longer
  envelope stage durations that previously
- Adjust LFO wave shapes for better beat fit
- Fix LFO retrigger issues

## v0.5.4

### Other

- Default to glow (OpenGL) backend
- Use much more recent version of iced, from its git repository

## v0.5.3

### Features

- Redesign dark and light modes
- Decrease size of plugin window to accommodate laptop screens
- Fix multiple HiDPI issues by updating baseview dependency

## v0.5.2

### Features

- Add GUI dark mode

### Bug fixes

- Fix bug where audio engine doesn't pick up preset changes

### Other

- Print OctaSine version and OS info to log file
- Simplify preset/parameter handling code
- Update dependencies

## v0.5.1

### Features

- Enable SIMD audio generation on Windows

### Other

- Update dependencies

## v0.5.0

First release
