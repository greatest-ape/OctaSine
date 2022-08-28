# Changelog

All notable changes to this project will be documented in this file.

## Unreleased

### Added

- Display (part of) left and right channel waveforms for all operators
- Add patch section dropdown with actions for opening patch/bank files, saving
  patches/banks, renaming patches and clearing current patch/bank
- Add functionality for setting parameter values exactly. Click on values to
  display a value input window.
- Reintroduce master volume and master frequency parameters as LFO targets
  (removed in version 0.7)

### Changed

- Rework panning
  - Use true stereo panning (not just balance) for mix and modulation output
  - Stop using constant power panning for modulation output. Previously, when
    an operator was panned to the center, its modulation output would scale at
    `cos(π/4)` (approximately 0.7071) the rate of feedback but at the same
    rate when panned hard to a side, which is not very intuitive. 
- Decrease minimum envelope stage duration from 10ms to 3.33ms
- When note off message is received during envelope attack phase, always go to
  release phase, even if it means that with low sustain volumes and a long
  release time, short notes might be loud for a lot longer than longer
  notes. This is how FM8 envelopes work.

### Fixed

- When note off message is received during envelope decay phase, go to release
  phase
- For LFOs in oneshot mode, after running once, stay at the end value instead
  of going back to zero

## 0.7.0 - 2022-06-08

This is a large release featuring lots of changes. Some major ones include:
- Full interface redesign for a more modern look
- Operators can now modulate multiple targets. Routing parameters were updated
  to be more intuitive
- LFOs were redesigned to be easier to work with
- Envelope groups were implemented for adjusting multiple envelopes
  simultaneously
- Operator and LFO mute buttons were added

This release contains breaking changes. It is __not__ compatible with patches
created with previous versions. However, you can keep using version 0.6.1
alongside this release, since it uses a different VST2 plugin ID.

### Changed

#### Operator changes

- Allow operators to modulate multiple carriers
- Set modulation index (mod out) in modulator, not carrier
- Increase range of feedback parameter to match modulation index
- Replace additive factor with mix out parameter, which doesn't
  affect modulation output
- Update frequency ratios, adding both harmonic and disharmonic ones
- Update free frequency parameter values
- Add operator mute parameter
- Feedback is now affected by key velocity
- Interpolate key velocity if key is pressed while envelope is still active
- Fix various envelope issues
- Remove ability to set end volume of attack (it is now always at maximum)
- Increase minimum envelope stage length to 10ms
 
#### LFO changes

- Use a more traditional LFO design that oscillates around the base value, not
  in a single direction.
- Scale LFO effect linearly and bypass normal parameter limits when targeting
  master frequency, master volume, operator free frequency, operator volume,
  LFO amount (magnitude) or LFO free frequency parameters
- Make LFO triangle wave type start at value 0.0
- Add LFO wave types reverse triangle, sine and reverse sine
- Default to no target
- Remove master volume and master frequency targets, since LFOs are really
  per-voice
- Add LFO mute parameter
- When BPM sync is turned off, use base frequency of 1 Hz

#### Other audio changes

- Update master frequency parameter values
- Use time-based instead of sample-based interpolation for parameters and LFOs
  for sample rate independence

#### GUI changes

- Major GUI redesign with layout and color changes
- Add envelope group functionality, enabling simultaneously editing multiple ones
- Zoom in envelopes by dragging up/down
- Control mix output instead of modulation output with modulation matrix
  operator boxes
- Replace operator wave type picker with a custom widget that displays the waveform
- Replace the LFO shape knob with a custom widget that displays the waveform
- Add per-operator modulation target picker
- Add per-operator mute button
- Add per-LFO mute button

#### Other changes

- Add octasine-cli crate with subcommands to convert between exported patches /
  patch banks and JSON
- Bump plugin unique ID to allow using this version in parallel with previous
  versions
- Use directories crate to determine where to save preference and log files
- Use gzip compression on exported patches and patch banks
- Update multiple dependencies, notably iced and baseview
- Do major code refactor

### Fixed

- Fetch BPM once per process call instead of each sample

## 0.6.1 - 2022-04-28

### Changed

- Include semver-compatible version information in plugin name (e.g.,
  "OctaSine v0.6") to ease using multiple releases alongside each other.

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
