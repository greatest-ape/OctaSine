# TODO

## GUI

* Fix ANOTHER envelope error:
  `ERROR] thread 'unnamed' panicked at 'Tessellate path: UnsupportedParamater': /[..]/github.com-1ecc6299db9ec823/iced_graphics-0.1.0/src/widget/canvas/frame.rs:100`
* Reset knobs to default with backspace
* Move frequency knobs to left of modulation stuff?
* Mod matrix
  * Show modulation index with strong color in operator boxes?
  * Show feedback?
* Wave type
  * Fix layout: choices should probably be centered vertically so they align
    with knobs
  * Wave type could possibly be a tiny button under the operator number if
    there is a lack of space
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
* improve CI with macOS too, baseview linux deps

## Code quality / safety

* NUM_PARAMETERS constant?
* rustfmt

## Other

* manual text input in parameters: DAW integration working anywhere?
* sample rate change: what needs to be done? (time reset?)
* Nice online documentation
* Consider logging when preset can't be loaded (see `load_bank_data`)

## Maybe do

* Volume shown in dB
* Iterator for presets and preset parameters
* proper beta scaling - double with doubling modulator frequency
* suspend mode and so on, maybe just reset time, note time, envelopes etc on resume
* Use FMA again for precision, possibly enabling removing .fract() call
  in fallback sound gen?
* Fuzz Log10Table (cargo-fuzz?)
