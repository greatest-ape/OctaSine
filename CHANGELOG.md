# Changelog

All notable changes to this project will be documented in this file.

## Unreleased

### Fixed

- Fix GUI issues when using the clap plugin in Bitwig
- Fix issue with activating operator 2 modulation target from GUI modulation
  matrix
- Show file save/load dialogs on top of plugin window on macOS
- When saving patch bank/state, also save selected patch index (and restore it
  on load)

## 0.9.0 - 2023-08-03

This release contains breaking changes. Voice phases are reset when they end,
which can audibly impact existing projects.

### Added

- Add monophonic voice mode (only once voice active at a time). In this mode,
  when envelopes are restarted, attack will proceed from the previous volume,
  not from zero
- Add glide (portamento)
  - Available for both polyphonic and monophonic voice modes
  - Can be set to OFF, LEG (only glide when playing legato) or ON (always
    glide)
  - Supports linear constant time (LCT) and linear constant rate (LCR, time
    per octave) modes
  - Supports optional BPM sync for time/rate
  - Optionally retrigger envelopes and LFOs when gliding in monophonic mode

### Changed

- Replace patch settings overlay with button for toggling alternative visible
  controls

### Fixed

- Reset voice phases when they end. This fixes an issue where subsequent key
  presses without intermediate parameter changes do not result in identical
  sounds.

## 0.8.7 - 2023-05-16

Audio output is not bit-for-bit identical to version 0.8.6, but there should be
no audible differences.

### Added

- Add velocity sensitivity parameters, i.e., parameters for how key press
  velocity affects:
  - Final voice volume
  - Modulation output (per-operator)
  - Feedback (per-operator)

### Changed

- Add parameter tooltips for better discoverability
- Change layout of bottom right corner and make plugin window 12 pixels wider
  to accommodate new controls.

## 0.8.6 - 2023-04-12

### Added

- Add LFO key sync parameter, controlling initial phase of LFOs when activated
  by a key press. With the default of value of "on", the phase starts at zero.
  When value is set to "off", the LFO will instead start at a random point of
  the wave cycle.
- Add pitch bend support. Parameters for upwards and downwards semitone range
  are accessible from the patch action dropdown menu.

## 0.8.5 - 2023-04-05

### Added

- Add square, triangle and saw operator waveforms. Patches created with
  previous versions should load from files perfectly. DAW projects should
  load perfectly in most cases, but if you've used automations on the
  waveform parameter, the effect will not be the same as before.

### Fixed

- Fix crashes on Windows when clearing patch bank or patches
- Display file pickers on top of plugin window on macOS

## 0.8.4 - 2023-04-03

### Added

- Implement [clap](https://cleveraudio.org/) state extension

### Changed

- Use new patch bank / patch format with better provisions for forward
  compatibility. Patches created with previous versions are automatically
  converted when opened.
- Upgrade to [iced](https://github.com/iced-rs/iced) v0.8

### Fixed

- Make CLAP plugin GUI work in Bitwig on Windows; it still doesn't work in
  Bitwig on macOS and only sometimes on Linux
- Fix clap miscompilation on Linux on aarch64

## 0.8.3 - 2023-02-14

### Added

- Add initial [clap](https://cleveraudio.org/) plugin support

### Changed

- Upgrade to [iced](https://github.com/iced-rs/iced) v0.7

### Fixed

- Fix crashes when opening GUI

## 0.8.2 - 2022-12-22

### Added

- Support sustain pedals (MIDI CC 64)

### Changed

- Upgrade to [iced](https://github.com/iced-rs/iced) v0.6, including partly
  rewriting [iced_baseview](https://github.com/BillyDM/iced_baseview).

### Fixed

- Improve detection of when mouse cursor hovers over envelope draggers

## 0.8.1 - 2022-10-29

This release features native Apple Silicon support, performance improvements
and bug fixes, including making parameters automatable in Bitwig Studio and
Carla.

Audio output is not bit-for-bit identical to version 0.8.0, but there should be
no audible differences.

### Added

- Add Apple Silicon support. The macOS release is now built as a universal binary

### Changed

- Improve performance by around 10% in many cases by improving CPU cache
  behaviour in audio generation
- [Port several SLEEF functions to Rust](https://github.com/greatest-ape/sleef-trig)
  to avoid relying on undefined behaviour, remove the need to use a nightly
  compiler and ease cross-compilation, e.g., for Apple Silicon. Unfortunately,
  performance of these functions is decreased somewhat.
- In audio generation, skip extracting voice data if envelope is ended
- Update dependencies

### Fixed

- Fix bug where plugin didn't properly tell host about automatable parameters,
  causing them not to be picked up by Bitwig, Carla and possibly other hosts
- Tweak audio gen to fix (very minor) differences between simd widths
- When envelope ends, set voice operator phase to 0.0

## 0.8.0 - 2022-08-28

This release features revamped stereo panning, shorter minimum envelope stage
lengths, improved patch management, displaying of waveforms and various fixes.

This release contains breaking changes. Patches might not be able to be fully
migrated. However, in many cases, importing patches from 0.7.0 and doing two
changes will work:
  - Setting `mod out` values to 0.7071 of what they previously were
  - Setting LFO targets to the correct value (they may have been changed)

You can keep version 0.7.0 alongside this release, since they use different
VST2 plugin IDs.

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
