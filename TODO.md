# TODO

## Important

* Wait for merge of raw-gl-context fix, use that repo
* Consider defaulting to wgpu on Linux

* Interpolation in processing parameters: should it be based on time rather
  than number of samples?

* Log level should maybe be warn
* Log file should be saved elsewhere, maybe in ::dirs::data_local_dir()
* Consider adding phase knobs
* Consider adding saw, square and triangle waves. Maybe look at
  TX81Z waveforms. https://www.reddit.com/r/synthesizers/comments/rkyk6j/comment/hpgcu6r/?utm_source=share&utm_medium=web2x&context=3

* Bugs
  * LFO issue: when time between key presses is short, lfo seems to still be
    running and affecting next key press. Valid for all shapes and both modes.
    Interpolation issue?
  * Mouse drag movements in pick list transfer through to envelope editor

* Build for Apple silicon
  * ADVSIMD (NEON) acceleration should be supported, at least by enabling the
    target feature. I'm not sure about how that is done when cross-compiling.

* GUI
  * Parameter editing: bracket changes with begin_edit and end_edit
    * iced_audio knobs would need events for starting and ending dragging
  * Scrolling in dropdowns
    * iced 0.4: https://github.com/hecrj/iced/pull/872
    * Does scrolling (including touch) need to be added to baseview
      macOS code? What about other platforms?

* Documentation
  * LFO shapes, shape/amount interaction

## Less important

* Manual under info button?
  * Presets are exported/imported through DAW
* Process benchmark output not same on Windows as on macOS/Linux
* Record video of workflow, upload to YouTube
* Consider updating envelope and lfo values in process benchmark too. This
  would further improve usefulness of output hashing.
* GUI
  * Modulation matrix: improve creation/update logic?
  * Operator audio output indicator, either binary or volume
  * Master audio output indicator
  * Zoom towards center of envelope duration instead of viewport if
    envelope doesn't cover viewport? (Or maybe always)
  * Master volume knob to the right of master frequency?
  * Reset knobs to default with backspace or maybe right click
    * Need to check if this is already supported in iced_audio
  * Nicer knob marks
    * Operator 2-4 middle marker
  * Do I need to run update_host_display?
    * Should it be run on knob drag release?
* sample rate change: what needs to be done? (time reset?)
* clippy

## Not important

* Test that number of sync and processing parameters is equal
* suspend mode and so on, maybe just reset time, note time, envelopes etc on resume
* Fuzz Log10Table (cargo-fuzz?)
* Is it necessary to look at time signatures etc for bpm sync?
  https://rustaudio.github.io/vst-rs/vst/api/struct.TimeInfo.html
* Preset parameter from text
  * Implement simple parsing etc for all
  * DAW integration working anywhere?
* Nice online documentation

## Don't do

* Cache sync value in interpolatable parameters too? Don't do this, it seems
  to hurt performance.
* proper beta scaling - double with doubling modulator frequency: too late now
