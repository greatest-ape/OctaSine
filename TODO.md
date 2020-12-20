# TODO

## GUI

* Fix ANOTHER envelope error:
  `ERROR] thread 'unnamed' panicked at 'Tessellate path: UnsupportedParamater': /[..]/github.com-1ecc6299db9ec823/iced_graphics-0.1.0/src/widget/canvas/frame.rs:100`
* Full-width horizontal rules
* Reset knobs to default with backspace
* Mod matrix
  * Show modulation index with strong color in operator boxes?
  * Show feedback?
* Cache operator string value?
* Envelopes
  * Maybe draw lines indicating top and bottom of draggable range
  * Interaction with draggers
  * Possibly button for snapping viewport etc
  * Horizontal scrolling?
* update_host_display stuff
  * Running update_host_display all the time tanks performance. Maybe only run
    it on knob release, or stillstand (very little movement since last event.)
  * Maybe don't load parameter changes in processing while dragging: not a
    problem if the previous point is implemented and probably not a problem
    now either

## Code quality / safety

* rustfmt

## Other

* Preset parameter from text
  * Implement simple parsing etc for all
  * DAW integration working anywhere?
* sample rate change: what needs to be done? (time reset?)
* Nice online documentation

## Maybe do

* Test that number of sync and processing parameters is equal
* proper beta scaling - double with doubling modulator frequency
* suspend mode and so on, maybe just reset time, note time, envelopes etc on resume
* Use FMA again for precision, possibly enabling removing .fract() call
  in fallback sound gen?
* Fuzz Log10Table (cargo-fuzz?)
