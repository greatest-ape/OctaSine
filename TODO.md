# TODO

## Important

* Master frequency text becomes two rows when value is high, so mod
  matrix skips down
* Scrolling in dropdowns
  * Needs to be added to baseview macOS code
* Consider updating envelope and lfo values in process benchmark too. This
  would further improve usefulness of output hashing.
* Automations in Live. Do I have to call HostCallback::automate?
* Envelopes
  * Maybe draw lines indicating top and bottom of draggable range
  * Test dragger interaction
  * Possibly button for fitting viewport
  * Zoom in on load
  * Make buttons look nicer
    * Consider moving up ALL button 5 pixels to compensate for padding and
      border

## Less important

* Modulation matrix: make adjusting operator volume possible
* Reset knobs to default with backspace or maybe right click
  * Need to check if this is already supported in iced_audio
* Parameterise lfo target picker over ParameterValue?
* Nicer knob marks
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
