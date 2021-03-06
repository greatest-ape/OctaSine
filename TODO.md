# TODO

## Important

* Build for Apple silicon
  * ADVSIMD (NEON) acceleration should be supported, at least by enabling the
    target feature. I'm not sure about how that is done when cross-compiling.

* GUI
  * Provide glow backend as alternative or maybe even default build
    * Crash when reopening GUI. Faulty superview call?
    * Crash when closing GUI?
    * Fix anti-aliasing? Maybe broken on Windows?
  * Parameter editing: bracket changes with begin_edit and end_edit
    * iced_audio knobs would need events for starting and ending dragging
  * Scrolling in dropdowns
    * iced 0.4: https://github.com/hecrj/iced/pull/872
    * Does scrolling (including touch bar) need to be added to baseview
      macOS code? What about other platforms?

## Less important

* Manual under info button?
  * Presets are exported/imported through DAW
* Process benchmark output not same on Windows as on macOS/Linux
* Debian 10 / other Linux distros CI
* Record video of workflow, upload to YouTube
* Pass through keyboard to DAW without using forked baseview
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
* Parameterise lfo target picker over ParameterValue?
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
