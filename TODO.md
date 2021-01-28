# TODO

## Important

* Pass through keyboard to DAW
* Scrolling in dropdowns
  * Needs to be added to baseview macOS code

## Less important

* Wider master volume spread?
* Consider updating envelope and lfo values in process benchmark too. This
  would further improve usefulness of output hashing.
* GUI
  * Modulation matrix: make adjusting operator volume possible
    * Use red color when volume exceeds 1.0?
    * Improve general structure and component updates
  * Operator audio output indicator, either binary or volume
  * Master audio output indicator
  * Zoom towards center of envelope duration instead of viewport if
    envelope doesn't cover viewport? (Or maybe always)
  * Master volume knob to the right of master frequency?
  * Reset knobs to default with backspace or maybe right click
    * Need to check if this is already supported in iced_audio
  * Nicer knob marks
    * Operator 2-4 middle marker
  * update_host_display stuff
    * Running update_host_display all the time tanks performance. Maybe only run
      it on knob release, or stillstand (very little movement since last event.)
    * Maybe don't load parameter changes in processing while dragging: not a
      problem if the previous point is implemented and probably not a problem
      now either
* sample rate change: what needs to be done? (time reset?)
* clippy

## Not important

* Test that number of sync and processing parameters is equal
* Parameterise lfo target picker over ParameterValue?
* proper beta scaling - double with doubling modulator frequency
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
