# TODO

## Important

* Pass through keyboard to DAW
* Master frequency text becomes two rows when value is high, so mod
  matrix skips down
* Scrolling in dropdowns
  * Needs to be added to baseview macOS code
* Consider updating envelope and lfo values in process benchmark too. This
  would further improve usefulness of output hashing.
* Automations in Live. Do I have to call HostCallback::automate to make
  parameters show up when clicking on them?
* Envelopes
  * Maybe draw lines indicating top and bottom of draggable range
  * Possibly button for fitting viewport
  * Zoom in on load

## Less important

* Wider master volume spread?
* GUI
  * Modulation matrix: make adjusting operator volume possible
  * Operator audio output indicator, either binary or volume
  * Master audio output indicator
  * Zoom in towards left if offset_x is zero?
  * Master volume knob to the right of master frequency?
  * Reset knobs to default with backspace or maybe right click
    * Need to check if this is already supported in iced_audio
  * Parameterise lfo target picker over ParameterValue?
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
