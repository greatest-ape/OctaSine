# TODO

## Important

* Pass through keyboard to DAW
  * Needs to be done in baseview and iced_baseview

## Less important

* calculate_curve_volume_output_in_range fails on (0.0, -0.0, -0.0, -0.0),
  so likely negative zero
* Scrolling in dropdowns
  * Needs to be added to baseview macOS code
* Consider updating envelope and lfo values in process benchmark too. This
  would further improve usefulness of output hashing.
* GUI
  * Improve macOS Retina support
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
* rustfmt

## Don't do

* Cache sync value in interpolatable parameters too? Don't do this, it seems
  to hurt performance.
* proper beta scaling - double with doubling modulator frequency: too late now
