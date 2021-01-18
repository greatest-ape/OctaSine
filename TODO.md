# TODO

## Important

* Changing preset doesn't seem to work
* Scrolling in dropdowns
* Dropdown color
* BPM sync doesn't work
* Check for any audio issues
* Consider updating envelope and lfo values in process benchmark too. This
  would further improve usefulness of output hashing.
* Automations in live. Do I have to call HostCallback::automate?

* Modulation matrix: make adjusting operator volume possible
* Envelopes
  * Maybe draw lines indicating top and bottom of draggable range
  * Interaction with draggers
  * Possibly button for snapping viewport etc
  * Horizontal scrolling?
  * Fix ANOTHER envelope error:
    `ERROR] thread 'unnamed' panicked at 'Tessellate path: UnsupportedParamater': /[..]/github.com-1ecc6299db9ec823/iced_graphics-0.1.0/src/widget/canvas/frame.rs:100`
    * Probably also related to zero envelope section length
* Reset knobs to default with backspace

## Less important

* Parameterise lfo target picker, knobs etc over ParameterValue for value
  conversions
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
