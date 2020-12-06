# TODO

## GUI

* Fix ANOTHER `ERROR] thread 'unnamed' panicked at 'Tessellate path: UnsupportedParamater': /[..]/github.com-1ecc6299db9ec823/iced_graphics-0.1.0/src/widget/canvas/frame.rs:100`
* Envelopes
  * Maybe draw lines indicating top and bottom of draggable range
  * Interaction with draggers
* Use shift modifier
* Maybe don't load parameter changes in processing while dragging.

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
