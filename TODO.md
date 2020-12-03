# TODO

## GUI

* Make sure that rendering when window is not in focus actually works!
* Envelopes
  * Curves that actually match real shape
  * Time markers: adjust frequency after total duration
  * Draw draggable circles at junctions
* Shift modifer: implement in iced_baseview if necessary
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
